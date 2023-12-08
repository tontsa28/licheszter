use crate::{
    client::Licheszter,
    error::Result,
    models::{board::{ChallengeGame, Challenges, EntityChallenge}, common::OkResponse},
};

impl Licheszter {
    /// Create a challenge.
    pub async fn challenge_create(
        &self,
        username: &str,
        form_params: Option<&[(&str, &str)]>,
    ) -> Result<EntityChallenge> {
        let url = format!("{}/api/challenge/{}", self.base_url, username);
        let mut builder = self.client.post(&url);

        if let Some(params) = form_params {
            builder = builder.form(&params);
        }

        self.to_model::<EntityChallenge>(builder).await
    }

    /// Accept a challenge.
    pub async fn challenge_accept(&self, challenge_id: &str) -> Result<()> {
        let url = format!("{}/api/challenge/{}/accept", self.base_url, challenge_id);
        let builder = self.client.post(&url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Decline a challenge.
    pub async fn challenge_decline(&self, challenge_id: &str, reason: Option<&str>) -> Result<()> {
        let url = format!("{}/api/challenge/{}/decline", self.base_url, challenge_id);
        let builder = self
            .client
            .post(&url)
            .form(&[("reason", reason.unwrap_or("generic"))]);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Cancel a challenge.
    pub async fn challenge_cancel(&self, challenge_id: &str) -> Result<()> {
        let url = format!("{}/api/challenge/{}/cancel", self.base_url, challenge_id);
        let builder = self.client.post(&url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Challenge Stockfish.
    pub async fn challenge_stockfish(
        &self,
        level: u8,
        form_params: Option<&[(&str, &str)]>,
    ) -> Result<ChallengeGame> {
        let url = format!("{}/api/challenge/ai", self.base_url);
        let mut builder = self.client.post(&url);

        let level = level.to_string();
        let mut form = vec![("level", level.as_str())];
        if let Some(params) = form_params {
            form.extend(params);
            builder = builder.form(&form);
        }

        self.to_model::<ChallengeGame>(builder).await
    }

    /// Get the challenges of the current user.
    pub async fn get_challenges(&self) -> Result<Challenges> {
        let url = format!("{}/api/challenge", self.base_url);
        let builder = self.client.get(&url);

        self.to_model::<Challenges>(builder).await
    }
}
