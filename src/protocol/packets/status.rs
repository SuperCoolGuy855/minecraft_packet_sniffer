use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use owo_colors::OwoColorize;
use crate::protocol::packets::format::direction_str;
use crate::protocol::packets::MinecraftPacket;
use crate::protocol::types::Datatype;

#[derive(Debug, Clone)]
pub struct StatusRequest;

impl MinecraftPacket for StatusRequest {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        Ok(Self)
    }
}

impl Display for StatusRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | StatusRequest", direction_str(true))
    }
}

#[derive(Debug, Clone)]
pub struct StatusResponse {
    pub json_response: String
}

impl MinecraftPacket for StatusResponse {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        let json_response = String::from_bytes(bytes)?;

         Ok(Self {
             json_response,
         })
    }
}

impl Display for StatusResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | StatusResponse | JSON response: {}", direction_str(false), self.json_response)
    }
}

#[derive(Debug, Clone)]
pub struct PingRequest {
    pub timestamp: i64
}

impl MinecraftPacket for PingRequest {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        let timestamp = i64::from_bytes(bytes)?;
        Ok(Self {
            timestamp,
        })
    }
}

impl Display for PingRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | PingRequest | Timestamp: {}", direction_str(true), self.timestamp)
    }
}

#[derive(Debug, Clone)]
pub struct PongResponse {
    pub timestamp: i64
}

impl MinecraftPacket for PongResponse {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        let timestamp = i64::from_bytes(bytes)?;
        Ok(Self {
            timestamp,
        })
    }
}

impl Display for PongResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | PongResponse | Timestamp: {}", direction_str(false), self.timestamp)
    }
}