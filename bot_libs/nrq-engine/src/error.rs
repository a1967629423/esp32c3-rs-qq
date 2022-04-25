#[cfg(not(feature = "std"))]
use super::simulate_std as std;
#[cfg(not(feature = "std"))]
use std::prelude::*;
use thiserror::Error;
pub type RQResult<T> = Result<T, RQError>;

#[derive(Error, Debug)]
pub enum RQError {
    #[error("other error {0}")]
    Other(String),

    #[error("failed to decode, {0}")]
    Decode(String),

    #[error("command_name mismatch, expected {0} get {1}")]
    CommandNameMismatch(String, String),

    #[error("timeout error")]
    Timeout,

    #[error("network error")]
    Network,

    #[error("jce error, {0}")]
    Jce(#[from] jcers::JceError),
    #[error("io error, {0}")]
    IO(String),

    #[error("unknown flag")]
    UnknownFlag,

    #[error("unknown encrypt type")]
    UnknownEncryptType,

    #[error("invalid packet type")]
    InvalidPacketType,
    #[error("invalid encrypt type")]
    InvalidEncryptType,
    #[error("packet dropped")]
    PacketDropped,
    #[error("session expired")]
    SessionExpired,
    #[error("session expired, {0}")]
    UnsuccessfulRetCode(i32),

    #[error("Token login failed")]
    TokenLoginFailed,
}
impl From<RQError> for () {
    fn from(_s: RQError) -> Self {
        ()
    }
}
