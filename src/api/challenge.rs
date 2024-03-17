use crate::{
    client::Licheszter,
    error::Result,
    models::{
        board::{ChallengeGame, Challenges, EntityChallenge},
        common::OkResponse,
    },
};

impl Licheszter {
    /// Create a challenge.
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

    /// Accept a challenge.
    pub async fn challenge_accept(&self, challenge_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/challenge/{challenge_id}/accept");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Decline a challenge.
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

    /// Cancel a challenge.
    pub async fn challenge_cancel(&self, challenge_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/challenge/{challenge_id}/cancel");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Challenge Stockfish.
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

    /// Get the challenges of the current user.
    pub async fn challenges(&self) -> Result<Challenges> {
        let mut url = self.base_url();
        url.set_path("api/challenge");
        let builder = self.client.get(url);

        self.to_model::<Challenges>(builder).await
    }
}
