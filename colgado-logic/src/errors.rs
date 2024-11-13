use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColgadoLogicError {
    #[error("{}", err)]
    RequestError {
        #[from]
        err: colgado_requests::errors::ColgadoRequestsError,
    },
}
