use crate::protocol::packets::*;
use crate::protocol::types::{Datatype, State, VarInt};
use color_eyre::eyre::bail;
use flate2::bufread::ZlibDecoder;
use itertools::Itertools;
use log::{debug, info};
use std::collections::VecDeque;
use std::io::Read;
use std::net::IpAddr;

pub struct MinecraftListener {
    state: State,
    compression: bool,
    server_ip: IpAddr,
    client_ip: IpAddr,
}

impl MinecraftListener {
    pub fn new(server_ip: IpAddr, client_ip: IpAddr) -> Self {
        Self {
            state: State::Handshaking,
            compression: false,
            server_ip,
            client_ip,
        }
    }

    pub fn parse_packet(
        &mut self,
        bytes: &[u8],
        server_bounded: bool,
    ) -> color_eyre::Result<Box<dyn MinecraftPacket>> {
        let mut bytes = VecDeque::from(bytes.to_vec());
        debug!("Data: {bytes:x?}");
        
        let (packet_id, mut bytes) = {
            let mut bytes = if !self.compression {
                bytes
            } else {
                let data_length = VarInt::from_bytes(&mut bytes)?;
                if data_length.value == 0 {
                    bytes
                } else {
                    let bytes_vec: Vec<_> = bytes.into_iter().collect();
                    let mut decoder = ZlibDecoder::new(&bytes_vec[..]);
                    let mut output = Vec::new();
                    decoder.read_to_end(&mut output)?;
                    let temp = VecDeque::from(output);
                    assert_eq!(temp.len(), data_length.value as usize);
                    temp
                }
            };
            let packet_id = VarInt::from_bytes(&mut bytes)?.value;
            (packet_id, bytes)
        };

        if server_bounded {
            // Server bounded
            match self.state {
                State::Handshaking => match packet_id {
                    0 => {
                        let packet = Handshake::from_bytes(&mut bytes)?;
                        info!("Switching state: {}", packet.next_state);
                        self.state = packet.next_state;
                        Ok(Box::new(packet))
                    }
                    _ => Ok(Box::new({
                        let mut unknown = Unknown::from_bytes(&mut bytes)?;
                        unknown.packet_id = packet_id;
                        unknown.server_bounded = server_bounded;
                        unknown
                    })),
                },
                State::Status => match packet_id {
                    0 => Ok(Box::new(StatusRequest::from_bytes(&mut bytes)?)),
                    1 => Ok(Box::new(PingRequest::from_bytes(&mut bytes)?)),
                    _ => Ok(Box::new({
                        let mut unknown = Unknown::from_bytes(&mut bytes)?;
                        unknown.packet_id = packet_id;
                        unknown.server_bounded = server_bounded;
                        unknown
                    })),
                },
                State::Login => match packet_id {
                    0 => Ok(Box::new(LoginStart::from_bytes(&mut bytes)?)),
                    3 => {
                        self.state = State::Configuration;
                        Ok(Box::new(LoginAcknowledged::from_bytes(&mut bytes)?))
                    }
                    _ => Ok(Box::new({
                        let mut unknown = Unknown::from_bytes(&mut bytes)?;
                        unknown.packet_id = packet_id;
                        unknown.server_bounded = server_bounded;
                        unknown
                    })),
                },
                State::Transfer => match packet_id {
                    _ => Ok(Box::new({
                        let mut unknown = Unknown::from_bytes(&mut bytes)?;
                        unknown.packet_id = packet_id;
                        unknown.server_bounded = server_bounded;
                        unknown
                    })),
                },
                State::Configuration => match packet_id {
                    0 => Ok(Box::new(ClientInformation::from_bytes(&mut bytes)?)),
                    2 => Ok(Box::new(ServerboundPluginMessage::from_bytes(&mut bytes)?)),
                    7 => Ok(Box::new(ServerboundKnownPacks::from_bytes(&mut bytes)?)),
                    _ => Ok(Box::new({
                        let mut unknown = Unknown::from_bytes(&mut bytes)?;
                        unknown.packet_id = packet_id;
                        unknown.server_bounded = server_bounded;
                        unknown
                    })),
                },
            }
        } else {
            // Client bounded
            match self.state {
                State::Handshaking => bail!("There is no client bounded Handshaking packet"), // TODO: Create custom error type
                State::Status => match packet_id {
                    0 => Ok(Box::new(StatusResponse::from_bytes(&mut bytes)?)),
                    1 => Ok(Box::new(PongResponse::from_bytes(&mut bytes)?)),
                    _ => Ok(Box::new({
                        let mut unknown = Unknown::from_bytes(&mut bytes)?;
                        unknown.packet_id = packet_id;
                        unknown.server_bounded = server_bounded;
                        unknown
                    })),
                },
                State::Login => match packet_id {
                    2 => Ok(Box::new(LoginSuccess::from_bytes(&mut bytes)?)),
                    3 => {
                        let packet = SetCompression::from_bytes(&mut bytes)?;
                        self.compression = packet.threshold.value >= 0;
                        if self.compression {
                            info!("Compression enabled");
                        } else {
                            info!("Compression disabled");
                        }

                        Ok(Box::new(packet))
                    }
                    _ => Ok(Box::new({
                        let mut unknown = Unknown::from_bytes(&mut bytes)?;
                        unknown.packet_id = packet_id;
                        unknown.server_bounded = server_bounded;
                        unknown
                    })),
                },
                State::Transfer => match packet_id {
                    _ => Ok(Box::new({
                        let mut unknown = Unknown::from_bytes(&mut bytes)?;
                        unknown.packet_id = packet_id;
                        unknown.server_bounded = server_bounded;
                        unknown
                    })),
                },
                State::Configuration => match packet_id {
                    1 => Ok(Box::new(ClientboundPluginMessage::from_bytes(&mut bytes)?)),
                    12 => Ok(Box::new(FeatureFlags::from_bytes(&mut bytes)?)),
                    14 => Ok(Box::new(ClientboundKnownPacks::from_bytes(&mut bytes)?)),
                    _ => Ok(Box::new({
                        let mut unknown = Unknown::from_bytes(&mut bytes)?;
                        unknown.packet_id = packet_id;
                        unknown.server_bounded = server_bounded;
                        unknown
                    })),
                },
            }
        }
    }
}
