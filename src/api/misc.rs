use crate::{
    client::Licheszter,
    error::Result,
    models::{
        board::Event,
        game::{UserGame, UserGames},
    },
};
use futures_util::Stream;

impl Licheszter {
    /// Stream incoming events.
    pub async fn events_stream(&self) -> Result<impl Stream<Item = Result<Event>>> {
        let mut url = self.base_url();
        url.set_path("api/stream/event");
        let builder = self.client.get(url);

        self.to_model_stream::<Event>(builder).await
    }

    /// Get ongoing games of the current user.
    // TODO: Move elsewhere when the whole endpoint group is implemented
    pub async fn games_ongoing(&self, nb_games: u8) -> Result<Vec<UserGame>> {
        let mut url = self.base_url();
        url.set_path("api/account/playing");
        let builder = self.client.get(url).query(&[("nb", nb_games)]);

        Ok(self.to_model::<UserGames>(builder).await?.now_playing)
    }
}
