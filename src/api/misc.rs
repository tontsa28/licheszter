use std::pin::Pin;

use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::{board::Event, common::OkResponse, user::BasicUser},
};
use futures_util::Stream;

impl Licheszter {
    /// Stream the events reaching a Lichess user in real time.
    /// When the stream opens, all current challenges and games are sent.
    pub async fn connect(&self) -> Result<Pin<Box<dyn Stream<Item = Result<Event>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, "api/stream/event");
        let builder = self.client.get(url);

        self.to_stream::<Event>(builder).await
    }

    /// Get online bots.
    pub async fn bots_online(
        &self,
        bots: u8,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<BasicUser>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, "api/bot/online");
        let builder = self.client.get(url).query(&[("nb", bots)]);

        self.to_stream::<BasicUser>(builder).await
    }

    /// Upgrade a Lichess player account into a bot account.
    /// This method only works for bot accounts.
    /// The account MUST NOT have any games played before upgrading.
    /// This action is irreversible.
    pub async fn bot_account_upgrade(&self, token: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, "api/bot/account/upgrade");
        let bearer = format!("Bearer {token}");
        let builder = self.client.post(url).header("Authorization", bearer);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }
}
