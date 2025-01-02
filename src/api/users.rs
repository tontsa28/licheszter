use crate::{
    client::Licheszter,
    config::users::UserStatusOptions,
    error::Result,
    models::user::{PerfType, RealtimeUser, TopUser, TopUserLeaderboard, TopUsers},
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

    /// Get the leaderboard for a single speed or variant (perf type).
    /// There are no leaderboards for correspondence or puzzles.
    pub async fn users_leaderboard(&self, amount: u8, perf_type: PerfType) -> Result<Vec<TopUser>> {
        let mut url = self.base_url();
        let path = format!("api/player/top/{amount}/{perf_type}");
        url.set_path(&path);
        let builder = self.client().get(url);

        Ok(self.into::<TopUserLeaderboard>(builder).await?.users)
    }
}
