use crate::{
    client::Licheszter,
    error::Result,
    models::{board::Event, game::UserGame},
};
use futures_util::Stream;
use serde_json::{from_value, Value};

impl Licheszter {
    /// Stream incoming events.
    pub async fn stream_events(&self) -> Result<impl Stream<Item = Result<Event>>> {
        let url = format!("{}/api/stream/event", self.base);
        let builder = self.client.get(&url);
        self.to_model_stream(builder).await
    }

    /// Get ongoing games of the current user.
    pub async fn get_ongoing_games(&self, nb_games: u8) -> Result<Vec<UserGame>> {
        let url = format!("{}/api/account/playing", self.base);
        let builder = self.client.get(&url).query(&[("nb", nb_games)]);
        let playing_json = self.to_model::<Value>(builder);
        from_value(playing_json.await?["nowPlaying"].take()).map_err(Into::into)
    }
}
