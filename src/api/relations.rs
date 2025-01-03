use futures_util::Stream;
use serde_json::Value;

use crate::{
    client::Licheszter,
    error::Result,
    models::{common::OkResponse, user::User},
};

impl Licheszter {
    /// Get a list of users followed by the logged in user.
    pub async fn relations_followed_users_list(&self) -> Result<impl Stream<Item = Result<User>>> {
        let mut url = self.base_url.clone();
        url.set_path("api/rel/following");
        let builder = self.client.get(url);

        self.into_stream::<User>(builder).await
    }

    /// Follow a player, adding them to your list of Lichess friends.
    pub async fn relations_follow(&self, username: &str) -> Result<()> {
        let mut url = self.base_url.clone();
        let path = format!("api/rel/follow/{username}");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Unfollow a player, removing them from your list of Lichess friends.
    pub async fn relations_unfollow(&self, username: &str) -> Result<()> {
        let mut url = self.base_url.clone();
        let path = format!("api/rel/unfollow/{username}");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Block a player, adding them to your list of blocked Lichess users.
    pub async fn relations_block(&self, username: &str) -> Result<()> {
        let mut url = self.base_url.clone();
        let path = format!("api/rel/block/{username}");
        url.set_path(&path);
        let builder = self.client.post(url);

        // TODO: Temporary solution, waiting for Lichess developers' comments on the returned data
        self.into::<Value>(builder).await?;
        Ok(())
    }

    /// Unblock a player, removing them from your list of blocked Lichess users.
    pub async fn relations_unblock(&self, username: &str) -> Result<()> {
        let mut url = self.base_url.clone();
        let path = format!("api/rel/unblock/{username}");
        url.set_path(&path);
        let builder = self.client.post(url);

        // TODO: Temporary solution, waiting for Lichess developers' comments on the returned data
        self.into::<Value>(builder).await?;
        Ok(())
    }
}
