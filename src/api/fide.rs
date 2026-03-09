use crate::{
    client::{LicheszterInner, UrlBase},
    error::Result,
    models::fide::FidePlayer,
};

use std::sync::Arc;

/// A struct for accessing the FIDE API endpoints.
#[derive(Debug)]
pub struct FideApi {
    pub(crate) inner: Arc<LicheszterInner>,
}

impl FideApi {
    /// Get information about a FIDE player.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn player(&self, player_id: u32) -> Result<FidePlayer> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/fide/player/{player_id}"));
        let builder = self.inner.client.get(url);

        self.inner.to_model::<FidePlayer>(builder).await
    }

    /// Search for FIDE players. Only player names can be searched for.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn search(&self, query: &str) -> Result<Vec<FidePlayer>> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/fide/player");
        let builder = self.inner.client.get(url).query(&[("q", query)]);

        self.inner.to_model::<Vec<FidePlayer>>(builder).await
    }
}
