use crate::{
    client::{LicheszterInner, UrlBase},
    error::Result,
    models::tablebase::Endgame,
};

use std::sync::Arc;

/// A struct for accessing the Tablebase API endpoints.
#[derive(Debug)]
pub struct TablebaseApi {
    pub(crate) inner: Arc<LicheszterInner>,
}

impl TablebaseApi {
    /// Lookup positions from the standard endgame tablebase.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn standard(&self, fen: &str) -> Result<Endgame> {
        let url = self.inner.req_url(UrlBase::Tablebase, "standard");
        let fen = fen.replace(' ', "_");
        let builder = self.inner.client.get(url).query(&[("fen", &fen)]);

        self.inner.to_model::<Endgame>(builder).await
    }

    /// Lookup positions from the atomic endgame tablebase.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn atomic(&self, fen: &str) -> Result<Endgame> {
        let url = self.inner.req_url(UrlBase::Tablebase, "atomic");
        let fen = fen.replace(' ', "_");
        let builder = self.inner.client.get(url).query(&[("fen", &fen)]);

        self.inner.to_model::<Endgame>(builder).await
    }

    /// Lookup positions from the antichess endgame tablebase.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn antichess(&self, fen: &str) -> Result<Endgame> {
        let url = self.inner.req_url(UrlBase::Tablebase, "antichess");
        let fen = fen.replace(' ', "_");
        let builder = self.inner.client.get(url).query(&[("fen", &fen)]);

        self.inner.to_model::<Endgame>(builder).await
    }
}
