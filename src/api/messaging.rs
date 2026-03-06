use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
};

/// A struct for accessing the Messaging API endpoints.
pub struct MessagingApi<'a> {
    pub(crate) client: &'a Licheszter,
}

impl MessagingApi<'_> {
    /// Send a private message to another player.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn private_send(&self, username: &str, text: &str) -> Result<()> {
        let url = self
            .client
            .req_url(UrlBase::Lichess, &format!("/inbox/{username}"));
        let builder = self.client.client.post(url).form(&[("text", text)]);

        self.client.execute(builder).await
    }
}
