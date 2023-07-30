use serde::Deserialize;
use std::{error::Error as StdError, fmt::Display, result::Result as StdResult};

/// This type is used to simplify a lot of the function return types in this library.
pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
    msg: String,
}

impl Error {
    fn new(kind: Kind, msg: String) -> Self {
        Error { kind, msg }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

impl From<LichessAPIError> for Error {
    fn from(value: LichessAPIError) -> Self {
        Error::new(Kind::LichessAPI, value.error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::new(Kind::Reqwest, value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::new(Kind::Json, value.to_string())
    }
}

#[derive(Debug)]
pub(crate) enum Kind {
    LichessAPI,
    Reqwest,
    Json,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "JSON error"),
            Self::LichessAPI => write!(f, "Lichess API error"),
            Self::Reqwest => write!(f, "reqwest error"),
        }
    }
}

/// APIError struct
#[derive(Debug, Deserialize)]
pub struct LichessAPIError {
    error: String,
}

impl StdError for LichessAPIError {
    fn description(&self) -> &str {
        &self.error
    }
}

impl Display for LichessAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
