use reqwest::{Response, StatusCode};
use serde_json::Value;
use std::{error::Error as StdError, fmt::Display, result::Result as StdResult};

/// A shorthand for the actual result type.
pub type Result<T> = StdResult<T, Error>;

// Simple string error for internal use
#[derive(Debug)]
pub(crate) struct StringError(pub(crate) String);

impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for StringError {}

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

    /// Returns true if the error is caused by an invalid authentication token.
    #[must_use]
    pub fn is_invalid_auth_token(&self) -> bool {
        matches!(self.kind, ErrorKind::InvalidAuthToken)
    }

    /// Returns true if the error is caused by HTTP client build failure.
    #[must_use]
    pub fn is_client_build(&self) -> bool {
        matches!(self.kind, ErrorKind::ClientBuild)
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
    InvalidAuthToken,
    ClientBuild,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO => write!(f, "IO error"),
            Self::Json => write!(f, "JSON error"),
            Self::Lichess => write!(f, "Lichess API error"),
            Self::Reqwest => write!(f, "reqwest error"),
            Self::UrlEncoded => write!(f, "url-encoded error"),
            Self::InvalidAuthToken => write!(f, "invalid authentication token"),
            Self::ClientBuild => write!(f, "HTTP client build error"),
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

        // Design decision: Return a simple "Not found" message for 404s with unparseable bodies.
        // Lichess often returns long HTML pages instead of JSON for 404 errors, even on API endpoints.
        // These HTML responses don't contain any actionable information, just generic error pages.
        // Returning "Not found" provides a cleaner, more consistent error message than showing raw HTML.
        let message = if status == StatusCode::NOT_FOUND && error.is_err() {
            String::from("Not found")
        } else {
            let error_json = error?;
            let error_msg = error_json
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unexpected error format, failed to parse the actual error message");

            let mut msg = error_msg.to_string();
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
