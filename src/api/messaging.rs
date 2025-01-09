use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::common::OkResponse,
};

impl Licheszter {
    pub async fn message_private_send(&self, username: &str, text: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("/inbox/{username}"));
        let builder = self.client.post(url).form(&[("text", text)]);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }
}
