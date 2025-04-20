use std::collections::VecDeque;
use color_eyre::eyre::OptionExt;
use itertools::Itertools;
use uuid::Uuid;
use crate::protocol::types::Datatype;
use crate::protocol::types::error::DatatypeError;

impl Datatype for Uuid {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        let uuid_bytes = bytes.drain(..16).collect_array().ok_or_eyre(DatatypeError::NotEnoughData)?;
        Ok(Uuid::from_bytes(uuid_bytes))
    }
}