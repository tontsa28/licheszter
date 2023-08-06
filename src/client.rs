use crate::error::{LichessAPIError, Result};
use futures_util::{Stream, StreamExt, TryStreamExt};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, RequestBuilder, Response,
};
use serde::de::DeserializeOwned;
use serde_json::from_str;
use std::io::{Error as StdIoError, ErrorKind as StdIoErrorKind};
use tokio::io::AsyncBufReadExt;
use tokio_stream::wrappers::LinesStream;
use tokio_util::io::StreamReader;

/// Licheszter enables the connection to the Lichess API.
#[derive(Debug)]
pub struct Licheszter {
    pub(crate) client: Client,
    pub(crate) base: String,
}

impl Default for Licheszter {
    /// Create an unauthenticated instance of Licheszter.
    /// To access the parts of the API that require authentication, create an authenticated instance.
    fn default() -> Licheszter {
        Licheszter {
            client: Client::builder().use_rustls_tls().build().unwrap(),
            base: String::from("https://lichess.org"),
        }
    }
}

impl Licheszter {
    /// Create an authenticated instance of Licheszter.
    pub fn new(pat: String) -> Licheszter {
        // Create a new header map & the authentication header
        let mut header_map = HeaderMap::new();
        let token = format!("Bearer {pat}");
        let mut auth_header = HeaderValue::from_str(&token).unwrap();

        // Insert the authentication header into the header map
        auth_header.set_sensitive(true);
        header_map.insert(header::AUTHORIZATION, auth_header);

        Licheszter {
            client: Client::builder()
                .default_headers(header_map)
                .use_rustls_tls()
                .build()
                .unwrap(),
            base: String::from("https://lichess.org"),
        }
    }

    /// Call the Lichess API.
    async fn api_call(&self, builder: RequestBuilder) -> Result<Response> {
        let request = builder.send().await?;

        // Check if the request succeeded
        if request.status().is_success() {
            Ok(request)
        } else {
            Err(LichessAPIError::new(request.status(), request.text().await?).into())
        }
    }

    /// Convert API response into a full model.
    pub(crate) async fn to_model_full<T: DeserializeOwned>(
        &self,
        builder: RequestBuilder,
    ) -> Result<T> {
        from_str(&self.api_call(builder).await?.text().await?).map_err(Into::into)
    }

    /// Convert API response into a stream model.
    pub(crate) async fn to_model_stream<T: DeserializeOwned>(
        &self,
        builder: RequestBuilder,
    ) -> Result<impl Stream<Item = Result<T>>> {
        let stream = self
            .api_call(builder)
            .await?
            .bytes_stream()
            .map_err(|err| StdIoError::new(StdIoErrorKind::Other, err));

        Ok(Box::pin(
            LinesStream::new(StreamReader::new(stream).lines()).filter_map(|line| async move {
                let line = line.ok()?;
                if !line.is_empty() {
                    Some(from_str(&line).map_err(Into::into))
                } else {
                    let ping = "{{\"type\":\"ping\"}}".to_string();
                    Some(from_str(&ping).map_err(Into::into))
                }
            }),
        ))
    }
}
