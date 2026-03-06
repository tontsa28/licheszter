use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::fide::FidePlayer,
};

/// A struct for accessing the FIDE API endpoints.
pub struct FideApi<'a> {
    pub(crate) client: &'a Licheszter,
}

impl FideApi<'_> {
    /// Get information about a FIDE player.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn player(&self, player_id: u32) -> Result<FidePlayer> {
        let url = self
            .client
            .req_url(UrlBase::Lichess, &format!("api/fide/player/{player_id}"));
        let builder = self.client.client.get(url);

        self.client.to_model::<FidePlayer>(builder).await
    }

    /// Search for FIDE players. Only player names can be searched for.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn search(&self, query: &str) -> Result<Vec<FidePlayer>> {
        let url = self.client.req_url(UrlBase::Lichess, "api/fide/player");
        let builder = self.client.client.get(url).query(&[("q", query)]);

        self.client.to_model::<Vec<FidePlayer>>(builder).await
    }
}
