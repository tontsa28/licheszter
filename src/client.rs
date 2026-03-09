// Re-export the API category structs so they're accessible as `licheszter::client::XxxApi`.
// The `api` module itself is `pub(crate)` to keep it as an internal implementation detail.
#[cfg(feature = "account")]
pub use crate::api::account::AccountApi;
#[cfg(feature = "analysis")]
pub use crate::api::analysis::AnalysisApi;
#[cfg(feature = "board")]
pub use crate::api::board::BoardApi;
#[cfg(feature = "bot")]
pub use crate::api::bot::BotApi;
#[cfg(feature = "challenges")]
pub use crate::api::challenges::ChallengesApi;
#[cfg(feature = "fide")]
pub use crate::api::fide::FideApi;
#[cfg(feature = "games")]
pub use crate::api::games::GamesApi;
#[cfg(feature = "messaging")]
pub use crate::api::messaging::MessagingApi;
#[cfg(feature = "openings")]
pub use crate::api::openings::OpeningsApi;
#[cfg(feature = "pairings")]
pub use crate::api::pairings::BulkPairingsApi;
#[cfg(feature = "puzzles")]
pub use crate::api::puzzles::PuzzlesApi;
#[cfg(feature = "relations")]
pub use crate::api::relations::RelationsApi;
#[cfg(feature = "simuls")]
pub use crate::api::simuls::SimulsApi;
#[cfg(feature = "tablebase")]
pub use crate::api::tablebase::TablebaseApi;
#[cfg(feature = "tv")]
pub use crate::api::tv::TvApi;
#[cfg(feature = "users")]
pub use crate::api::users::UsersApi;
use crate::{
    error::{LichessError, Result},
    models::common::OkResponse,
};

#[cfg(feature = "streaming")]
use futures_util::{stream, Stream, TryStreamExt};

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, IntoUrl, RequestBuilder, Url,
};
use serde::de::DeserializeOwned;
use std::{fmt::Display, sync::Arc};

#[cfg(feature = "streaming")]
use std::{io::Error as StdIoError, pin::Pin};

#[cfg(feature = "streaming")]
use tokio::io::{AsyncBufReadExt, BufReader};

#[cfg(feature = "streaming")]
use tokio_util::io::StreamReader;

// Lichess default URL constants
const BASE_URL: &str = "https://lichess.org";

#[cfg(feature = "openings")]
const OPENINGS_URL: &str = "https://explorer.lichess.org";

#[cfg(feature = "tablebase")]
const TABLEBASE_URL: &str = "https://tablebase.lichess.org";

// Default user agent
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Shared inner state for the Lichess client.
///
/// This struct holds the `reqwest` HTTP client and base URLs.
/// It is wrapped in an [`Arc`] and shared between [`Licheszter`] and all API category structs.
///
/// Note: The `reqwest` [`Client`] itself uses `Arc` internally, so cloning it is very cheap
/// (just incrementing a reference count). The connection pool and configuration are shared
/// across all clones. This means passing the client around via `Arc<LicheszterInner>` adds
/// virtually no overhead.
#[derive(Debug)]
pub(crate) struct LicheszterInner {
    pub(crate) client: Client,
    pub(crate) base_url: Url,
    #[cfg(feature = "openings")]
    pub(crate) openings_url: Url,
    #[cfg(feature = "tablebase")]
    pub(crate) tablebase_url: Url,
}

impl LicheszterInner {
    // Convert the API response into a deserialized model
    pub(crate) async fn to_model<T>(&self, builder: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // Send the request & get the response
        let response = builder.send().await?;

        // Return an error if the request failed
        if !response.status().is_success() {
            return Err(LichessError::from_response(response).await?.into());
        }

        // Deserialize the response data into JSON
        serde_json::from_slice::<T>(&response.bytes().await?).map_err(Into::into)
    }

