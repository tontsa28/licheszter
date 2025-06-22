use crate::{
    client::{Licheszter, UrlBase},
    config::users::UserStatusOptions,
    error::Result,
    models::{
        common::OkResponse,
        user::{
            BasicUser, Crosstable, MinimalUser, PerfType, RatingHistory, RealtimeUser, StreamingUser,
            TopUser, TopUserLeaderboard, TopUsers, User, UserActivity, UserAutocomplete, UserNote,
            UserPerformance,
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
        let mut url = self.req_url(UrlBase::Lichess, "api/users/status");

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
        let url = self.req_url(UrlBase::Lichess, "api/player");
        let builder = self.client.get(url);

        self.into::<TopUsers>(builder).await
    }

    /// Get the leaderboard for a single speed or variant (perf type).
    /// There are no leaderboards for correspondence or puzzles.
    pub async fn users_leaderboard(&self, amount: u8, perf_type: PerfType) -> Result<Vec<TopUser>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/player/top/{amount}/{perf_type}"));
        let builder = self.client.get(url);

        Ok(self.into::<TopUserLeaderboard>(builder).await?.users)
    }

    /// Read public data of a user.
    pub async fn users_profile(&self, username: &str, trophies: bool) -> Result<User> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/user/{username}"));
        let builder = self.client.get(url).query(&[("trophies", trophies)]);

        self.into::<User>(builder).await
    }

    /// Read rating history of a user, for all perf types.
    /// There is at most one entry per day.
    /// Format of an entry is `(year, month, day, rating)` - `month` starts at zero.
    pub async fn users_rating_history(&self, username: &str) -> Result<Vec<RatingHistory>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/user/{username}/rating-history"));
        let builder = self.client.get(url);

        self.into::<Vec<RatingHistory>>(builder).await
    }

    /// Read performance statistics of a user, for a single performance.
    pub async fn users_performance(&self, username: &str, perf: PerfType) -> Result<UserPerformance> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/user/{username}/perf/{perf}"));
        let builder = self.client.get(url);

        self.into::<UserPerformance>(builder).await
    }

    /// Read data to generate the activity feed of a user.
    pub async fn users_activity(&self, username: &str) -> Result<Vec<UserActivity>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/user/{username}/activity"));
        let builder = self.client.get(url);

        self.into::<Vec<UserActivity>>(builder).await
    }

    /// Get up to 300 users by their IDs.
    /// This endpoint is limited to 8 000 users every 10 minutes and 120 000 every day.
    pub async fn users_list(&self, ids: Vec<&str>) -> Result<Vec<BasicUser>> {
        let url = self.req_url(UrlBase::Lichess, "api/users");
        let builder = self.client.post(url).body(ids.join(","));

        self.into::<Vec<BasicUser>>(builder).await
    }

    /// Get basic information about currently streaming users.
    pub async fn users_streamers_live(&self) -> Result<Vec<StreamingUser>> {
        let url = self.req_url(UrlBase::Lichess, "api/streamer/live");
        let builder = self.client.get(url);

        self.into::<Vec<StreamingUser>>(builder).await
    }

    /// Get total number of games, and current score, of any two users.
    /// If `matchup` is set to `true` and the users are currently playing, then this method also gets the current match game number and scores.
    pub async fn users_crosstable(&self, user1: &str, user2: &str, matchup: bool) -> Result<Crosstable> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/crosstable/{user1}/{user2}"));
        let builder = self.client.get(url).query(&[("matchup", matchup)]);

        self.into::<Crosstable>(builder).await
    }

    /// Provides autocompletion options for an incomplete username.
    pub async fn users_autocomplete(&self, term: &str, friend: bool) -> Result<Vec<String>> {
        let url = self.req_url(UrlBase::Lichess, "api/player/autocomplete");
        let builder = self.client.get(url).query(&(("term", term), ("friend", friend)));

        self.into::<Vec<String>>(builder).await
    }

    /// Provides detailed autocompletion options for an incomplete username.
    /// Each result contains some basic information about the user in question.
    pub async fn users_autocomplete_details(
        &self,
        term: &str,
        friend: bool,
    ) -> Result<Vec<MinimalUser>> {
        let url = self.req_url(UrlBase::Lichess, "api/player/autocomplete");
        let builder =
            self.client
                .get(url)
                .query(&(("term", term), ("object", true), ("friend", friend)));

        Ok(self.into::<UserAutocomplete>(builder).await?.result)
    }

    /// Add a private note about the given account.
    /// This note is only visible to the logged in user.
    pub async fn users_notes_write(&self, username: &str, text: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/user/{username}/note"));
        let builder = self.client.post(url).form(&[("text", text)]);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Get the private notes that you have added for a user.
    pub async fn users_notes_read(&self, username: &str) -> Result<Vec<UserNote>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/user/{username}/note"));
        let builder = self.client.get(url);

        self.into::<Vec<UserNote>>(builder).await
    }
}
