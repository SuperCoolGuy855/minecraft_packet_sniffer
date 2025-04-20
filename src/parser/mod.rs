pub mod error;
pub mod ether_ip;
pub mod tcp;
pub mod udp;

mod test;

use std::fmt::Debug;
use color_eyre::Result;

pub trait PacketParser {
    fn from_bytes(data: &[u8]) -> Result<Self>
    where
        Self: Sized;
}


pub fn slice_to_array_unchecked<'a, T, const N: usize>(slice: &'a [T]) -> [T; N]
where
    [T; N]: TryFrom<&'a [T]> + Debug,
    <[T; N] as TryFrom<&'a [T]>>::Error: Debug,
{
    slice.try_into().unwrap()
}