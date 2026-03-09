use crate::{
    client::{LicheszterInner, UrlBase},
    error::Result,
};

use std::sync::Arc;

/// A struct for accessing the Messaging API endpoints.
#[derive(Debug)]
pub struct MessagingApi {
    pub(crate) inner: Arc<LicheszterInner>,
}

impl MessagingApi {
    /// Send a private message to another player.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn private_send(&self, username: &str, text: &str) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("/inbox/{username}"));
        let builder = self.inner.client.post(url).form(&[("text", text)]);

        self.inner.execute(builder).await
    }
}
