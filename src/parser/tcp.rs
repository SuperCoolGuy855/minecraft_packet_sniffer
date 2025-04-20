use crate::parser::error::PacketError;
use crate::parser::ether_ip::IPPacketInfo;
use crate::parser::{slice_to_array_unchecked, PacketParser};
use bitflags::bitflags;
use color_eyre::eyre::Context;
use std::fmt::{Display, Formatter};
// #[derive(Debug, Copy, Clone)]
// #[repr(u8)]
// pub enum TCPFlag {
//     Fin = 1,
//     Syn = 2,
//     Rst = 4,
//     Psh = 8,
//     Ack = 16,
//     Urg = 32,
// }

bitflags! {
    #[derive(Clone, Debug)]
    pub struct TCPFlag: u16 {
        const FIN = 0b1;
        const SYN = 0b10;
        const RST = 0b100;
        const PSH = 0b1000;
        const ACK = 0b10000;
        const URG = 0b100000;
        const ECE = 0b1000000;
        const CWR = 0b10000000;
    }
}

#[derive(Debug, Clone)]
pub struct TCPPacketInfo {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub flags: TCPFlag,
    pub window_size: u16,
    pub checksum: u16,
    pub options: Option<Vec<u8>>, // TODO: Make struct
    pub payload: Vec<u8>,
}

impl PacketParser for TCPPacketInfo {
    // FIXME: This is incorrect, refer back to TCP header
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

        let seq_num = u32::from_be_bytes(slice_to_array_unchecked(
            data.get(4..8)
                .ok_or(PacketError::IncorrectLength)
                .wrap_err("Not enough data to parse seq_num")?,
        ));

        let ack_num = u32::from_be_bytes(slice_to_array_unchecked(
            data.get(8..12)
                .ok_or(PacketError::IncorrectLength)
                .wrap_err("Not enough data to parse ack_num")?,
        ));

        let flags_num = u16::from_be_bytes(slice_to_array_unchecked(
            data.get(12..14)
                .ok_or(PacketError::IncorrectLength)
                .wrap_err("Not enough data to parse flags")?,
        ));
        let flags = TCPFlag::from_bits_truncate(flags_num);

        let data_offset_bytes = ((flags_num & 0b1111000000000000) >> 12) * 4;

        let window_size = u16::from_be_bytes(slice_to_array_unchecked(
            data.get(14..16)
                .ok_or(PacketError::IncorrectLength)
                .wrap_err("Not enough data to parse window_size")?,
        ));

        let checksum = u16::from_be_bytes(slice_to_array_unchecked(
            data.get(16..18)
                .ok_or(PacketError::IncorrectLength)
                .wrap_err("Not enough data to parse window_size")?,
        ));

        let options = if data_offset_bytes > 20 {
            Some(
                data.get(20..data_offset_bytes as usize)
                    .ok_or(PacketError::IncorrectLength)
                    .wrap_err("Not enough data to parse window_size")?
                    .to_vec(),
            )
        } else {
            None
        };

        let payload = data[data_offset_bytes as usize..].to_vec(); // TODO: Check error

        Ok(Self {
            src_port,
            dst_port,
            seq_num,
            ack_num,
            flags,
            window_size,
            checksum,
            options,
            payload,
        })
    }
}

impl Display for TCPPacketInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TCP | Src port: {}, \
               Dst port: {}, \
               Sequence number: {}, \
               Acknowledge number: {}, \
               Flags: {:?}, \
               Window size: {}, \
               Checksum: {}, \
               Option: {:?}",
            self.src_port,
            self.dst_port,
            self.seq_num,
            self.ack_num,
            self.flags,
            self.window_size,
            self.checksum,
            self.options
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tcp_flags_test() {
        let temp = TCPFlag::from_bits_truncate(0x1218);
        println!("{temp:?}");
    }

    #[test]
    fn test() {
        let temp = ((0x8012 & 0b1111000000000000) >> 12) * 4;
        println!("{temp} {temp:b}");
    }
}
