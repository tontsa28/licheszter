use futures_util::Stream;
use serde_json::{Value, from_value};
use crate::client::{Licheszter, LicheszterResult};
use crate::models::{board::Event, game::UserGame};

impl Licheszter {
    /// Stream incoming events
    pub async fn stream_events(&self) -> LicheszterResult<impl Stream<Item = LicheszterResult<Event>>> {
        let addr = format!("{}/api/stream/event", self.base);
        let builder = self.client.get(&addr);
        self.to_model_stream(builder).await
    }

    /// Get ongoing games of the current user
    pub async fn get_ongoing_games(&self, nb_games: u8) -> LicheszterResult<Vec<UserGame>> {
        let addr = format!("{}/api/account/playing", self.base);
        let builder = self.client.get(&addr).query(&[("nb", nb_games)]);
        let playing_json = self.to_model_full::<Value>(builder);
        from_value(playing_json.await?["nowPlaying"].take()).map_err(Into::into)
    }
}