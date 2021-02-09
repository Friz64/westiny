use serde::{Serialize, Deserialize};
use derive_new::new;
use std::fmt::{Display, Debug, Formatter};
use crate::components::Input;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PacketType {
    ConnectionRequest {
        player_name: String
    },
    ConnectionResponse(Result<ClientInitialData>),
    InputState {
        input: Input
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ClientInitialData {
    pub player_network_id: u64,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    AlreadyConnected,
    Other,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let literal = match self {
            ErrorKind::AlreadyConnected => "Client already connected",
            ErrorKind::Other => "Other error"
        };

        write!(f, "{}", literal)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, new, Eq, PartialEq)]
pub struct Error {
    error_kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Westiny network error: {}", self.error_kind)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;