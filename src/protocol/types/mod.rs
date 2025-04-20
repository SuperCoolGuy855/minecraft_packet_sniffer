pub mod error;
mod var_num;
mod string;
mod primitives;
mod state;
mod uuid;
mod player_property;
mod vec;
mod identifier;
mod known_pack;

use std::collections::VecDeque;

pub use var_num::*;
pub use primitives::*;
pub use state::State;
pub use player_property::*;
pub use identifier::*;
pub use vec::*;
pub use known_pack::*;

pub trait Datatype {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized;
}

pub trait MinecraftArray {
    fn from_array_bytes(len: usize, bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized;
}