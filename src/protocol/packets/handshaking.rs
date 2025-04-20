use super::MinecraftPacket;
use crate::parser::PacketParser;
use crate::protocol::types::{Datatype, State, VarInt};
use num_traits::FromPrimitive;
use owo_colors::OwoColorize;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::protocol::packets::format::direction_str;

#[derive(Debug, Clone)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: State,
}

impl MinecraftPacket for Handshake {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let protocol_version = VarInt::from_bytes(bytes)?;
        let server_address = String::from_bytes(bytes)?;
        let server_port = u16::from_bytes(bytes)?;
        let next_state_int = VarInt::from_bytes(bytes)?;
        let next_state = State::from_num(next_state_int.value)?;

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}

impl Display for Handshake {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | Handshake | Protocol version: {}, Server IP: {}, Server port: {}, Next state: {}",
            direction_str(true),
            self.protocol_version,
            self.server_address,
            self.server_port,
            self.next_state
        )
    }
}
