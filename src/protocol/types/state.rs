use std::fmt::{Display, Formatter};
use color_eyre::eyre::OptionExt;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use crate::protocol::types::error::DatatypeError;

#[derive(FromPrimitive, Debug, Copy, Clone)]
#[repr(i32)]
pub enum State {
    Handshaking = 0,
    Status = 1,
    Login = 2,
    Transfer = 3,
    Configuration = 4,
}

impl State {
    pub fn from_num(num: i32) -> color_eyre::Result<Self> {
        Self::from_i32(num).ok_or_eyre(DatatypeError::UnknownState(num))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            State::Handshaking => "Handshaking",
            State::Status => "Status",
            State::Login => "Login",
            State::Transfer => "Transfer",
            State::Configuration => "Configuration"
        })
    }
}