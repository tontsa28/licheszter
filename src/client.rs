use crate::error::{LichessAPIError, Result};
use futures_util::{Stream, StreamExt, TryStreamExt};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, RequestBuilder,
};
use serde::de::DeserializeOwned;
use std::{
    fmt::Display,
    io::{Error as StdIoError, ErrorKind as StdIoErrorKind},
};
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
    /// To access the parts of the API that require authentication, create an [`authenticated instance`](fn@Licheszter::new).
    fn default() -> Licheszter {
        Licheszter::new_unauthenticated()
    }
}

impl Licheszter {
    /// Create an authenticated instance of Licheszter.
    pub fn new<S>(token: S) -> Licheszter
    where
        S: AsRef<str> + Display,
    {
        // Create a new header map & the authentication header
        let mut header_map = HeaderMap::new();
        let token = format!("Bearer {token}");
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

    /// Create an unauthenticated instance of Licheszter.
    /// To access the parts of the API that require authentication, create an [`authenticated instance`](fn@Licheszter::new).
    pub fn new_unauthenticated() -> Licheszter {
        Licheszter {
            client: Client::builder().use_rustls_tls().build().unwrap(),
            base: String::from("https://lichess.org"),
        }
    }

    // Convert the API response into a deserialized model
    pub(crate) async fn to_model<T>(&self, builder: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // Send the request
        let request = builder.send().await?;

        // Return an error if the request failed
        if !request.status().is_success() {
            return Err(LichessAPIError::new(request.status(), request.text().await?).into());
        }

        // Deserialize the response data into JSON
        serde_json::from_slice::<T>(&request.bytes().await?).map_err(Into::into)
    }

    // Convert API response into a deserialized stream model
    pub(crate) async fn to_model_stream<T>(
        &self,
        builder: RequestBuilder,
    ) -> Result<impl Stream<Item = Result<T>>>
    where
        T: DeserializeOwned,
    {
        // Send the request
        let request = builder.send().await?;

        // Return an error if the request failed
        if !request.status().is_success() {
            return Err(LichessAPIError::new(request.status(), request.text().await?).into());
        }

        // Get the byte stream returned by the response
        let stream = request
            .bytes_stream()
            .map_err(|err| StdIoError::new(StdIoErrorKind::Other, err));

        Ok(Box::pin(
            LinesStream::new(StreamReader::new(stream).lines()).filter_map(|line| async move {
                let line = line.ok()?;

                // Suppose that the stream event is a ping if it's empty
                if line.is_empty() {
                    let ping = "{{\"type\":\"ping\"}}".to_string();

                    // Return the stream event as a ping
                    return Some(serde_json::from_str::<T>(&ping).map_err(Into::into));
                }

                Some(serde_json::from_str::<T>(&line).map_err(Into::into))
            }),
        ))
    }
}
