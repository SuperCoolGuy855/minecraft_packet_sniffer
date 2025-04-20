mod handshaking;
mod status;
mod format;
mod login;
mod unknown;
mod configuration;

use std::any::Any;
use std::collections::VecDeque;
use std::fmt::{Debug, Display};
pub use handshaking::*;
pub use status::*;
pub use login::*;
pub use configuration::*;
pub use unknown::*;

pub trait MinecraftPacket: Debug + Display + Any {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized;
}