use std::pin::Pin;

use futures_util::Stream;
use reqwest::header;

use crate::{
    client::{LicheszterInner, UrlBase},
    config::{games::GameOptions, pairings::BulkPairingOptions},
    error::Result,
    models::{
        game::Game,
        pairings::{BulkPairing, BulkPairings},
    },
};

use std::sync::Arc;

/// A struct for accessing the Bulk Pairings API endpoints.
#[derive(Debug)]
pub struct BulkPairingsApi {
    pub(crate) inner: Arc<LicheszterInner>,
}

impl BulkPairingsApi {
    /// Get a list of bulk pairings you created.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn list(&self) -> Result<Vec<BulkPairing>> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/bulk-pairing");
        let builder = self.inner.client.get(url);

        Ok(self.inner.to_model::<BulkPairings>(builder).await?.bulks)
    }

    /// Schedule many games at once, up to 24 hours in advance.
    /// Authentication tokens are required for all paired players.
    /// You can schedule up to 500 games every 10 minutes.
    /// If games have a real-time clock, each player must have only one pairing.
    /// For correspondence games, players can have multiple pairings within the same bulk.
    /// The pairing must contain time control and player information.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn create(&self, options: &BulkPairingOptions) -> Result<BulkPairing> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/bulk-pairing");
        let mut builder = self.inner.client.post(url);

        // Add the options to the request
        let encoded = comma_serde_urlencoded::to_string(options)?;
        builder = builder.body(encoded).header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");

        self.inner.to_model::<BulkPairing>(builder).await
    }

    /// Immediately start all clocks of the games of a bulk pairing.
    /// This overrides the clock start setting of an existing pairing.
    /// If the games have not yet been created or the clocks have already started, this method does nothing.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn clocks_start(&self, bulk_id: &str) -> Result<()> {
        let url = self.inner.req_url(UrlBase::Lichess, &format!("api/bulk-pairing/{bulk_id}/start-clocks"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }

    /// Get a single bulk pairing by its ID.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn show(&self, bulk_id: &str) -> Result<BulkPairing> {
        let url = self.inner.req_url(UrlBase::Lichess, &format!("api/bulk-pairing/{bulk_id}"));
        let builder = self.inner.client.get(url);

        self.inner.to_model::<BulkPairing>(builder).await
    }

    /// Cancel and delete a bulk pairing that is scheduled in the future.
    /// If the games have already been created, this method does nothing.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn cancel(&self, bulk_id: &str) -> Result<()> {
        let url = self.inner.req_url(UrlBase::Lichess, &format!("api/bulk-pairing/{bulk_id}"));
        let builder = self.inner.client.delete(url);

        self.inner.execute(builder).await
    }

    /// Download games of a bulk pairing.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn export(&self, bulk_id: &str, options: Option<&GameOptions>) -> Result<Pin<Box<dyn Stream<Item = Result<Game>> + Send>>> {
        let mut url = self.inner.req_url(UrlBase::Lichess, &format!("api/bulk-pairing/{bulk_id}/games"));

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.inner.client.get(url).header(header::ACCEPT, "application/x-ndjson");
        self.inner.to_stream::<Game>(builder).await
    }
}
