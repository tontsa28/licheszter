use std::pin::Pin;

use futures_util::Stream;

use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::user::User,
};

impl Licheszter {
    /// Get a list of users followed by the logged in user.
    pub async fn relations_followed_users_list(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<User>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, "api/rel/following");
        let builder = self.client.get(url);

        self.to_stream::<User>(builder).await
    }

    /// Follow a player, adding them to your list of Lichess friends.
    pub async fn relations_follow(&self, username: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/rel/follow/{username}"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }

    /// Unfollow a player, removing them from your list of Lichess friends.
    pub async fn relations_unfollow(&self, username: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/rel/unfollow/{username}"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }

    /// Block a player, adding them to your list of blocked Lichess users.
    pub async fn relations_block(&self, username: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/rel/block/{username}"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }

    /// Unblock a player, removing them from your list of blocked Lichess users.
    pub async fn relations_unblock(&self, username: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/rel/unblock/{username}"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }
}
