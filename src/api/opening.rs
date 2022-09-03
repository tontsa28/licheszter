use crate::{client::{Licheszter, LicheszterResult}, models::opening::Opening};

impl Licheszter {
    /// Search Lichess masters database
    pub async fn opening_masters(&self, query_params: &[(&str, &str)]) -> LicheszterResult<Opening> {
        let addr = format!("https://explorer.lichess.ovh/masters");
        let builder = self.client.get(&addr).query(&query_params);
        self.to_model_full(builder).await
    }
}