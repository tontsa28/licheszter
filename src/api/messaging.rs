use crate::{client::Licheszter, error::Result, models::common::OkResponse};

impl Licheszter {
    pub async fn message_private_send(&self, username: &str, text: &str) -> Result<()> {
        let mut url = self.base_url.clone();
        let path = format!("/inbox/{username}");
        url.set_path(&path);
        let builder = self.client.post(url).form(&[("text", text)]);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }
}
