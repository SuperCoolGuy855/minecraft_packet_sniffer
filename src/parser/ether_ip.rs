use super::{error::PacketError, slice_to_array_unchecked, PacketParser};
use color_eyre::eyre::{bail, WrapErr};
use color_eyre::Result;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone)]
pub struct IPPacketInfo {
    pub src_mac: [u8; 6],     // TODO: Make struct for better display
    pub dst_mac: [u8; 6],     // TODO: Make struct for better display
    pub ether_type: u16,      // TODO: Make enum
    pub payload_protocol: u8, // TODO: Make enum
    pub src_ip: IpAddr,
    pub dst_ip: IpAddr,
    pub payload: Vec<u8>,
}

impl PacketParser for IPPacketInfo {
    fn from_bytes(data: &[u8]) -> Result<Self> {
        let dst_mac: [u8; 6] = slice_to_array_unchecked(
            data.get(0..6)
                .ok_or(PacketError::IncorrectLength)
                .wrap_err("Not enough data to parse dst_mac")?,
        );

        let src_mac: [u8; 6] = slice_to_array_unchecked(
            data.get(6..12)
                .ok_or(PacketError::IncorrectLength)
                .wrap_err("Not enough data to parse src_mac")?,
        );

        let ether_type: u16 = u16::from_be_bytes(slice_to_array_unchecked(
            data.get(12..14)
                .ok_or(PacketError::IncorrectLength)
                .wrap_err("Not enough data to parse ether_type")?,
        ));

        let (payload_protocol, src_ip, dst_ip, payload_start, payload_end) = match ether_type {
            0x0800 => {
                // IPv4
                let header_length = (*data
                    .get(14)
                    .ok_or(PacketError::IncorrectLength)
                    .wrap_err("Not enough data to parse header_length")?
                    & 0b00001111)
                    * 4;

                let total_length_bytes: [u8; 2] = slice_to_array_unchecked(
                    data.get(16..18)
                        .ok_or(PacketError::IncorrectLength)
                        .wrap_err("Not enough data to parse total_length")?,
                );
                let total_length = u16::from_be_bytes(total_length_bytes);

                let payload_length = total_length - header_length as u16;

                let protocol = *data
                    .get(23)
                    .ok_or(PacketError::IncorrectLength)
                    .wrap_err("Not enough data to parse payload_protocol")?;

                let src_ip_bytes: [u8; 4] = slice_to_array_unchecked(
                    data.get(26..30)
                        .ok_or(PacketError::IncorrectLength)
                        .wrap_err("Not enough data to parse src_ip_bytes")?,
                );
                let src_ip = IpAddr::V4(Ipv4Addr::from(src_ip_bytes));

                let dst_ip_bytes: [u8; 4] = slice_to_array_unchecked(
                    data.get(30..34)
                        .ok_or(PacketError::IncorrectLength)
                        .wrap_err("Not enough data to parse dst_ip_bytes")?,
                );
                let dst_ip = IpAddr::V4(Ipv4Addr::from(dst_ip_bytes));

                (protocol, src_ip, dst_ip, 34, 34 + payload_length as usize)
            }
            0x86DD => {
                // IPv6
                let payload_length_bytes: [u8; 2] = slice_to_array_unchecked(
                    data.get(18..20)
                        .ok_or(PacketError::IncorrectLength)
                        .wrap_err("Not enough data to parse payload_length")?,
                );
                let payload_length = u16::from_be_bytes(payload_length_bytes);

                let protocol = *data
                    .get(20)
                    .ok_or(PacketError::IncorrectLength)
                    .wrap_err("Not enough data to parse payload_protocol")?;

                let src_ip_bytes: [u8; 16] = slice_to_array_unchecked(
                    data.get(22..38)
                        .ok_or(PacketError::IncorrectLength)
                        .wrap_err("Not enough data to parse src_ip_bytes")?,
                );
                let src_ip = IpAddr::V6(Ipv6Addr::from(src_ip_bytes));

                let dst_ip_bytes: [u8; 16] = slice_to_array_unchecked(
                    data.get(38..54)
                        .ok_or(PacketError::IncorrectLength)
                        .wrap_err("Not enough data to parse dst_ip_bytes")?,
                );
                let dst_ip = IpAddr::V6(Ipv6Addr::from(dst_ip_bytes));

                (protocol, src_ip, dst_ip, 54, 54 + payload_length as usize)
            }

            _ => {
                bail!(PacketError::UnknownProtocol(ether_type));
            }
        };

        let payload = data[payload_start..payload_end].to_vec(); // TODO: Check for error

        Ok(Self {
            src_mac,
            dst_mac,
            ether_type,
            payload_protocol,
            src_ip,
            dst_ip,
            payload,
        })
    }
}

impl Display for IPPacketInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EtherIP | Src MAC: {:x?}, Dst MAC: {:x?}, EtherType: {:x}, Protocol: {}, Src IP: {}, Dst IP: {}",
            self.src_mac,
            self.dst_mac,
            self.ether_type,
            self.payload_protocol,
            self.src_ip,
            self.dst_ip
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        IPPacketInfo::from_bytes(b"123").unwrap();
    }
}
