use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColgadoRequestsError {
    #[error("Client id not found")]
    VarError {
        #[from]
        err: GettingDataError,
    },
    #[error("Invalid token: {}", err)]
    InvalidToken {
        #[from]
        err: TokenError,
    },
    #[error("WebSocket error: {}", err)]
    WebSockerError {
        #[from]
        err: reqwest::Error,
    },
}

#[derive(Error, Debug)]
pub enum GettingDataError {
    #[error("File not found")]
    FileNotFound(#[from] io::Error),
    #[error("Client id or urls not found")]
    VarError(#[from] toml::de::Error),
}

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("The sended and the received states must be the same")]
    DifferentStates,
    #[error("Token not received")]
    TokenNotReceived,
    #[error("Validating token")]
    InvalidToken,
    #[error("Deserializing json")]
    DeserializingError,
    #[error("{}", err)]
    IoError {
        #[from]
        err: io::Error,
    },
    #[error("{}", err)]
    ReqwestError {
        #[from]
        err: reqwest::Error,
    },
    #[error("Invalid address {}", err)]
    InvalidAddress {
        #[from]
        err: url::ParseError,
    },
}
