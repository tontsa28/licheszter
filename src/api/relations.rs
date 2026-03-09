use std::pin::Pin;

use futures_util::Stream;

use crate::{
    client::{LicheszterInner, UrlBase},
    error::Result,
    models::user::User,
};

use std::sync::Arc;

/// A struct for accessing the Relations API endpoints.
#[derive(Debug)]
pub struct RelationsApi {
    pub(crate) inner: Arc<LicheszterInner>,
}

impl RelationsApi {
    /// Get a list of users followed by the logged in user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn followed_users_list(&self) -> Result<Pin<Box<dyn Stream<Item = Result<User>> + Send>>> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/rel/following");
        let builder = self.inner.client.get(url);

        self.inner.to_stream::<User>(builder).await
    }

    /// Follow a player, adding them to your list of Lichess friends.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn follow(&self, username: &str) -> Result<()> {
        let url = self.inner.req_url(UrlBase::Lichess, &format!("api/rel/follow/{username}"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }

    /// Unfollow a player, removing them from your list of Lichess friends.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn unfollow(&self, username: &str) -> Result<()> {
        let url = self.inner.req_url(UrlBase::Lichess, &format!("api/rel/unfollow/{username}"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }

    /// Block a player, adding them to your list of blocked Lichess users.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn block(&self, username: &str) -> Result<()> {
        let url = self.inner.req_url(UrlBase::Lichess, &format!("api/rel/block/{username}"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }

    /// Unblock a player, removing them from your list of blocked Lichess users.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn unblock(&self, username: &str) -> Result<()> {
        let url = self.inner.req_url(UrlBase::Lichess, &format!("api/rel/unblock/{username}"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }
}
