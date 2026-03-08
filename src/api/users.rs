use crate::{
    client::{LicheszterInner, UrlBase},
    config::users::UserStatusOptions,
    error::Result,
    models::user::{
        BasicUser, Crosstable, MinimalUser, PerfType, RatingHistory, RealtimeUser, StreamingUser,
        TopUser, TopUserLeaderboard, TopUsers, User, UserActivity, UserAutocomplete, UserNote,
        UserPerformance,
    },
};

use std::sync::Arc;

/// A struct for accessing the Users API endpoints.
#[derive(Debug)]
pub struct UsersApi {
    pub(crate) inner: Arc<LicheszterInner>,
}

impl UsersApi {
    /// Get the status of one or more users at the same time.
    /// Works with up to 100 users.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn status(
        &self,
        user_ids: &[&str],
        options: Option<&UserStatusOptions>,
    ) -> Result<Vec<RealtimeUser>> {
        let mut url = self.inner.req_url(UrlBase::Lichess, "api/users/status");

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.inner.client.get(url).query(&[("ids", user_ids.join(","))]);
        self.inner.to_model::<Vec<RealtimeUser>>(builder).await
    }

    /// Get the top 10 players for each speed and variant.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn top10(&self) -> Result<TopUsers> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/player");
        let builder = self.inner.client.get(url);

        self.inner.to_model::<TopUsers>(builder).await
    }

    /// Get the leaderboard for a single speed or variant (perf type).
    /// There are no leaderboards for correspondence or puzzles.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn leaderboard(&self, amount: u8, perf_type: PerfType) -> Result<Vec<TopUser>> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/player/top/{amount}/{perf_type}"));
        let builder = self.inner.client.get(url);

        Ok(self.inner.to_model::<TopUserLeaderboard>(builder).await?.users)
    }

    /// Read public data of a user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn profile(&self, username: &str, trophies: bool) -> Result<User> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/user/{username}"));
        let builder = self.inner.client.get(url).query(&[("trophies", trophies)]);

        self.inner.to_model::<User>(builder).await
    }

    /// Read rating history of a user, for all perf types.
    /// There is at most one entry per day.
    /// Format of an entry is `(year, month, day, rating)` - `month` starts at zero.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn rating_history(&self, username: &str) -> Result<Vec<RatingHistory>> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/user/{username}/rating-history"));
        let builder = self.inner.client.get(url);

        self.inner.to_model::<Vec<RatingHistory>>(builder).await
    }

    /// Read performance statistics of a user, for a single performance.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn performance(&self, username: &str, perf: PerfType) -> Result<UserPerformance> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/user/{username}/perf/{perf}"));
        let builder = self.inner.client.get(url);

        self.inner.to_model::<UserPerformance>(builder).await
    }

    /// Read data to generate the activity feed of a user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn activity(&self, username: &str) -> Result<Vec<UserActivity>> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/user/{username}/activity"));
        let builder = self.inner.client.get(url);

        self.inner.to_model::<Vec<UserActivity>>(builder).await
    }

    /// Get up to 300 users by their IDs.
    /// This endpoint is limited to 8 000 users every 10 minutes and 120 000 every day.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn list(&self, user_ids: &[&str]) -> Result<Vec<BasicUser>> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/users");
        let builder = self.inner.client.post(url).body(user_ids.join(","));

        self.inner.to_model::<Vec<BasicUser>>(builder).await
    }

    /// Get basic information about currently streaming users.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn streamers_live(&self) -> Result<Vec<StreamingUser>> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/streamer/live");
        let builder = self.inner.client.get(url);

        self.inner.to_model::<Vec<StreamingUser>>(builder).await
    }

    /// Get total number of games, and current score, of any two users.
    /// If `matchup` is set to `true` and the users are currently playing, then this method also gets the current match game number and scores.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn crosstable(&self, user1: &str, user2: &str, matchup: bool) -> Result<Crosstable> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/crosstable/{user1}/{user2}"));
        let builder = self.inner.client.get(url).query(&[("matchup", matchup)]);

        self.inner.to_model::<Crosstable>(builder).await
    }

    /// Provides autocompletion options for an incomplete username.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn autocomplete(&self, term: &str, friend: bool) -> Result<Vec<String>> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/player/autocomplete");
        let builder = self
            .inner
            .client
            .get(url)
            .query(&(("term", term), ("friend", friend)));

        self.inner.to_model::<Vec<String>>(builder).await
    }

    /// Provides detailed autocompletion options for an incomplete username.
    /// Each result contains some basic information about the user in question.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn autocomplete_details(&self, term: &str, friend: bool) -> Result<Vec<MinimalUser>> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/player/autocomplete");
        let builder =
            self.inner
                .client
                .get(url)
                .query(&(("term", term), ("object", true), ("friend", friend)));

        Ok(self.inner.to_model::<UserAutocomplete>(builder).await?.result)
    }

    /// Add a private note about the given account.
    /// This note is only visible to the logged in user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn notes_write(&self, username: &str, text: &str) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/user/{username}/note"));
        let builder = self.inner.client.post(url).form(&[("text", text)]);

        self.inner.execute(builder).await
    }

    /// Get the private notes that you have added for a user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn notes_read(&self, username: &str) -> Result<Vec<UserNote>> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/user/{username}/note"));
        let builder = self.inner.client.get(url);

        self.inner.to_model::<Vec<UserNote>>(builder).await
    }
}
