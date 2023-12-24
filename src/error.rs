use reqwest::{Response, StatusCode};
use serde::Deserialize;
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

    /// Returns true if the error is an [`IO` error](struct@std::io::Error).
    #[must_use]
    pub fn is_io(&self) -> bool {
        matches!(self.kind, ErrorKind::IO)
    }

    /// Returns true if the error is produced by the Lichess API.
    #[must_use]
    pub fn is_lichess(&self) -> bool {
        matches!(self.kind, ErrorKind::LichessAPI)
    }

    /// Returns true if the error is a [`reqwest` error](struct@reqwest::Error).
    #[must_use]
    pub fn is_reqwest(&self) -> bool {
        matches!(self.kind, ErrorKind::Reqwest)
    }

    /// Returns true if the error is a [`JSON` error](struct@serde_json::Error).
    #[must_use]
    pub fn is_json(&self) -> bool {
        matches!(self.kind, ErrorKind::Json)
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
    fn from(source: std::io::Error) -> Self {
        Error::new(ErrorKind::IO, source)
    }
}

impl From<LichessAPIError> for Error {
    fn from(source: LichessAPIError) -> Self {
        Error::new(ErrorKind::LichessAPI, source)
    }
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::new(ErrorKind::Reqwest, source)
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::new(ErrorKind::Json, source)
    }
}

#[derive(Debug, Clone)]
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

// An error produced by the Lichess API
#[derive(Debug)]
pub(crate) struct LichessAPIError {
    status: StatusCode,
    msg: String,
}

impl LichessAPIError {
    pub(crate) async fn from_response(response: Response) -> Result<Self> {
        let status = response.status();
        let msg =
            serde_json::from_slice::<LichessAPIErrorMessage>(&response.bytes().await?)?.to_string();
        Ok(LichessAPIError { status, msg })
    }
}

impl StdError for LichessAPIError {}

impl Display for LichessAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP code {}: {}", self.status, self.msg)
    }
}

#[derive(Debug, Deserialize)]
struct LichessAPIErrorMessage {
    error: String,
}

impl Display for LichessAPIErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
