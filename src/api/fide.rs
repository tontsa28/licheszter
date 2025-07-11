use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::fide::FidePlayer,
};

impl Licheszter {
    /// Get information about a FIDE player.
    pub async fn fide_player(&self, player_id: u32) -> Result<FidePlayer> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/fide/player/{player_id}"));
        let builder = self.client.get(url);

        self.into::<FidePlayer>(builder).await
    }
}
