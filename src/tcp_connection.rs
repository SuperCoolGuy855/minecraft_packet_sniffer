use crate::get_client_ip_from_packet;
use crate::parser::ether_ip::IPPacketInfo;
use crate::parser::tcp::{TCPFlag, TCPPacketInfo};
use crate::parser::PacketParser;
use flume::Receiver;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Data(Vec<u8>, bool),
    Closed,
}

#[derive(Debug, Clone, Default)]
pub struct TCPData {
    data: Vec<u8>,
    next_seq: u32,
    buffer: HashMap<u32, TCPPacketInfo>,
}

#[derive(Debug, Clone)]
pub struct TCPConnection {
    rx: Receiver<(IPPacketInfo, bool)>,
    c2s_data: TCPData,
    s2c_data: TCPData,
    closing_state: u8,
}

impl TCPConnection {
    pub fn new(rx: Receiver<(IPPacketInfo, bool)>) -> Self {
        Self {
            rx,
            c2s_data: Default::default(),
            s2c_data: Default::default(),
            closing_state: 0,
        }
    }

    pub fn next_packet(&mut self) -> color_eyre::Result<ConnectionState> {
        let rx = &self.rx;
        loop {
            let (ip_packet, server_bounded) = rx.recv()?;
            let tcp_packet = TCPPacketInfo::from_bytes(&ip_packet.payload)?;
            {
                let tcp_data = if server_bounded {
                    &mut self.c2s_data
                } else {
                    &mut self.s2c_data
                };

                if tcp_packet.flags.contains(TCPFlag::ACK) && self.closing_state > 0 {
                    self.closing_state += 1;
                    if self.closing_state == 4 {
                        return Ok(ConnectionState::Closed);
                    }
                    continue;
                } else if tcp_packet.flags.contains(TCPFlag::SYN) {
                    tcp_data.next_seq = tcp_packet.seq_num + 1;
                    continue;
                } else if tcp_packet.flags.contains(TCPFlag::FIN) {
                    self.closing_state += 1;
                    continue;
                } else if !tcp_packet.payload.is_empty() {
                    tcp_data.buffer.insert(tcp_packet.seq_num, tcp_packet);
                }

                loop {
                    match tcp_data.buffer.remove(&tcp_data.next_seq) {
                        None => break,
                        Some(tcp_packet) => {
                            let flags = tcp_packet.flags.clone();
                            tcp_data.next_seq += tcp_packet.payload.len() as u32;
                            tcp_data.data.extend(tcp_packet.payload);
                            if flags.contains(TCPFlag::PSH) {
                                let mut output = vec![];
                                output.append(&mut tcp_data.data);
                                return Ok(ConnectionState::Data(output, server_bounded));
                            }
                        }
                    }
                }
            }
        }
    }
}
