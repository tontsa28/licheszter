use reqwest::{Response, StatusCode};
use serde_json::Value;
use std::{error::Error as StdError, fmt::Display, result::Result as StdResult};

/// A shorthand for the actual result type.
pub type Result<T> = StdResult<T, Error>;

/// A general, library-wide error type that will be returned in case of any error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Box<dyn StdError + Send + Sync + 'static>,
}

impl Error {
    // Creates a new instance of `Error`.
    pub(crate) fn new<E>(kind: ErrorKind, source: E) -> Self
    where
        E: Into<Box<dyn StdError + Send + Sync + 'static>>,
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
        matches!(self.kind, ErrorKind::Lichess)
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

    /// Returns true if the error is a [`url-encoded` error](enum@comma_serde_urlencoded::ser::Error).
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

impl From<LichessError> for Error {
    fn from(source: LichessError) -> Self {
        Error::new(ErrorKind::Lichess, source)
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
    Lichess,
    Reqwest,
    Json,
    UrlEncoded,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO => write!(f, "IO error"),
            Self::Json => write!(f, "JSON error"),
            Self::Lichess => write!(f, "Lichess API error"),
            Self::Reqwest => write!(f, "reqwest error"),
            Self::UrlEncoded => write!(f, "url-encoded error"),
        }
    }
}

// An error produced by the Lichess API
#[derive(Debug)]
pub(crate) struct LichessError {
    status: StatusCode,
    message: String,
}

impl LichessError {
    pub(crate) async fn from_response(response: Response) -> Result<Self> {
        let status = response.status();
        let error = serde_json::from_slice::<Value>(&response.bytes().await?);

        // Return a simple "not found" message if the response is a 404 HTML page
        let message = if status == StatusCode::NOT_FOUND && error.is_err() {
            String::from("Not found")
        } else {
            let mut msg = error?
                .get("error")
                .unwrap_or(&Value::String(
                    "Unexpected error format, failed to parse the actual error message".to_string(),
                ))
                .to_string();
            let removable_chars = ['{', '}', '[', ']', '"'];
            msg.retain(|c| !removable_chars.contains(&c));
            msg.replace(':', ": ")
        };

        Ok(LichessError { status, message })
    }
}

impl StdError for LichessError {}

impl Display for LichessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP code {}: {}", self.status, self.message)
    }
}
