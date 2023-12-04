use crate::{
    client::Licheszter,
    error::Result,
    models::explorer::{Opening, PlayerOpening},
};
use futures_util::Stream;

impl Licheszter {
    /// Search the Masters opening database.
    pub async fn opening_masters(&self, query_params: Option<&[(&str, &str)]>) -> Result<Opening> {
        let mut url = self.explorer_url();
        url.set_path("masters");
        let mut builder = self.client.get(url);

        if let Some(params) = query_params {
            builder = builder.query(&params);
        }

        self.to_model::<Opening>(builder).await
    }

    /// Search the Lichess opening database.
    pub async fn opening_lichess(&self, query_params: Option<&[(&str, &str)]>) -> Result<Opening> {
        let mut url = self.explorer_url();
        url.set_path("lichess");
        let mut builder = self.client.get(url);

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
        let mut url = self.explorer_url();
        url.set_path("player");
        let mut builder = self.client.get(url);

        if let Some(params) = query_params {
            builder = builder.query(&params);
        }

        self.to_model_stream::<PlayerOpening>(builder).await
    }
}
