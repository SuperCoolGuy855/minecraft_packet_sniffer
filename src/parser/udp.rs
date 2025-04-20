use crate::parser::error::PacketError;
use crate::parser::{slice_to_array_unchecked, PacketParser};
use color_eyre::eyre::Context;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct UDPPacketInfo {
    pub src_port: u16,
    pub dst_port: u16,
    pub checksum: u16,
    pub payload: Vec<u8>,
}

impl PacketParser for UDPPacketInfo {
    fn from_bytes(data: &[u8]) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let src_port = u16::from_be_bytes(slice_to_array_unchecked(
            data.get(0..2)
                .ok_or(PacketError::IncorrectLength)
                .wrap_err("Not enough data to parse src_port")?,
        ));

        let dst_port = u16::from_be_bytes(slice_to_array_unchecked(
            data.get(2..4)
                .ok_or(PacketError::IncorrectLength)
                .wrap_err("Not enough data to parse dst_port")?,
        ));

        let checksum = u16::from_be_bytes(slice_to_array_unchecked(
            data.get(6..8)
                .ok_or(PacketError::IncorrectLength)
                .wrap_err("Not enough data to parse checksum")?,
        ));

        let payload = data[8..].to_vec(); // TODO: Check error

        Ok(Self {
            src_port,
            dst_port,
            checksum,
            payload,
        })
    }
}

impl Display for UDPPacketInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UDP | Src port: {}, Dst port: {}, Checksum: {}",
            self.src_port, self.dst_port, self.checksum
        )
    }
}
