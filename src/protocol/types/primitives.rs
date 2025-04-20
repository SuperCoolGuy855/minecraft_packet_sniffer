use super::Datatype;
use crate::protocol::types::error::DatatypeError;
use itertools::Itertools;
use std::collections::VecDeque;
use color_eyre::eyre::OptionExt;

impl Datatype for u16 {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let short_bytes = bytes
            .drain(..2)
            .collect_array()
            .ok_or(DatatypeError::NotEnoughData)?;
        Ok(u16::from_be_bytes(short_bytes))
    }
}

impl Datatype for i64 {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let long_bytes = bytes
            .drain(..8)
            .collect_array()
            .ok_or(DatatypeError::NotEnoughData)?;
        Ok(i64::from_be_bytes(long_bytes))
    }
}

impl Datatype for bool {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        let bool_bytes = bytes.pop_front().ok_or(DatatypeError::NotEnoughData)?;
        Ok(bool_bytes == 1)
    }
}

impl Datatype for u8 {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        bytes.pop_front().ok_or_eyre(DatatypeError::NotEnoughData)
    }
}

impl Datatype for i8 {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        Ok(u8::from_bytes(bytes)? as i8)
    }
}