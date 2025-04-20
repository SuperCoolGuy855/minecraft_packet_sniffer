use super::Datatype;
use super::VarInt;
use itertools::Itertools;
use std::collections::VecDeque;

impl Datatype for String {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let length = VarInt::from_bytes(bytes)?;
        let string_bytes = bytes.drain(..length.value as usize).collect_vec();
        Ok(String::from_utf8(string_bytes)?)
    }
}
