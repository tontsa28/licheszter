use reqwest::{Response, StatusCode};
use serde::Deserialize;
use serde_json::Value;
use std::{error::Error as StdError, fmt::Display, result::Result as StdResult};

/// A shorthand for the actual result type.
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

    /// Returns true if the error is a [`url-encoded` error](struct@comma_serde_urlencoded::ser::Error).
    #[must_use]
    pub fn is_urlencoded(&self) -> bool {
        matches!(self.kind, ErrorKind::UrlEncoded)
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

impl From<comma_serde_urlencoded::ser::Error> for Error {
    fn from(source: comma_serde_urlencoded::ser::Error) -> Self {
        Error::new(ErrorKind::UrlEncoded, source)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ErrorKind {
    IO,
    LichessAPI,
    Reqwest,
    Json,
    UrlEncoded,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO => write!(f, "IO error"),
            Self::Json => write!(f, "JSON error"),
            Self::LichessAPI => write!(f, "Lichess API error"),
            Self::Reqwest => write!(f, "reqwest error"),
            Self::UrlEncoded => write!(f, "url-encoded error"),
        }
    }
}

// An error produced by the Lichess API
#[derive(Debug)]
pub(crate) struct LichessAPIError {
    status: StatusCode,
    message: String,
}

impl LichessAPIError {
    pub(crate) async fn from_response(response: Response) -> Result<Self> {
        let status = response.status();
        let error = serde_json::from_slice::<Value>(&response.bytes().await?);

        // Return a simple "not found" message if the response is a 404 HTML page
        let message = if status == StatusCode::NOT_FOUND && error.is_err() {
            String::from("Not found")
        } else {
            error?.to_string()
        };

        Ok(LichessAPIError { status, message })
    }
}

impl StdError for LichessAPIError {}

impl Display for LichessAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP code {}: {}", self.status, self.message)
    }
}

#[derive(Debug)]
enum LichessAPIErrorMessage {
    StringError(String),
    ObjectError(Value),
}

impl<'de> Deserialize<'de> for LichessAPIErrorMessage {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::String(s) => Ok(Self::StringError(s)),
            Value::Object(_) => Ok(Self::ObjectError(value)),
            _ => Err(serde::de::Error::custom("Unexpected error format")),
        }
    }
}

impl Display for LichessAPIErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringError(s) => write!(f, "{s}"),
            Self::ObjectError(value) => write!(f, "{value}"),
        }
    }
}
