use std::io::{Error, ErrorKind};
use futures_util::{Stream, StreamExt, TryStreamExt};
use reqwest::{header, Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde_json::from_str;
use tokio::io::AsyncBufReadExt;
use tokio_stream::wrappers::LinesStream;
use tokio_util::io::StreamReader;
use crate::error::LicheszterError;

/// LicheszterResult type
pub type LicheszterResult<T> = Result<T, LicheszterError>;

/// Licheszter struct
#[derive(Debug)]
pub struct Licheszter {
    pub(crate) client: Client,
    pub(crate) base: String
}

// Implement necessary functions for the struct
impl Licheszter {
    /// Create an unauthenticated instance of Licheszter
    pub fn default() -> Licheszter {
        Licheszter {
            client: Client::new(),
            base: String::from("https://lichess.org")
        }
    }

    /// Create an authenticated instance of Licheszter
    pub fn new(pat: String) -> Licheszter {
        let mut header_map = header::HeaderMap::new();
        let mut auth_header = header::HeaderValue::from_str(&format!("Bearer {}", pat)).unwrap();

        auth_header.set_sensitive(true);
        header_map.insert(header::AUTHORIZATION, auth_header);

        Licheszter {
            client: Client::builder()
                .default_headers(header_map)
                .build()
                .unwrap(),
            base: String::from("https://lichess.org")
        }
    }

    /// Create a request to the Lichess API
    async fn api_call(&self, builder: RequestBuilder) -> LicheszterResult<Response> {
        let response = builder.send().await.map_err(LicheszterError::from)?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(LicheszterError::from_response(response).await)
        }
    }

    /// Convert API response into a raw string
    #[allow(dead_code)]
    pub(crate) async fn to_raw_str(&self, builder: RequestBuilder) -> LicheszterResult<String> {
        self.api_call(builder).await?.text().await.map_err(Into::into)
    }

    /// Convert API response into raw bytes
    #[allow(dead_code)]
    pub(crate) async fn to_raw_bytes(&self, builder: RequestBuilder) -> LicheszterResult<impl Stream<Item = LicheszterResult<bytes::Bytes>>> {
        Ok(self.api_call(builder).await?.bytes_stream().map_err(Into::into))
    }

    /// Convert API response into a full model
    pub(crate) async fn to_model_full<T: DeserializeOwned>(&self, builder: RequestBuilder) -> LicheszterResult<T> {
        from_str(&self.api_call(builder).await?.text().await?).map_err(Into::into)
    }

    /// Convert API response into a stream model
    pub(crate) async fn to_model_stream<T: DeserializeOwned>(&self, builder: RequestBuilder) -> LicheszterResult<impl Stream<Item = LicheszterResult<T>>> {
        let stream = self.api_call(builder).await?.bytes_stream().map_err(|err| Error::new(ErrorKind::Other, err));

        Ok(Box::pin(
            LinesStream::new(StreamReader::new(stream).lines()).filter_map(|line| async move {
                let line = line.ok()?;
                if !line.is_empty() {
                    Some(from_str(&line).map_err(Into::into))
                } else {
                    None
                }
            })
        ))
    }
}