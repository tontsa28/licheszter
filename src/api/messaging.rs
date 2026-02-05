use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
};

impl Licheszter {
    /// Send a private message to another player.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn message_private_send(&self, username: &str, text: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("/inbox/{username}"));
        let builder = self.client.post(url).form(&[("text", text)]);

        self.execute(builder).await
    }
}
