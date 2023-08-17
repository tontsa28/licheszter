use crate::{
    client::Licheszter,
    error::Result,
    models::explorer::{Opening, PlayerOpening},
};

impl Licheszter {
    /// Search Masters database
    pub async fn opening_masters(&self, query_params: &[(&str, &str)]) -> Result<Opening> {
        let url = format!("https://explorer.lichess.ovh/masters");
        let builder = self.client.get(&url).query(&query_params);
        self.to_model(builder).await
    }

    /// Search Lichess database
    pub async fn opening_lichess(&self, query_params: &[(&str, &str)]) -> Result<Opening> {
        let url = format!("https://explorer.lichess.ovh/lichess");
        let builder = self.client.get(&url).query(&query_params);
        self.to_model(builder).await
    }

    /// Search Player database
    pub async fn opening_player(&self, query_params: &[(&str, &str)]) -> Result<PlayerOpening> {
        let url = format!("https://explorer.lichess.ovh/player");
        let builder = self.client.get(&url).query(&query_params);
        self.to_model(builder).await
    }
}
