use std::pin::Pin;

use futures_util::Stream;

use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::user::User,
};

/// A struct for accessing the Relations API endpoints.
pub struct RelationsApi<'a> {
    pub(crate) client: &'a Licheszter,
}

impl RelationsApi<'_> {
    /// Get a list of users followed by the logged in user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn followed_users_list(&self) -> Result<Pin<Box<dyn Stream<Item = Result<User>> + Send>>> {
        let url = self.client.req_url(UrlBase::Lichess, "api/rel/following");
        let builder = self.client.client.get(url);

        self.client.to_stream::<User>(builder).await
    }

    /// Follow a player, adding them to your list of Lichess friends.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn follow(&self, username: &str) -> Result<()> {
        let url = self
            .client
            .req_url(UrlBase::Lichess, &format!("api/rel/follow/{username}"));
        let builder = self.client.client.post(url);

        self.client.execute(builder).await
    }

    /// Unfollow a player, removing them from your list of Lichess friends.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn unfollow(&self, username: &str) -> Result<()> {
        let url = self
            .client
            .req_url(UrlBase::Lichess, &format!("api/rel/unfollow/{username}"));
        let builder = self.client.client.post(url);

        self.client.execute(builder).await
    }

    /// Block a player, adding them to your list of blocked Lichess users.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn block(&self, username: &str) -> Result<()> {
        let url = self
            .client
            .req_url(UrlBase::Lichess, &format!("api/rel/block/{username}"));
        let builder = self.client.client.post(url);

        self.client.execute(builder).await
    }

    /// Unblock a player, removing them from your list of blocked Lichess users.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn unblock(&self, username: &str) -> Result<()> {
        let url = self
            .client
            .req_url(UrlBase::Lichess, &format!("api/rel/unblock/{username}"));
        let builder = self.client.client.post(url);

        self.client.execute(builder).await
    }
}
