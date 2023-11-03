use crate::error::{LichessAPIError, Result};
use futures_util::{Stream, StreamExt, TryStreamExt};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, IntoUrl, RequestBuilder, Url,
};
use serde::de::DeserializeOwned;
use std::{
    fmt::Display,
    io::{Error as StdIoError, ErrorKind as StdIoErrorKind},
};
use tokio::io::AsyncBufReadExt;
use tokio_stream::wrappers::LinesStream;
use tokio_util::io::StreamReader;

// Data stream ping JSON & Lichess default URL constants
const PING: &str = "{\"type\":\"ping\"}";
const BASE_URL: &str = "https://lichess.org";

/// [`Licheszter`] is used to connect to the Lichess API.
#[derive(Debug)]
pub struct Licheszter {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
}

impl Licheszter {
    /// Constructs a new `Licheszter`.
    ///
    /// Use `Licheszter::builder()` instead if you want to configure the `Licheszter` instance.
    pub fn new() -> Licheszter {
        LicheszterBuilder::new().build()
    }

    /// Creates a [`LicheszterBuilder`](struct@LicheszterBuilder) to configure a [`Licheszter`].
    ///
    /// This is the same as [`LicheszterBuilder::new()`](fn@LicheszterBuilder::new()).
    pub fn builder() -> LicheszterBuilder {
        LicheszterBuilder::default()
    }

    /// Get the base URL used in this instance of `Licheszter`.
    pub fn base_url(&self) -> Url {
        self.base_url.to_owned()
    }

    /// Get the `reqwest` Client behind this instance of `Licheszter`.
    pub fn client(&self) -> Client {
        self.client.to_owned()
    }

    // Convert the API response into a deserialized model
    pub(crate) async fn to_model<T>(&self, builder: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // Send the request & get the response
        let response = builder.send().await?;

        // Return an error if the request failed
        if !response.status().is_success() {
            return Err(LichessAPIError::from_response(response).await?.into());
        }

        // Deserialize the response data into JSON
        serde_json::from_slice::<T>(&response.bytes().await?).map_err(Into::into)
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
        let response = builder.send().await?;

        // Return an error if the request failed
        if !response.status().is_success() {
            return Err(LichessAPIError::from_response(response).await?.into());
        }

        // Get the byte stream returned by the response
        let stream = response
            .bytes_stream()
            .map_err(|err| StdIoError::new(StdIoErrorKind::Other, err));

        // Create a reader over the lines
        let reader = LinesStream::new(StreamReader::new(stream).lines());

        // Map the lines depending on their contents
        let lines = reader.map(|line| {
            let line = line?;

            // Return the stream event as a ping if it's empty
            if line.is_empty() {
                return serde_json::from_str::<T>(PING).map_err(Into::into);
            }

            serde_json::from_str::<T>(&line).map_err(Into::into)
        });

        Ok(Box::pin(lines))
    }
}

impl Default for Licheszter {
    /// Create an unauthenticated instance of Licheszter.
    fn default() -> Self {
        Self::new()
    }
}

/// A [`LicheszterBuilder`] can be used to create a new instance of [`Licheszter`] with custom configuration.
#[derive(Debug)]
pub struct LicheszterBuilder {
    client: Client,
    base_url: Url,
}

impl LicheszterBuilder {
    /// Constructs a new `LicheszterBuilder`.
    ///
    /// This is the same as [`Licheszter::builder()`](fn@Licheszter::builder()).
    pub fn new() -> LicheszterBuilder {
        LicheszterBuilder::default()
    }

    /// Returns a [`Licheszter`](struct@Licheszter) that uses this [`LicheszterBuilder`] configuration.
    pub fn build(self) -> Licheszter {
        Licheszter {
            client: self.client,
            base_url: self.base_url,
        }
    }

    /// Use authentication to gain full access to the Lichess API.
    /// This is recommended for most use cases.
    pub fn with_authentication<S>(mut self, token: S) -> LicheszterBuilder
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

        self.client = Client::builder()
            .default_headers(header_map)
            .use_rustls_tls()
            .build()
            .unwrap();
        self
    }

    /// Insert a valid base URL of a custom Lichess server.
    /// This can be useful, for example, when hosting your own server for debugging purposes.
    ///
    /// # Errors:
    /// If the given URL cannot be converted into a [`url::Url`], a [`url::ParseError`] will be returned.
    pub fn with_custom_server(mut self, url: impl IntoUrl) -> Result<LicheszterBuilder> {
        self.base_url = url.into_url()?;
        Ok(self)
    }
}

impl Default for LicheszterBuilder {
    /// Create an unauthenticated instance of Licheszter.
    fn default() -> Self {
        Self {
            client: Client::builder().use_rustls_tls().build().unwrap(),
            base_url: Url::parse(BASE_URL).unwrap(),
        }
    }
}
