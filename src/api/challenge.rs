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
}