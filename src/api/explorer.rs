use futures_util::Stream;

use crate::{
    client::Licheszter,
    error::Result,
    models::explorer::{Opening, PlayerOpening},
};

impl Licheszter {
    /// Search the Masters opening database.
    pub async fn opening_masters(&self, query_params: Option<&[(&str, &str)]>) -> Result<Opening> {
        let url = format!("https://explorer.lichess.ovh/masters");
        let mut builder = self.client.get(&url);

        if let Some(params) = query_params {
            builder = builder.query(&params);
        }

        self.to_model::<Opening>(builder).await
    }

    /// Search the Lichess opening database.
    pub async fn opening_lichess(&self, query_params: Option<&[(&str, &str)]>) -> Result<Opening> {
        let url = format!("https://explorer.lichess.ovh/lichess");
        let mut builder = self.client.get(&url);

        if let Some(params) = query_params {
            builder = builder.query(&params);
        }

        self.to_model::<Opening>(builder).await
    }

    /// Search the player opening database.
    pub async fn opening_player(
        &self,
        query_params: Option<&[(&str, &str)]>,
    ) -> Result<impl Stream<Item = Result<PlayerOpening>>> {
        let url = format!("https://explorer.lichess.ovh/player");
        let mut builder = self.client.get(&url);

        if let Some(params) = query_params {
            builder = builder.query(&params);
        }

        self.to_model_stream::<PlayerOpening>(builder).await
    }
}
