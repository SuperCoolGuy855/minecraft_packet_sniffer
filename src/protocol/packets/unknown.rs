use crate::protocol::packets::format::direction_str;
use crate::protocol::packets::MinecraftPacket;
use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Unknown {
    pub packet_id: i32,
    pub server_bounded: bool,
    pub payload: Vec<u8>,
}

impl MinecraftPacket for Unknown {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            packet_id: 0,
            server_bounded: false,
            payload: bytes.iter().copied().collect_vec(),
        })
    }
}

impl Display for Unknown {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | Unknown Packet | Packet ID: {}, Payload: {:?}",
            direction_str(self.server_bounded),
            self.packet_id,
            self.payload
        )
    }
}
