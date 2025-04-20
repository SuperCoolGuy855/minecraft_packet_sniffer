use std::collections::VecDeque;
use crate::protocol::types::Datatype;

#[derive(Debug, Clone)]
pub struct KnownPack {
    namespace: String,
    id: String,
    version: String,
}

impl Datatype for KnownPack {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        let namespace = String::from_bytes(bytes)?;
        let id = String::from_bytes(bytes)?;
        let version = String::from_bytes(bytes)?;

        Ok(Self {
            namespace,
            id,
            version,
        })
    }
}