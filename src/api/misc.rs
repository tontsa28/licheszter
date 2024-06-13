use crate::{
    client::Licheszter,
    error::Result,
    models::{
        board::Event, common::OkResponse, game::{UserGame, UserGames}
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

    /// Upgrade a Lichess player account into a bot account.
    /// This method only works for bot accounts.
    /// The account MUST NOT have any games played before upgrading.
    /// This action is irreversible.
    pub async fn account_upgrade_bot(&self, token: &str) -> Result<()> {
        let mut url = self.base_url();
        url.set_path("api/bot/account/upgrade");
        let bearer = format!("Bearer {token}");
        let builder = self.client.post(url).header("Authorization", bearer);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }
}