    // Convert API response into a deserialized stream model
    #[cfg(feature = "streaming")]
    pub(crate) async fn to_stream<T>(
        &self,
        builder: RequestBuilder,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<T>> + Send>>>
    where
        T: DeserializeOwned,
    {
        // Send the request
        let response = builder.send().await?;

        // Return an error if the request failed
        if !response.status().is_success() {
            return Err(LichessError::from_response(response).await?.into());
        }

        // Get the byte stream returned by the response
        let byte_stream = response.bytes_stream();

        // Create a reader over the lines
        let reader = BufReader::new(StreamReader::new(byte_stream.map_err(StdIoError::other)));
        let lines = reader.lines();

        // Create the stream
        let stream = stream::unfold(lines, |mut lines| async {
            loop {
                match lines.next_line().await {
                    Ok(Some(line)) => {
                        // If the line is empty, just skip it
                        if line.is_empty() {
                            continue;
                        }

                        // Deserialize the line and return it
                        let parsed = serde_json::from_str::<T>(&line).map_err(Into::into);
                        return Some((parsed, lines));
                    }
                    Ok(None) => return None,
                    Err(e) => return Some((Err(e.into()), lines)),
                }
            }
        });

        Ok(Box::pin(stream))
    }

    // Convert the API response into a string
    #[cfg(any(feature = "games", feature = "openings"))]
    pub(crate) async fn to_string(&self, builder: RequestBuilder) -> Result<String> {
        // Send the request & get the response
        let response = builder.send().await?;

        // Return an error if the request failed
        if !response.status().is_success() {
            return Err(LichessError::from_response(response).await?.into());
        }

        Ok(response.text().await?)
    }

    // Execute a request that returns an OkResponse and discard the response body
    pub(crate) async fn execute(&self, builder: RequestBuilder) -> Result<()> {
        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    // Construct the full URL of a request with given path
    pub(crate) fn req_url(&self, url: UrlBase, path: &str) -> Url {
        let mut base = match url {
            UrlBase::Lichess => self.base_url.clone(),
            #[cfg(feature = "openings")]
            UrlBase::Openings => self.openings_url.clone(),
            #[cfg(feature = "tablebase")]
            UrlBase::Tablebase => self.tablebase_url.clone(),
        };
        base.set_path(path);
        base
    }
}

/// [`Licheszter`] is used to connect to the Lichess API.
///
/// API endpoints are organized into categories, each accessible as a field
/// through its corresponding accessor method (e.g., `client.account()`, `client.challenges()`).
/// Each API category struct shares the underlying HTTP client and configuration
/// via `Arc`, so accessing them is very cheap.
#[derive(Debug)]
pub struct Licheszter {
    pub(crate) inner: Arc<LicheszterInner>,
    #[cfg(feature = "account")]
    account: AccountApi,
    #[cfg(feature = "analysis")]
    analysis: AnalysisApi,
    #[cfg(feature = "board")]
    board: BoardApi,
    #[cfg(feature = "bot")]
    bot: BotApi,
    #[cfg(feature = "challenges")]
    challenges: ChallengesApi,
    #[cfg(feature = "fide")]
    fide: FideApi,
    #[cfg(feature = "games")]
    games: GamesApi,
    #[cfg(feature = "messaging")]
    messaging: MessagingApi,
    #[cfg(feature = "openings")]
    openings: OpeningsApi,
    #[cfg(feature = "pairings")]
    pairings: BulkPairingsApi,
    #[cfg(feature = "puzzles")]
    puzzles: PuzzlesApi,
    #[cfg(feature = "relations")]
    relations: RelationsApi,
    #[cfg(feature = "simuls")]
    simuls: SimulsApi,
    #[cfg(feature = "tablebase")]
    tablebase: TablebaseApi,
    #[cfg(feature = "tv")]
    tv: TvApi,
    #[cfg(feature = "users")]
    users: UsersApi,
}

impl Licheszter {
    /// Constructs a new [`Licheszter`].
    ///
    /// Use [`Licheszter::builder()`] instead if you want to configure the [`Licheszter`] instance.
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

    /// Get the base URL used in this [`Licheszter`] client.
    #[must_use]
    pub fn base_url(&self) -> Url {
        self.inner.base_url.clone()
    }

    /// Get the `reqwest::Client` behind this [`Licheszter`] instance.
    ///
    /// Note: `reqwest::Client` uses `Arc` internally, so this clone is very cheap.
    /// The returned client shares the same connection pool and configuration.
    #[must_use]
    pub fn client(&self) -> Client {
        self.inner.client.clone()
    }

    /// Get the opening explorer server URL used in this [`Licheszter`] client.
    #[cfg(feature = "openings")]
    #[must_use]
    pub fn openings_url(&self) -> Url {
        self.inner.openings_url.clone()
    }

    /// Get the tablebase server URL used in this [`Licheszter`] client.
    #[cfg(feature = "tablebase")]
    #[must_use]
    pub fn tablebase_url(&self) -> Url {
        self.inner.tablebase_url.clone()
    }

    /// Access the Account API endpoints.
    #[cfg(feature = "account")]
    #[must_use]
    pub fn account(&self) -> &AccountApi {
        &self.account
    }

    /// Access the Challenges API endpoints.
    #[cfg(feature = "challenges")]
    #[must_use]
    pub fn challenges(&self) -> &ChallengesApi {
        &self.challenges
    }

    /// Access the Users API endpoints.
    #[cfg(feature = "users")]
    #[must_use]
    pub fn users(&self) -> &UsersApi {
        &self.users
    }

    /// Access the Games API endpoints.
    #[cfg(feature = "games")]
    #[must_use]
    pub fn games(&self) -> &GamesApi {
        &self.games
    }

    /// Access the Puzzles API endpoints.
    #[cfg(feature = "puzzles")]
    #[must_use]
    pub fn puzzles(&self) -> &PuzzlesApi {
        &self.puzzles
    }

    /// Access the Bot API endpoints.
    #[cfg(feature = "bot")]
    #[must_use]
    pub fn bot(&self) -> &BotApi {
        &self.bot
    }

    /// Access the Board API endpoints.
    #[cfg(feature = "board")]
    #[must_use]
    pub fn board(&self) -> &BoardApi {
        &self.board
    }

    /// Access the Relations API endpoints.
    #[cfg(feature = "relations")]
    #[must_use]
    pub fn relations(&self) -> &RelationsApi {
        &self.relations
    }

    /// Access the TV API endpoints.
    #[cfg(feature = "tv")]
    #[must_use]
    pub fn tv(&self) -> &TvApi {
        &self.tv
    }

    /// Access the Messaging API endpoints.
    #[cfg(feature = "messaging")]
    #[must_use]
    pub fn messaging(&self) -> &MessagingApi {
        &self.messaging
    }

    /// Access the Bulk Pairings API endpoints.
    #[cfg(feature = "pairings")]
    #[must_use]
    pub fn bulk_pairings(&self) -> &BulkPairingsApi {
        &self.pairings
    }

    /// Access the Simuls API endpoints.
    #[cfg(feature = "simuls")]
    #[must_use]
    pub fn simuls(&self) -> &SimulsApi {
        &self.simuls
    }

    /// Access the FIDE API endpoints.
    #[cfg(feature = "fide")]
    #[must_use]
    pub fn fide(&self) -> &FideApi {
        &self.fide
    }

    /// Access the Analysis API endpoints.
    #[cfg(feature = "analysis")]
    #[must_use]
    pub fn analysis(&self) -> &AnalysisApi {
        &self.analysis
    }

    /// Access the Openings API endpoints.
    #[cfg(feature = "openings")]
    #[must_use]
    pub fn openings(&self) -> &OpeningsApi {
        &self.openings
    }

    /// Access the Tablebase API endpoints.
    #[cfg(feature = "tablebase")]
    #[must_use]
    pub fn tablebase(&self) -> &TablebaseApi {
        &self.tablebase
    }
}

impl Default for Licheszter {
    /// Create an unauthenticated instance of Licheszter.
    fn default() -> Self {
        Self::new()
    }
}

/// A [`LicheszterBuilder`] can be used to create a new instance of [`Licheszter`] with custom configuration.
#[derive(Debug, Clone)]
pub struct LicheszterBuilder {
    client: Client,
    base_url: Url,
    #[cfg(feature = "openings")]
    openings_url: Url,
    #[cfg(feature = "tablebase")]
    tablebase_url: Url,
}

impl LicheszterBuilder {
    /// Constructs a new [`LicheszterBuilder`].
    ///
    /// This is the same as [`Licheszter::builder()`](fn@Licheszter::builder).
    #[must_use]
    pub fn new() -> LicheszterBuilder {
        LicheszterBuilder::default()
    }

    /// Returns a [`Licheszter`](struct@Licheszter) that uses this [`LicheszterBuilder`] configuration.
    #[must_use]
    pub fn build(self) -> Licheszter {
        let inner = Arc::new(LicheszterInner {
            client: self.client,
            base_url: self.base_url,
            #[cfg(feature = "openings")]
            openings_url: self.openings_url,
            #[cfg(feature = "tablebase")]
            tablebase_url: self.tablebase_url,
        });

        Licheszter {
            #[cfg(feature = "account")]
            account: AccountApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "analysis")]
            analysis: AnalysisApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "board")]
            board: BoardApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "bot")]
            bot: BotApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "challenges")]
            challenges: ChallengesApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "fide")]
            fide: FideApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "games")]
            games: GamesApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "messaging")]
            messaging: MessagingApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "openings")]
            openings: OpeningsApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "pairings")]
            pairings: BulkPairingsApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "puzzles")]
            puzzles: PuzzlesApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "relations")]
            relations: RelationsApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "simuls")]
            simuls: SimulsApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "tablebase")]
            tablebase: TablebaseApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "tv")]
            tv: TvApi {
                inner: Arc::clone(&inner),
            },
            #[cfg(feature = "users")]
            users: UsersApi {
                inner: Arc::clone(&inner),
            },
            inner,
        }
    }

    /// Use authentication to gain full access to the Lichess API.
    /// This is recommended for most use cases.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The authentication token contains invalid characters (non-visible ASCII, newlines, etc.)
    /// - The HTTP client fails to initialize (extremely rare)
    pub fn with_authentication<S>(mut self, token: S) -> Result<LicheszterBuilder>
    where
        S: AsRef<str> + Display,
    {
        // Create a new header map & the authentication header
        let mut header_map = HeaderMap::new();
        let token = format!("Bearer {token}");

        // Validate the token and create header (returns error instead of panicking)
        let mut auth_header = HeaderValue::from_str(&token)?;

        // Insert the authentication header into the header map
        auth_header.set_sensitive(true);
        header_map.insert(header::AUTHORIZATION, auth_header);

        self.client = Client::builder()
            .default_headers(header_map)
            .user_agent(USER_AGENT)
            .tls_backend_rustls()
            .build()?;
        Ok(self)
    }

    /// Insert a valid base URL of a custom Lichess server.
    /// This can be useful, for example, when hosting your own server for debugging purposes.
    ///
    /// # Errors
    /// Returns an error if the given URL cannot be converted into a [`reqwest::Url`].
    pub fn with_base_url(mut self, url: impl IntoUrl) -> Result<LicheszterBuilder> {
        self.base_url = url.into_url()?;
        Ok(self)
    }

    /// Insert a valid URL of a custom opening explorer server.
    /// This can be useful, for example, when hosting your own server for debugging purposes.
    ///
    /// # Errors
    /// Returns an error if the given URL cannot be converted into a [`reqwest::Url`].
    #[cfg(feature = "openings")]
    pub fn with_openings_url(mut self, url: impl IntoUrl) -> Result<LicheszterBuilder> {
        self.openings_url = url.into_url()?;
        Ok(self)
    }

    /// Insert a valid URL of a custom endgame tablebase server.
    /// This can be useful, for example, when hosting your own server for debugging purposes.
    ///
    /// # Errors
    /// Returns an error if the given URL cannot be converted into a [`reqwest::Url`].
    #[cfg(feature = "tablebase")]
    pub fn with_tablebase_url(mut self, url: impl IntoUrl) -> Result<LicheszterBuilder> {
        self.tablebase_url = url.into_url()?;
        Ok(self)
    }
}

impl Default for LicheszterBuilder {
    /// Create an unauthenticated instance of Licheszter.
    fn default() -> Self {
        Self {
            client: Client::builder()
                .user_agent(USER_AGENT)
                .tls_backend_rustls()
                .build()
                .expect("Failed to build HTTP client - this should never fail with default configuration"),
            base_url: Url::parse(BASE_URL).expect("BASE_URL constant is not a valid URL"),
            #[cfg(feature = "openings")]
            openings_url: Url::parse(OPENINGS_URL).expect("OPENINGS_URL constant is not a valid URL"),
            #[cfg(feature = "tablebase")]
            tablebase_url: Url::parse(TABLEBASE_URL).expect("TABLEBASE_URL constant is not a valid URL"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum UrlBase {
    Lichess,
    #[cfg(feature = "openings")]
    Openings,
    #[cfg(feature = "tablebase")]
    Tablebase,
}
