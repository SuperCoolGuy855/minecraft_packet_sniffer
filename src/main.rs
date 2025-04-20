mod parser;
mod protocol;
mod sniffer;
mod tcp_connection;

use crate::parser::ether_ip::IPPacketInfo;
use crate::parser::PacketParser;
use crate::sniffer::sniffer;
use color_eyre::eyre::{bail, OptionExt};
use color_eyre::Result;
use flume::{Receiver, Sender};
use log::{debug, error, info, LevelFilter};
use pcap::{Capture, Device, Error};
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::thread;

pub fn get_client_ip_from_packet(packet: &IPPacketInfo, server_ip: IpAddr) -> (IpAddr, bool) {
    if packet.src_ip == server_ip {
        (packet.dst_ip, false)
    } else {
        (packet.src_ip, true)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();

    let server_str = "192.168.1.2";
    let server_ip = IpAddr::from_str(server_str)?;

    let device_list = Device::list()?;
    println!("{:#?}", device_list);

    let device = device_list
        .into_iter()
        .find(|x| x.name == *r"\Device\NPF_{7BA758E3-7038-4689-95D4-975A621288E4}")
        .ok_or_eyre("Device not found")?;

    let mut cap = Capture::from_device(device)
        .unwrap()
        .promisc(true)
        .timeout(0)
        .immediate_mode(true)
        .open()?;
    cap.filter(&format!("tcp and net {server_str} and port 25565"), true)?;

    let mut listener_channels = HashMap::new();
    let (close_noti_tx, close_noti_rx) = flume::bounded(5);

    loop {
        let packet = match cap.next_packet() {
            Ok(p) => p,
            Err(Error::TimeoutExpired) => continue,
            Err(e) => bail!(e),
        };

        if let Ok(ip) = close_noti_rx.try_recv() {
            listener_channels.remove(&ip);
        }

        let ip_packet = IPPacketInfo::from_bytes(packet.data)?;
        let (client_ip, server_bounded) = get_client_ip_from_packet(&ip_packet, server_ip);

        // let tx = listener_channels.entry(client_ip).or_insert({
        //     debug!("Creating new channel for {client_ip}");
        //     let (tx, rx) = flume::bounded(20);
        //     thread::spawn(move || listener_thread_wrapper(rx, server_ip, client_ip));
        //     tx
        // });

        let tx = match listener_channels.get(&client_ip) {
            Some(tx) => tx,
            None => {
                debug!("Creating new channel for {client_ip}");
                let (tx, rx) = flume::bounded(20);
                let tx_clone = close_noti_tx.clone();
                thread::spawn(move || sniffer_wrapper(rx, server_ip, client_ip, tx_clone));
                listener_channels.insert(client_ip, tx);
                listener_channels.get(&client_ip).expect("Just added above")
            }
        };

        if tx.send((ip_packet, server_bounded)).is_err() {
            debug!("Channel errored: Removing {client_ip}");
            listener_channels.remove(&client_ip);
            continue;
        }

        listener_channels.retain(|k, v| {
            if v.is_disconnected() {
                debug!("Channel closed: Removing");
                false
            } else {
                true
            }
        });
    }

    Ok(())
}

fn sniffer_wrapper(
    rx: Receiver<(IPPacketInfo, bool)>,
    server_ip: IpAddr,
    client_ip: IpAddr,
    close_noti_tx: Sender<IpAddr>,
) {
    info!("New connection from {client_ip}");
    let res = sniffer(rx, server_ip, client_ip);
    if let Err(e) = res {
        error!("Sniffer error: {e}");
    };
    info!("Connection closed from {client_ip}");
    let _ = close_noti_tx.send(client_ip);
}
