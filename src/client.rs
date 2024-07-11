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

// Lichess default URL constants
const BASE_URL: &str = "https://lichess.org";
const EXPLORER_URL: &str = "https://explorer.lichess.ovh";
const TABLEBASE_URL: &str = "https://tablebase.lichess.ovh";

/// [`Licheszter`] is used to connect to the Lichess API.
#[derive(Debug)]
pub struct Licheszter {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
    pub(crate) explorer_url: Url,
    pub(crate) tablebase_url: Url,
}

impl Licheszter {
    /// Constructs a new `Licheszter`.
    ///
    /// Use `Licheszter::builder()` instead if you want to configure the `Licheszter` instance.
    #[must_use]
    pub fn new() -> Licheszter {
        LicheszterBuilder::new().build()
    }

    /// Creates a [`LicheszterBuilder`](struct@LicheszterBuilder) to configure a [`Licheszter`].
    ///
    /// This is the same as [`LicheszterBuilder::new()`](fn@LicheszterBuilder::new).
    #[must_use]
    pub fn builder() -> LicheszterBuilder {
        LicheszterBuilder::default()
    }

    /// Get the base URL used in this instance of `Licheszter`.
    #[must_use]
    pub fn base_url(&self) -> Url {
        self.base_url.clone()
    }

    /// Get the `reqwest` Client behind this instance of `Licheszter`.
    #[must_use]
    pub fn client(&self) -> Client {
        self.client.clone()
    }

    /// Get the opening explorer server URL used in this instance of `Licheszter`.
    #[must_use]
    pub fn explorer_url(&self) -> Url {
        self.explorer_url.clone()
    }

    /// Get the tablebase server URL used in this instance of `Licheszter`.
    #[must_use]
    pub fn tablebase_url(&self) -> Url {
        self.tablebase_url.clone()
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
    pub(crate) async fn to_model_stream<'de, T>(
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
        let lines = reader.filter_map(|line| async {
            let line = line.ok()?;

            // Return the stream event as a ping if it's empty
            if line.is_empty() {
                return None;
            }

            Some(serde_json::from_str::<T>(&line).map_err(Into::into))
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
    explorer_url: Url,
    tablebase_url: Url,
}

impl LicheszterBuilder {
    /// Constructs a new `LicheszterBuilder`.
    ///
    /// This is the same as [`Licheszter::builder()`](fn@Licheszter::builder).
    #[must_use]
    pub fn new() -> LicheszterBuilder {
        LicheszterBuilder::default()
    }

    /// Returns a [`Licheszter`](struct@Licheszter) that uses this [`LicheszterBuilder`] configuration.
    #[must_use]
    pub fn build(self) -> Licheszter {
        Licheszter {
            client: self.client,
            base_url: self.base_url,
            explorer_url: self.explorer_url,
            tablebase_url: self.tablebase_url,
        }
    }

    /// Use authentication to gain full access to the Lichess API.
    /// This is recommended for most use cases.
    #[must_use]
    pub fn with_authentication<S>(mut self, token: S) -> LicheszterBuilder
    where
        S: AsRef<str> + Display,
    {
        // Create a new header map & the authentication header
        let mut header_map = HeaderMap::new();
        let token = format!("Bearer {token}");
        let mut auth_header = HeaderValue::from_str(&token)
            .expect("Authentication token should only contain visible ASCII characters");

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
    pub fn with_base_url(mut self, url: impl IntoUrl) -> Result<LicheszterBuilder> {
        self.base_url = url.into_url()?;
        Ok(self)
    }

    pub fn with_explorer_url(mut self, url: impl IntoUrl) -> Result<LicheszterBuilder> {
        self.explorer_url = url.into_url()?;
        Ok(self)
    }

    pub fn with_tablebase_url(mut self, url: impl IntoUrl) -> Result<LicheszterBuilder> {
        self.tablebase_url = url.into_url()?;
        Ok(self)
    }
}

impl Default for LicheszterBuilder {
    /// Create an unauthenticated instance of Licheszter.
    fn default() -> Self {
        Self {
            client: Client::builder().use_rustls_tls().build().unwrap(),
            base_url: Url::parse(BASE_URL).unwrap(),
            explorer_url: Url::parse(EXPLORER_URL).unwrap(),
            tablebase_url: Url::parse(TABLEBASE_URL).unwrap(),
        }
    }
}
