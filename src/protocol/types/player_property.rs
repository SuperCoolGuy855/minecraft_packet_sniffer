use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::protocol::types::Datatype;

#[derive(Debug, Clone)]
pub struct PlayerProperty {
    name: String,
    value: String,
    signature: Option<String>
}

impl Datatype for PlayerProperty {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        let name = String::from_bytes(bytes)?;
        let value = String::from_bytes(bytes)?;

        let sig_is_present = bool::from_bytes(bytes)?;
        let signature = if sig_is_present {
            Some(String::from_bytes(bytes)?)
        } else {
            None
        };

        Ok(Self {
            name,
            value,
            signature,
        })
    }
}