use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum ExplorerError {
    Custom(String),
    DBError(sqlx::Error),
    IOError(std::io::Error),
    TomlDeError(toml::de::Error),
    HexError(rustc_hex::FromHexError),
    ParseUrlError(url::ParseError),
    SerdeError(serde_json::Error),
}

impl From<serde_json::Error> for ExplorerError {
    fn from(e: serde_json::Error) -> Self {
        ExplorerError::SerdeError(e)
    }
}

impl From<String> for ExplorerError {
    fn from(e: String) -> Self {
        ExplorerError::Custom(e)
    }
}

impl From<url::ParseError> for ExplorerError {
    fn from(e: url::ParseError) -> Self {
        ExplorerError::ParseUrlError(e)
    }
}

impl From<rustc_hex::FromHexError> for ExplorerError {
    fn from(e: rustc_hex::FromHexError) -> Self {
        ExplorerError::HexError(e)
    }
}

impl From<std::io::Error> for ExplorerError {
    fn from(e: std::io::Error) -> Self {
        ExplorerError::IOError(e)
    }
}

impl From<toml::de::Error> for ExplorerError {
    fn from(e: toml::de::Error) -> Self {
        ExplorerError::TomlDeError(e)
    }
}

impl From<sqlx::Error> for ExplorerError {
    fn from(e: sqlx::Error) -> Self {
        ExplorerError::DBError(e)
    }
}

pub type Result<T> = core::result::Result<T, ExplorerError>;

impl IntoResponse for ExplorerError {
    fn into_response(self) -> Response {
        let err_msg = match self {
            ExplorerError::Custom(e) => e,
            ExplorerError::DBError(e) => e.to_string(),
            ExplorerError::IOError(e) => e.to_string(),
            ExplorerError::TomlDeError(e) => e.to_string(),
            ExplorerError::HexError(e) => e.to_string(),
            ExplorerError::ParseUrlError(e) => e.to_string(),
            ExplorerError::SerdeError(e) => e.to_string(),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, err_msg).into_response()
    }
}
