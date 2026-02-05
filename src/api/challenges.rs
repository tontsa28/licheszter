use std::pin::Pin;

use futures_util::Stream;
use reqwest::header;

use crate::{
    client::{Licheszter, UrlBase},
    config::challenges::{AIChallengeOptions, ChallengeOptions, OpenChallengeOptions},
    error::Result,
    models::{
        challenge::{
            AIChallenge, Challenge, ChallengeComplete, ChallengeDeclineReason, Challenges, OpenChallenge,
        },
        game::AILevel,
    },
};

impl Licheszter {
    /// Get a list of challenges created by targeted at you.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn challenge_list(&self) -> Result<Challenges> {
        let url = self.req_url(UrlBase::Lichess, "api/challenge");
        let builder = self.client.get(url);

        self.to_model::<Challenges>(builder).await
    }

    /// Challenge someone to play.
    /// The targeted player can choose to accept or decline.
    /// The game ID will be the same as the challenge ID.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn challenge_create(
        &self,
        username: &str,
        options: Option<&ChallengeOptions>,
    ) -> Result<Challenge> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/challenge/{username}"));
        let mut builder = self.client.post(url);

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?.replace('_', ".");
            builder = builder
                .body(encoded)
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
        }

        self.to_model::<Challenge>(builder).await
    }

    /// Challenge someone to play and stream the response.
    /// The challenge is kept alive until the connection is closed by the client.
    /// The targeted player can choose to accept or decline.
    /// The game ID will be the same as the challenge ID.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn challenge_create_connect(
        &self,
        username: &str,
        options: Option<&ChallengeOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChallengeComplete>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/challenge/{username}"));
        let mut builder = self.client.post(url).form(&[("keepAliveStream", true)]);

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = "keepAliveStream=true&".to_string()
                + &comma_serde_urlencoded::to_string(options)?.replace('_', ".");
            builder = builder
                .body(encoded)
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
        }

        self.to_stream::<ChallengeComplete>(builder).await
    }

    /// Get details about a specific challenge, even if it has been recently accepted, canceled or declined.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn challenge_show(&self, challenge_id: &str) -> Result<Challenge> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/challenge/{challenge_id}/show"));
        let builder = self.client.get(url);

        self.to_model::<Challenge>(builder).await
    }

    /// Accept an incoming challenge.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn challenge_accept(&self, challenge_id: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/challenge/{challenge_id}/accept"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }

    /// Decline an incoming challenge.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn challenge_decline(
        &self,
        challenge_id: &str,
        reason: Option<ChallengeDeclineReason>,
    ) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/challenge/{challenge_id}/decline"));
        let builder = self
            .client
            .post(url)
            .form(&[("reason", reason.unwrap_or(ChallengeDeclineReason::Generic))]);

        self.execute(builder).await
    }

    /// Cancel a challenge you sent.
    /// Aborts the game if the challenge was accepted, but the game was not yet played.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn challenge_cancel(
        &self,
        challenge_id: &str,
        opponent_token: Option<&str>,
    ) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/challenge/{challenge_id}/cancel"));
        let mut builder = self.client.post(url);

        // Add the opponent token as a query parameter if it's present
        if let Some(token) = opponent_token {
            builder = builder.query(&[("opponentToken", token)]);
        }

        self.execute(builder).await
    }

    /// Start a game with Lichess AI (Stockfish).
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn challenge_ai(
        &self,
        level: AILevel,
        options: Option<&AIChallengeOptions>,
    ) -> Result<AIChallenge> {
        let url = self.req_url(UrlBase::Lichess, "api/challenge/ai");
        let mut builder = self.client.post(url).form(&[("level", level as u8)]);

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?.replace('_', ".");
            let level = comma_serde_urlencoded::to_string([("level", level as u8)])? + "&";
            let form = level + &encoded;
            builder = builder.body(form);
        }

        self.to_model::<AIChallenge>(builder).await
    }

    /// Create a challenge that 2 players can join.
    /// The first 2 players to click the URLs will be paired for a game.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn challenge_create_open(
        &self,
        options: Option<&OpenChallengeOptions>,
    ) -> Result<OpenChallenge> {
        let url = self.req_url(UrlBase::Lichess, "api/challenge/open");
        let mut builder = self.client.post(url);

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?.replace('_', ".");
            builder = builder
                .body(encoded)
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
        }

        self.to_model::<OpenChallenge>(builder).await
    }

    /// Start the clocks of a game immediately, even if a player has not yet made a move.
    /// If the clocks have already started, this method will have no effect.
    ///
    /// Requires the OAuth tokens of both players to contain the `challenge:write` scope.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn challenge_game_clocks_start(
        &self,
        game_id: &str,
        token1: &str,
        token2: &str,
    ) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/challenge/{game_id}/start-clocks"));
        let builder = self
            .client
            .post(url)
            .query(&[("token1", token1), ("token2", token2)]);

        self.execute(builder).await
    }

    /// Add seconds to the opponent's clock.
    /// Can be used to create games with time odds.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn challenge_opponent_clock_increment(&self, game_id: &str, seconds: u32) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/round/{game_id}/add-time/{seconds}"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }
}
