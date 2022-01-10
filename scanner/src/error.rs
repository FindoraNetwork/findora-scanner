#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    ChronoError(chrono::ParseError),
    ParseIntError(std::num::ParseIntError),
    Base64Error(base64::DecodeError),
    SerdeJsonError(serde_json::Error),
    JoinError(tokio::task::JoinError),
    DBError(tokio_postgres::Error),
    TryIntoError(core::num::TryFromIntError),
    EvmTxParseError,
}

impl From<core::num::TryFromIntError> for Error {
    fn from(e: core::num::TryFromIntError) -> Self {
        Error::TryIntoError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Self {
        Error::ChronoError(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseIntError(e)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::Base64Error(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJsonError(e)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Self {
        Error::JoinError(e)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(e: tokio_postgres::Error) -> Self {
        Error::DBError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
