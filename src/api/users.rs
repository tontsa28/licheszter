use crate::{
    client::Licheszter,
    config::users::UserStatusOptions,
    error::Result,
    models::{
        common::OkResponse,
        user::{
            PerfType, RatingHistory, RealtimeUser, TopUser, TopUserLeaderboard, TopUsers, User,
            UserNote,
        },
    },
};

impl Licheszter {
    /// Get the status of one or more users at the same time.
    /// Works with up to 100 users.
    pub async fn users_status(
        &self,
        ids: Vec<&str>,
        options: Option<&UserStatusOptions>,
    ) -> Result<Vec<RealtimeUser>> {
        let mut url = self.base_url.clone();
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
        let mut url = self.base_url.clone();
        url.set_path("api/player");
        let builder = self.client.get(url);

        self.into::<TopUsers>(builder).await
    }

    /// Get the leaderboard for a single speed or variant (perf type).
    /// There are no leaderboards for correspondence or puzzles.
    pub async fn users_leaderboard(&self, amount: u8, perf_type: PerfType) -> Result<Vec<TopUser>> {
        let mut url = self.base_url.clone();
        let path = format!("api/player/top/{amount}/{perf_type}");
        url.set_path(&path);
        let builder = self.client().get(url);

        Ok(self.into::<TopUserLeaderboard>(builder).await?.users)
    }

    /// Read public data of a user.
    pub async fn users_profile(&self, username: &str, trophies: bool) -> Result<User> {
        let mut url = self.base_url.clone();
        let path = format!("api/user/{username}");
        url.set_path(&path);
        let builder = self.client.get(url).query(&[("trophies", trophies)]);

        self.into::<User>(builder).await
    }

    /// Read rating history of a user, for all perf types.
    /// There is at most one entry per day.
    /// Format of an entry is `(year, month, day, rating)` - `month` starts at zero.
    pub async fn users_rating_history(&self, username: &str) -> Result<Vec<RatingHistory>> {
        let mut url = self.base_url.clone();
        let path = format!("api/user/{username}/rating-history");
        url.set_path(&path);
        let builder = self.client.get(url);

        self.into::<Vec<RatingHistory>>(builder).await
    }

    /// Add a private note about the given account.
    /// This note is only visible to the logged in user.
    pub async fn users_notes_write(&self, username: &str, text: &str) -> Result<()> {
        let mut url = self.base_url.clone();
        let path = format!("api/user/{username}/note");
        url.set_path(&path);
        let builder = self.client.post(url).form(&[("text", text)]);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Get the private notes that you have added for a user.
    pub async fn users_notes_read(&self, username: &str) -> Result<Vec<UserNote>> {
        let mut url = self.base_url.clone();
        let path = format!("api/user/{username}/note");
        url.set_path(&path);
        let builder = self.client.get(url);

        self.into::<Vec<UserNote>>(builder).await
    }
}
