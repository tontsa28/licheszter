use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::fide::FidePlayer,
};

impl Licheszter {
    /// Get information about a FIDE player.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn fide_player(&self, player_id: u32) -> Result<FidePlayer> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/fide/player/{player_id}"));
        let builder = self.client.get(url);

        self.to_model::<FidePlayer>(builder).await
    }

    /// Search for FIDE players. Only player names can be searched for.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn fide_search(&self, query: &str) -> Result<Vec<FidePlayer>> {
        let url = self.req_url(UrlBase::Lichess, "api/fide/player");
        let builder = self.client.get(url).query(&[("q", query)]);

        self.to_model::<Vec<FidePlayer>>(builder).await
    }
}
