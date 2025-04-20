use thiserror::Error;

#[derive(Debug, Copy, Clone, Error)]
pub enum DatatypeError {
    #[error("Input is too big for {0}")]
    TooBig(&'static str),
    #[error("Not enough data to parse")]
    NotEnoughData,
    #[error("Unknown state: {0}")]
    UnknownState(i32),
    #[error("Invalid input")]
    Invalid,
}
