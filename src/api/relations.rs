use futures_util::Stream;

use crate::{
    client::Licheszter,
    error::Result,
    models::{common::OkResponse, user::User},
};

impl Licheszter {
    /// Get a list of users followed by the logged in user.
    pub async fn relations_followed_users_list(&self) -> Result<impl Stream<Item = Result<User>>> {
        let mut url = self.base_url();
        url.set_path("api/rel/following");
        let builder = self.client.get(url);

        self.to_model_stream::<User>(builder).await
    }

    /// Follow a player, adding them to your list of Lichess friends.
    pub async fn relations_follow(&self, username: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/rel/follow/{username}");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }
}
