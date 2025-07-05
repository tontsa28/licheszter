use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::tv::TvGames,
};

impl Licheszter {
    /// Get basic information about the TV games for each speed and variant, including computer and bot games.
    pub async fn tv_current_games(&self) -> Result<TvGames> {
        let url = self.req_url(UrlBase::Lichess, "api/tv/channels");
        let builder = self.client.get(url);

        self.into::<TvGames>(builder).await
    }
}
