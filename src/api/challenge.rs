use serde_json::{from_value, Value};
use crate::client::{Licheszter, LicheszterResult};

impl Licheszter {
    /// Create a challenge
    pub async fn create_challenge(&self, username: &str, form_params: Option<&[(&str, &str)]>) -> LicheszterResult<EntityChallenge> {
        let addr = format!("{}/api/challenge/{}", self.base, username);
        let mut builder = self.client.post(&addr);
        if let Some(params) = form_params {
            builder = builder.form(&params);
        }
        self.to_model_full(builder).await
    }

    /// Accept a challenge
    pub async fn challenge_accept(&self, challenge_id: &str) -> LicheszterResult<()> {
        let addr = format!("{}/api/challenge/{}/accept", self.base, challenge_id);
        let builder = self.client.post(&addr);
        let ok_json = self.to_model_full::<Value>(builder);
        assert!(from_value::<bool>(ok_json.await?["ok"].take())?);
        Ok(())
    }
}