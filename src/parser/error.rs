use thiserror::Error;

#[derive(Debug, Copy, Clone, Error)]
pub enum PacketError {
    #[error("Packet is invalid")]
    Invalid,
    #[error("Packet has wrong length")]
    IncorrectLength,
    #[error("Packet has unknown protocol: {0}")]
    UnknownProtocol(u16),
}
