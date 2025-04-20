use std::any::Any;
use crate::parser::ether_ip::IPPacketInfo;
use crate::protocol::types::{Datatype, VarInt};
use crate::protocol::MinecraftListener;
use crate::tcp_connection::{ConnectionState, TCPConnection};
use flume::Receiver;
use itertools::Itertools;
use log::info;
use std::collections::VecDeque;
use std::net::IpAddr;
use crate::protocol::packets::Handshake;

pub fn sniffer(
    rx: Receiver<(IPPacketInfo, bool)>,
    server_ip: IpAddr,
    client_ip: IpAddr,
) -> color_eyre::Result<()> {
    let mut listener = MinecraftListener::new(server_ip, client_ip);
    let mut tcp_conn = TCPConnection::new(rx);
    let mut buffer = VecDeque::new();
    let mut expected_length = 0;

    while let ConnectionState::Data(payload, server_bounded) = tcp_conn.next_packet()? {
        buffer.extend(payload);

        if expected_length == 0 {
            let length = VarInt::from_bytes(&mut buffer)?;
            expected_length = length.value as usize;
        }

        if buffer.len() >= expected_length {
            let packet_data = buffer.drain(..expected_length).collect_vec();
            expected_length = 0;
            let minecraft_packet = listener.parse_packet(&packet_data, server_bounded)?;
            info!("{minecraft_packet}");
        }
    }
    Ok(())
}
