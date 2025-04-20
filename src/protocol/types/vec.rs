use crate::protocol::types::{Datatype, MinecraftArray, VarInt};
use std::collections::VecDeque;

impl<T> Datatype for Vec<T>
where
    T: Datatype,
{
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let len = VarInt::from_bytes(bytes)?.value as usize;
        Vec::from_array_bytes(len, bytes)
    }
}

impl<T> MinecraftArray for Vec<T>
where
    T: Datatype,
{
    fn from_array_bytes(len: usize, bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self> {
        let mut output = Vec::with_capacity(len);
        for _ in 0..len {
            output.push(T::from_bytes(bytes)?);
        }

        Ok(output)
    }
}
