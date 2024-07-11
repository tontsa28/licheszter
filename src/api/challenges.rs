use reqwest::header;

use crate::{
    client::Licheszter,
    config::challenges::ChallengeCreateOptions,
    error::Result,
    models::{
        board::{Challenge, ChallengeDeclineReason, ChallengeGame, Challenges},
        common::OkResponse,
    },
};

impl Licheszter {
    /// Get a list of challenges created by targeted at you.
    pub async fn challenge_list(&self) -> Result<Challenges> {
        let mut url = self.base_url();
        url.set_path("api/challenge");
        let builder = self.client.get(url);

        self.to_model::<Challenges>(builder).await
    }

    /// Challenge someone to play.
    /// The targeted player can choose to accept or decline.
    /// The game ID will be the same as the challenge ID.
    pub async fn challenge_create(
        &self,
        username: &str,
        options: Option<&ChallengeCreateOptions>,
    ) -> Result<Challenge> {
        let mut url = self.base_url();
        let path = format!("api/challenge/{username}");
        url.set_path(&path);
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

    /// Get details about a specific challenge, even if it has been recently accepted, canceled or declined.
    pub async fn challenge_show(&self, challenge_id: &str) -> Result<Challenge> {
        let mut url = self.base_url();
        let path = format!("api/challenge/{challenge_id}/show");
        url.set_path(&path);
        let builder = self.client.get(url);

        self.to_model::<Challenge>(builder).await
    }

    /// Accept an incoming challenge.
    pub async fn challenge_accept(&self, challenge_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/challenge/{challenge_id}/accept");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Decline an incoming challenge.
    pub async fn challenge_decline(
        &self,
        challenge_id: &str,
        reason: Option<ChallengeDeclineReason>,
    ) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/challenge/{challenge_id}/decline");
        url.set_path(&path);
        let builder = self
            .client
            .post(url)
            .form(&[("reason", reason.unwrap_or(ChallengeDeclineReason::Generic))]);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Cancel a challenge you sent.
    /// Aborts the game if the challenge was accepted, but the game was not yet played.
    pub async fn challenge_cancel(
        &self,
        challenge_id: &str,
        opponent_token: Option<&str>,
    ) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/challenge/{challenge_id}/cancel");
        url.set_path(&path);
        let mut builder = self.client.post(url);

        // Add the opponent token as a query parameter if it's present
        if let Some(token) = opponent_token {
            builder = builder.query(&[("opponentToken", token)]);
        }

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Start a game with Lichess AI (Stockfish).
    pub async fn challenge_ai(
        &self,
        level: u8,
        opt_params: Option<&[(&str, &str)]>,
    ) -> Result<ChallengeGame> {
        let mut url = self.base_url();
        url.set_path("api/challenge/ai");
        let mut builder = self.client.post(url);

        let level = level.to_string();
        let mut form = vec![("level", level.as_str())];
        if let Some(params) = opt_params {
            form.extend(params);
            builder = builder.form(&form);
        }

        self.to_model::<ChallengeGame>(builder).await
    }

    /// Create a challenge that 2 players can join.
    /// The first 2 players to click the URLs will be paired for a game.
    pub async fn challenge_create_open(&self) -> Result<()> {
        todo!("An optional function argument solution is required")
    }

    /// Start the clocks of a game immediately, even if a player has not yet made a move.
    /// If the clocks have already started, this method will have no effect.
    ///
    /// Requires the OAuth tokens of both players to contain the `challenge:write` scope.
    pub async fn challenge_game_clocks_start(&self, game_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/challenge/{game_id}/start-clocks");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Add seconds to the opponent's clock.
    /// Can be used to create games with time odds.
    pub async fn challenge_opponent_clock_increment(
        &self,
        game_id: &str,
        seconds: u32,
    ) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/round/{game_id}/add-time/{seconds}");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }
}
