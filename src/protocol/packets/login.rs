use crate::protocol::packets::format::direction_str;
use crate::protocol::packets::MinecraftPacket;
use crate::protocol::types::{Datatype, PlayerProperty, VarInt};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LoginStart {
    pub name: String,
    pub uuid: Uuid,
}

impl MinecraftPacket for LoginStart {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let name = String::from_bytes(bytes)?;
        let uuid = <Uuid as Datatype>::from_bytes(bytes)?;

        Ok(Self { name, uuid })
    }
}

impl Display for LoginStart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | LoginStart | Name: {}, UUID: {}",
            direction_str(true),
            self.name,
            self.uuid
        )
    }
}

#[derive(Debug, Clone)]
pub struct SetCompression {
    pub threshold: VarInt,
}

impl MinecraftPacket for SetCompression {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let threshold = VarInt::from_bytes(bytes)?;
        Ok(Self { threshold })
    }
}

impl Display for SetCompression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | SetCompression | Threshold: {}",
            direction_str(false),
            self.threshold
        )
    }
}

#[derive(Debug)]
pub struct LoginSuccess {
    uuid: Uuid,
    username: String,
    property: Vec<PlayerProperty>,
}

impl MinecraftPacket for LoginSuccess {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let uuid = <Uuid as Datatype>::from_bytes(bytes)?;
        let username = String::from_bytes(bytes)?;
        let property = Vec::from_bytes(bytes)?;

        Ok(Self {
            uuid,
            username,
            property,
        })
    }
}

impl Display for LoginSuccess {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | LoginSuccess | UUID: {}, Username: {}, Properties: ",
            direction_str(false),
            self.uuid,
            self.username
        )?;
        for property in &self.property {
            write!(f, "{property:?}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LoginAcknowledged;

impl MinecraftPacket for LoginAcknowledged {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self)
    }
}

impl Display for LoginAcknowledged {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | LoginAcknowledged", direction_str(true))
    }
}
