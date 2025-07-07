use std::pin::Pin;

use futures_util::Stream;

use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::tv::{TvGameEvent, TvGames},
};

impl Licheszter {
    /// Get basic information about the TV games for each speed and variant, including computer and bot games.
    pub async fn tv_games(&self) -> Result<TvGames> {
        let url = self.req_url(UrlBase::Lichess, "api/tv/channels");
        let builder = self.client.get(url);

        self.into::<TvGames>(builder).await
    }

    /// Stream positions and moves of the current TV game.
    pub async fn tv_game_connect(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<TvGameEvent>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, "api/tv/feed");
        let builder = self.client.get(url);

        self.into_stream::<TvGameEvent>(builder).await
    }
}
