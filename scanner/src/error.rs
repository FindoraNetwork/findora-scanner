#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    ChronoError(chrono::ParseError),
    ParseIntError(std::num::ParseIntError),
    Base64Error(base64::DecodeError),
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

pub type Result<T> = core::result::Result<T, Error>;
