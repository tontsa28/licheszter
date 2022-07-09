use reqwest::{header, Response, StatusCode};
use serde::Deserialize;
use thiserror::Error;

/// LicheszterError enum
#[derive(Debug, Error)]
pub enum LicheszterError {
    #[error("Exceeded request limit")]
    RateLimit(Option<usize>),
    #[error("Request error: {}", 0)]
    Request(#[from] reqwest::Error),
    #[error("Status code {}: {}", 0, 1)]
    StatusCode(u16, String),
    #[error("API error: {}", 0)]
    API(#[from] APIError),
    #[error("JSON parse error: {}", 0)]
    ParseJSON(#[from] serde_json::Error),
    #[error("IO error: {}", 0)]
    IO(#[from] std::io::Error)
}

impl LicheszterError {
    pub(crate) async fn from_response(response: Response) -> Self {
        match response.status() {
            StatusCode::TOO_MANY_REQUESTS => Self::RateLimit(
                response
                    .headers()
                    .get(header::RETRY_AFTER)
                    .and_then(|header| header.to_str().ok())
                    .and_then(|duration| duration.parse().ok())
            ),
            status => response
                .json::<APIError>()
                .await
                .map(Into::into)
                .unwrap_or_else(|_| status.into())
        }
    }
}

impl From<StatusCode> for LicheszterError {
    fn from(c: StatusCode) -> Self {
        Self::StatusCode(
            c.as_u16(),
            c.canonical_reason().unwrap_or("Unknown").to_string()
        )
    }
}

/// APIError struct
#[derive(Debug, Error, Deserialize)]
#[error("Error: {}", error)]
pub struct APIError {
    error: String
}