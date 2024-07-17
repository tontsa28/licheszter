use crate::{
    client::Licheszter,
    error::Result,
    models::{common::OkResponse, user::BotUser},
};
use futures_util::Stream;

impl Licheszter {
    /// Get online bots.
    pub async fn bots_online(&self, bots: u8) -> Result<impl Stream<Item = Result<BotUser>>> {
        let mut url = self.base_url();
        url.set_path("api/bot/online");
        let builder = self.client.get(url).query(&[("nb", bots)]);

        self.to_model_stream::<BotUser>(builder).await
    }

    /// Upgrade a Lichess player account into a bot account.
    /// This method only works for bot accounts.
    /// The account MUST NOT have any games played before upgrading.
    /// This action is irreversible.
    pub async fn bot_account_upgrade(&self, token: &str) -> Result<()> {
        let mut url = self.base_url();
        url.set_path("api/bot/account/upgrade");
        let bearer = format!("Bearer {token}");
        let builder = self.client.post(url).header("Authorization", bearer);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }
}
