use crate::protocol::types::error::DatatypeError;
use crate::protocol::types::Datatype;
use color_eyre::eyre::{OptionExt, WrapErr};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Identifier {
    pub namespace: String,
    pub value: String,
}

impl Datatype for Identifier {
    fn from_bytes(bytes: &mut VecDeque<u8>) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        let data = String::from_bytes(bytes)?;
        let (namespace, value) = data
            .split_once(":")
            .ok_or_eyre(DatatypeError::Invalid)
            .wrap_err(format!("Can't parse Identifier: {data}"))?;

        Ok(Self {
            namespace: namespace.to_string(),
            value: value.to_string(),
        })
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.value)
    }
}
