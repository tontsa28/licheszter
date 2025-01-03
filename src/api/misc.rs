use crate::{
    client::Licheszter,
    error::Result,
    models::{
        board::Event,
        common::OkResponse,
        game::{UserGame, UserGames},
        user::BotUser,
    },
};
use futures_util::Stream;

impl Licheszter {
    /// Stream the events reaching a Lichess user in real time.
    /// When the stream opens, all current challenges and games are sent.
    pub async fn connect(&self) -> Result<impl Stream<Item = Result<Event>>> {
        let mut url = self.base_url.clone();
        url.set_path("api/stream/event");
        let builder = self.client.get(url);

        self.into_stream::<Event>(builder).await
    }

    /// Get the ongoing games of the current user.
    /// The most urgent games are listed first.
    // TODO: Move elsewhere when the whole endpoint group is implemented
    pub async fn games_ongoing(&self, games: u8) -> Result<Vec<UserGame>> {
        let mut url = self.base_url.clone();
        url.set_path("api/account/playing");
        let builder = self.client.get(url).query(&[("nb", games)]);

        Ok(self.into::<UserGames>(builder).await?.now_playing)
    }

    /// Get online bots.
    pub async fn bots_online(&self, bots: u8) -> Result<impl Stream<Item = Result<BotUser>>> {
        let mut url = self.base_url.clone();
        url.set_path("api/bot/online");
        let builder = self.client.get(url).query(&[("nb", bots)]);

        self.into_stream::<BotUser>(builder).await
    }

    /// Upgrade a Lichess player account into a bot account.
    /// This method only works for bot accounts.
    /// The account MUST NOT have any games played before upgrading.
    /// This action is irreversible.
    pub async fn bot_account_upgrade(&self, token: &str) -> Result<()> {
        let mut url = self.base_url.clone();
        url.set_path("api/bot/account/upgrade");
        let bearer = format!("Bearer {token}");
        let builder = self.client.post(url).header("Authorization", bearer);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }
}
