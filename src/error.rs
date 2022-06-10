use reqwest::{header, Response, StatusCode};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LicheszterError {
    RateLimit(Option<usize>),
    Request(#[from] reqwest::Error),
    StatusCode(u16, String),
    API(#[from] APIError),
    ParseJSON(#[from] serde_json::Error),
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

#[derive(Debug, Error, Deserialize)]
pub struct APIError {
    error: String
}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for LicheszterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}