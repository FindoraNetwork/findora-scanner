use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::Error::RowNotFound;
use std::num::ParseFloatError;

#[derive(Debug)]
pub enum ExplorerError {
    Custom(String),
    DBErr(sqlx::Error),
    IOErr(std::io::Error),
    TomlDeErr(toml::de::Error),
    HexErr(rustc_hex::FromHexError),
    ParseUrlErr(url::ParseError),
    SerdeJsonErr(serde_json::Error),
    ReqwestErr(reqwest::Error),
    ParseFloatErr(ParseFloatError),
}
impl From<ParseFloatError> for ExplorerError {
    fn from(e: ParseFloatError) -> Self {
        ExplorerError::ParseFloatErr(e)
    }
}
impl From<reqwest::Error> for ExplorerError {
    fn from(e: reqwest::Error) -> Self {
        ExplorerError::ReqwestErr(e)
    }
}

impl From<serde_json::Error> for ExplorerError {
    fn from(e: serde_json::Error) -> Self {
        ExplorerError::SerdeJsonErr(e)
    }
}

impl From<String> for ExplorerError {
    fn from(e: String) -> Self {
        ExplorerError::Custom(e)
    }
}

impl From<url::ParseError> for ExplorerError {
    fn from(e: url::ParseError) -> Self {
        ExplorerError::ParseUrlErr(e)
    }
}

impl From<rustc_hex::FromHexError> for ExplorerError {
    fn from(e: rustc_hex::FromHexError) -> Self {
        ExplorerError::HexErr(e)
    }
}

impl From<std::io::Error> for ExplorerError {
    fn from(e: std::io::Error) -> Self {
        ExplorerError::IOErr(e)
    }
}

impl From<toml::de::Error> for ExplorerError {
    fn from(e: toml::de::Error) -> Self {
        ExplorerError::TomlDeErr(e)
    }
}

impl From<sqlx::Error> for ExplorerError {
    fn from(e: sqlx::Error) -> Self {
        ExplorerError::DBErr(e)
    }
}

pub type Result<T> = core::result::Result<T, ExplorerError>;

impl IntoResponse for ExplorerError {
    fn into_response(self) -> Response {
        let err_msg = match self {
            ExplorerError::Custom(e) => e,
            ExplorerError::DBErr(e) => {
                if let RowNotFound = e {
                    return (StatusCode::NOT_FOUND, "not found").into_response();
                }
                e.to_string()
            }
            ExplorerError::IOErr(e) => e.to_string(),
            ExplorerError::TomlDeErr(e) => e.to_string(),
            ExplorerError::HexErr(e) => e.to_string(),
            ExplorerError::ParseUrlErr(e) => e.to_string(),
            ExplorerError::SerdeJsonErr(e) => e.to_string(),
            ExplorerError::ReqwestErr(e) => e.to_string(),
            ExplorerError::ParseFloatErr(e) => e.to_string(),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, err_msg).into_response()
    }
}
