use serde_json::{from_value, Value};
use crate::client::{Licheszter, LicheszterResult};

impl Licheszter {
    /// Create a challenge
    pub async fn challenge_create(&self, username: &str, form_params: Option<&[(&str, &str)]>) -> LicheszterResult<EntityChallenge> {
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

    /// Decline a challenge
    pub async fn challenge_decline(&self, challenge_id: &str, reason: Option<&str>) -> LicheszterResult<()> {
        let addr = format!("{}/api/challenge/{}/decline", self.base, challenge_id);
        let form = vec![("reason", reason.map_or("generic".to_string(), String::from))];
        let builder = self.client.post(&addr).form(&form);
        let ok_json = self.to_model_full::<Value>(builder);
        assert!(from_value::<bool>(ok_json.await?["ok"].take())?);
        Ok(())
    }

    /// Cancel a challenge
    pub async fn challenge_cancel(&self, challenge_id: &str) -> LicheszterResult<()> {
        let addr = format!("{}/api/challenge/{}/cancel", self.base, challenge_id);
        let builder = self.client.post(&addr);
        let ok_json = self.to_model_full::<Value>(builder);
        assert!(from_value::<bool>(ok_json.await?["ok"].take())?);
        Ok(())
    }

    /// Challenge Stockfish
    pub async fn challenge_stockfish(&self, level: u8, form_params: Option<&[(&str, &str)]>) -> LicheszterResult<ChallengeGame> {
        let addr = format!("{}/api/challenge/ai", self.base);
        let mut form = vec![("level", level.to_string())];
        if let Some(params) = form_params {
            for (key, val) in params.iter() {
                form.push((key, val.to_string()));
            }
        }
        let builder = self.client.post(&addr).form(&form);
        self.to_model_full(builder).await
    }
}