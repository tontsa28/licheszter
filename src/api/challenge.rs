use crate::{
    client::Licheszter,
    error::Result,
    models::{
        board::{ChallengeGame, Challenges, EntityChallenge},
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
        opt_params: Option<&[(&str, &str)]>,
    ) -> Result<EntityChallenge> {
        let mut url = self.base_url();
        let path = format!("api/challenge/{username}");
        url.set_path(&path);
        let mut builder = self.client.post(url);

        if let Some(params) = opt_params {
            builder = builder.form(&params);
        }

        self.to_model::<EntityChallenge>(builder).await
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
    pub async fn challenge_decline(&self, challenge_id: &str, reason: Option<&str>) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/challenge/{challenge_id}/decline");
        url.set_path(&path);
        let builder = self
            .client
            .post(url)
            .form(&[("reason", reason.unwrap_or("generic"))]);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Cancel a challenge you sent.
    /// Aborts the game if the challenge was accepted, but the game was not yet played.
    pub async fn challenge_cancel(&self, challenge_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/challenge/{challenge_id}/cancel");
        url.set_path(&path);
        let builder = self.client.post(url);

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
