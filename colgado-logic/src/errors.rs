use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColgadoLogicError {
    #[error("{}", err)]
    RequestError {
        #[from]
        err: trequests::errors::TRequestsError,
    },
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Word too long")]
    InvalidWord,
}
impl GameError {
    pub fn twitch_message_error(&self) -> &'static str {
        match self {
            GameError::InvalidWord => "La palabra es demasiado larga",
        }
    }
}
