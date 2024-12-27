use crate::{
    client::Licheszter,
    config::users::UserStatusOptions,
    error::Result,
    models::user::{RealtimeUser, TopUsers},
};

impl Licheszter {
    /// Get the status of one or more users at the same time.
    /// Works with up to 100 users.
    pub async fn users_status(
        &self,
        ids: Vec<&str>,
        options: Option<&UserStatusOptions>,
    ) -> Result<Vec<RealtimeUser>> {
        let mut url = self.base_url();
        url.set_path("api/users/status");

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.client.get(url).query(&[("ids", ids.join(","))]);
        self.into::<Vec<RealtimeUser>>(builder).await
    }

    /// Get the top 10 players for each speed and variant.
    pub async fn users_top10(&self) -> Result<TopUsers> {
        let mut url = self.base_url();
        url.set_path("api/player");
        let builder = self.client.get(url);

        self.into::<TopUsers>(builder).await
    }
}
