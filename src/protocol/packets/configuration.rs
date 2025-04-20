use crate::protocol::packets::format::direction_str;
use crate::protocol::packets::MinecraftPacket;
use crate::protocol::types::*;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct ServerboundPluginMessage {
    channel: Identifier,
    data: Vec<u8>,
}

impl MinecraftPacket for ServerboundPluginMessage {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let channel = Identifier::from_bytes(bytes)?;
        let length = bytes.len();
        let data = Vec::from_array_bytes(length, bytes)?;

        Ok(Self { channel, data })
    }
}

impl Display for ServerboundPluginMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | ServerboundPluginMessage | Channel: {}, Data: {:?}",
            direction_str(true),
            self.channel,
            self.data
        )
    }
}

#[derive(Debug, Clone)]
pub struct ClientInformation {
    locale: String,
    view_distance: i8,
    chat_mode: VarInt,
    chat_colors: bool,
    displayed_skin_parts: u8,
    main_hand: VarInt,
    enable_text_filtering: bool,
    allow_server_listings: bool,
    particle_status: VarInt,
}

impl MinecraftPacket for ClientInformation {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let locale = String::from_bytes(bytes)?;
        let view_distance = i8::from_bytes(bytes)?;
        let chat_mode = VarInt::from_bytes(bytes)?;
        let chat_colors = bool::from_bytes(bytes)?;
        let displayed_skin_parts = u8::from_bytes(bytes)?;
        let main_hand = VarInt::from_bytes(bytes)?;
        let enable_text_filtering = bool::from_bytes(bytes)?;
        let allow_server_listings = bool::from_bytes(bytes)?;
        let particle_status = VarInt::from_bytes(bytes)?;

        Ok(Self {
            locale,
            view_distance,
            chat_mode,
            chat_colors,
            displayed_skin_parts,
            main_hand,
            enable_text_filtering,
            allow_server_listings,
            particle_status,
        })
    }
}

impl Display for ClientInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let chat_mode = {
            match self.chat_mode.value {
                0 => "Enabled",
                1 => "Commands only",
                2 => "Hidden",
                _ => "Unknown",
            }
        };

        let skin_parts = {
            let mut skin_parts_vec = vec![];
            let bit = self.displayed_skin_parts;
            if bit & 0x01 != 0 {
                skin_parts_vec.push("cape");
            }
            if bit & 0x02 != 0 {
                skin_parts_vec.push("jacket");
            }
            if bit & 0x04 != 0 {
                skin_parts_vec.push("left sleeve");
            }
            if bit & 0x08 != 0 {
                skin_parts_vec.push("right sleeve");
            }
            if bit & 0x10 != 0 {
                skin_parts_vec.push("left pants leg");
            }
            if bit & 0x20 != 0 {
                skin_parts_vec.push("right pants leg");
            }
            if bit & 0x40 != 0 {
                skin_parts_vec.push("hat");
            }
            skin_parts_vec.join(" - ")
        };

        let main_hand = {
            match self.main_hand.value {
                0 => "Left",
                1 => "Right",
                _ => "Unknown",
            }
        };

        let particle_status = {
            match self.particle_status.value {
                0 => "All",
                1 => "Decreased",
                2 => "Minimal",
                _ => "Unknown",
            }
        };

        write!(
            f,
            "{} | ClientInformation | Locale: {}, \
            View distance: {}, \
            Chat mode: {}, \
            Chat colors: {}, \
            Skin parts: {}, \
            Main hand: {}, \
            Text filtering: {}, \
            Server listings: {}, \
            Particle status: {}",
            direction_str(true),
            self.locale,
            self.view_distance,
            chat_mode,
            self.chat_colors,
            skin_parts,
            main_hand,
            self.enable_text_filtering,
            self.allow_server_listings,
            particle_status
        )
    }
}

#[derive(Debug, Clone)]
pub struct ClientboundPluginMessage {
    channel: Identifier,
    data: Vec<u8>,
}

impl MinecraftPacket for ClientboundPluginMessage {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let temp = ServerboundPluginMessage::from_bytes(bytes)?;
        Ok(Self{
            channel: temp.channel,
            data: temp.data,
        })
    }
}

impl Display for ClientboundPluginMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | ClientboundPluginMessage | Channel: {}, Data: {:?}",
            direction_str(false),
            self.channel,
            self.data
        )
    }
}

#[derive(Debug, Clone)]
pub struct FeatureFlags {
    feature_flags: Vec<Identifier>,
}

impl Display for FeatureFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let feature_flags = self.feature_flags.iter().map(|x| x.to_string()).join(", ");
        write!(f, "{} | FeatureFlags | Feature flags: [{}]", direction_str(false), feature_flags)
    }
}

impl MinecraftPacket for FeatureFlags {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        let feature_flags = Vec::from_bytes(bytes)?;
        Ok(Self {
            feature_flags,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClientboundKnownPacks {
    known_packs: Vec<KnownPack>
}

impl MinecraftPacket for ClientboundKnownPacks {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        let known_packs = Vec::from_bytes(bytes)?;

        Ok(Self {
            known_packs,
        })
    }
}

impl Display for ClientboundKnownPacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | ClientboundKnownPacks | Known packs: {:?}", direction_str(false), self.known_packs)
    }
}

#[derive(Debug, Clone)]
pub struct ServerboundKnownPacks {
    known_packs: Vec<KnownPack>
}

impl MinecraftPacket for ServerboundKnownPacks {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized
    {
        let temp = ClientboundKnownPacks::from_bytes(bytes)?;
        Ok(Self {
            known_packs: temp.known_packs,
        })
    }
}

impl Display for ServerboundKnownPacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | ServerboundKnownPacks | Known packs: {:?}", direction_str(true), self.known_packs)
    }
}
