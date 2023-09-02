use crate::{
    client::Licheszter,
    error::Result,
    models::{board::Event, game::{UserGame, UserGames}},
};
use futures_util::Stream;

impl Licheszter {
    /// Stream incoming events.
    pub async fn stream_events(&self) -> Result<impl Stream<Item = Result<Event>>> {
        let url = format!("{}/api/stream/event", self.base);
        let builder = self.client.get(&url);

        self.to_model_stream::<Event>(builder).await
    }

    /// Get ongoing games of the current user.
    pub async fn get_ongoing_games(&self, nb_games: u8) -> Result<Vec<UserGame>> {
        let url = format!("{}/api/account/playing", self.base);
        let builder = self.client.get(&url).query(&[("nb", nb_games)]);

        Ok(self.to_model::<UserGames>(builder).await?.now_playing)
    }
}
