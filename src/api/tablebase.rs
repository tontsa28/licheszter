use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::tablebase::Endgame,
};

impl Licheszter {
    /// Lookup positions from the standard endgame tablebase.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn tablebase_standard(&self, fen: &str) -> Result<Endgame> {
        let url = self.req_url(UrlBase::Tablebase, "standard");
        let fen = fen.replace(' ', "_");
        let builder = self.client.get(url).query(&[("fen", &fen)]);

        self.to_model::<Endgame>(builder).await
    }

    /// Lookup positions from the atomic endgame tablebase.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn tablebase_atomic(&self, fen: &str) -> Result<Endgame> {
        let url = self.req_url(UrlBase::Tablebase, "atomic");
        let fen = fen.replace(' ', "_");
        let builder = self.client.get(url).query(&[("fen", &fen)]);

        self.to_model::<Endgame>(builder).await
    }

    /// Lookup positions from the antichess endgame tablebase.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn tablebase_antichess(&self, fen: &str) -> Result<Endgame> {
        let url = self.req_url(UrlBase::Tablebase, "antichess");
        let fen = fen.replace(' ', "_");
        let builder = self.client.get(url).query(&[("fen", &fen)]);

        self.to_model::<Endgame>(builder).await
    }
}
