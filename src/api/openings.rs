use std::pin::Pin;

use crate::{
    client::{LicheszterInner, UrlBase},
    config::openings::{LichessOpeningsOptions, MastersOpeningsOptions, PlayerOpeningsOptions},
    error::Result,
    models::{
        common::Color,
        openings::{Opening, PlayerOpening},
    },
};

use std::sync::Arc;
use futures_util::Stream;

/// A struct for accessing the Openings API endpoints.
#[derive(Debug)]
pub struct OpeningsApi {
    pub(crate) inner: Arc<LicheszterInner>,
}

impl OpeningsApi {
    /// Lookup positions from the Masters opening database.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn masters(&self, options: Option<&MastersOpeningsOptions>) -> Result<Opening> {
        let mut url = self.inner.req_url(UrlBase::Openings, "masters");

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.inner.client.get(url);
        self.inner.to_model::<Opening>(builder).await
    }

    /// Lookup positions from the Lichess opening database.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn lichess(&self, options: Option<&LichessOpeningsOptions>) -> Result<Opening> {
        let mut url = self.inner.req_url(UrlBase::Openings, "lichess");

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.inner.client.get(url);
        self.inner.to_model::<Opening>(builder).await
    }

    /// Lookup positions from the Player opening database.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn player(
        &self,
        player: &str,
        color: Color,
        options: Option<&PlayerOpeningsOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<PlayerOpening>> + Send>>> {
        let mut url = self.inner.req_url(UrlBase::Openings, "player");
        let encoded = comma_serde_urlencoded::to_string((("player", player), ("color", color)))?;
        url.set_query(Some(&encoded));

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = encoded + "&" + &comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.inner.client.get(url);
        self.inner.to_stream::<PlayerOpening>(builder).await
    }

    /// Get an OTB (over the board) master game in PGN format.
    /// # NOTE:
    /// This method does NOT deserialize the PGN data, it must be manually parsed.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be read.
    pub async fn masters_otb_game(&self, game_id: &str) -> Result<String> {
        let url = self
            .inner
            .req_url(UrlBase::Openings, &format!("masters/pgn/{game_id}"));
        let builder = self.inner.client.get(url);

        self.inner.to_string(builder).await
    }
}
