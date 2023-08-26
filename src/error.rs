use reqwest::StatusCode;
use std::{error::Error as StdError, fmt::Display, result::Result as StdResult};

/// Used to simplify a lot of the function return types in this library.
pub type Result<T> = StdResult<T, Error>;

/// A general, library-wide error type that will be returned in case of any error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Box<dyn StdError>,
}

impl Error {
    // Creates a new instance of `Error`.
    pub(crate) fn new<E>(kind: ErrorKind, source: E) -> Self
    where
        E: Into<Box<dyn StdError>>,
    {
        Error {
            kind,
            source: source.into(),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.source.as_ref())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.source)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::new(ErrorKind::IO, value)
    }
}

impl From<LichessAPIError> for Error {
    fn from(value: LichessAPIError) -> Self {
        Error::new(ErrorKind::LichessAPI, value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::new(ErrorKind::Reqwest, value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::new(ErrorKind::Json, value)
    }
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    IO,
    LichessAPI,
    Reqwest,
    Json,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO => write!(f, "IO error"),
            Self::Json => write!(f, "JSON error"),
            Self::LichessAPI => write!(f, "Lichess API error"),
            Self::Reqwest => write!(f, "reqwest error"),
        }
    }
}

/// APIError struct
#[derive(Debug)]
pub(crate) struct LichessAPIError {
    code: StatusCode,
    msg: String,
}

impl LichessAPIError {
    pub(crate) fn new(code: StatusCode, msg: String) -> Self {
        LichessAPIError { code, msg }
    }
}

impl StdError for LichessAPIError {}

impl Display for LichessAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP code {}: {}", self.code, self.msg)
    }
}
