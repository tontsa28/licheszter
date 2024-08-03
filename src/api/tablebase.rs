use crate::{client::Licheszter, error::Result, models::tablebase::Endgame};

impl Licheszter {
    /// Lookup positions from the standard endgame tablebase.
    pub async fn tablebase_standard(&self, fen: &str) -> Result<Endgame> {
        let mut url = self.tablebase_url();
        url.set_path("standard");
        let fen = fen.replace(' ', "_");
        let builder = self.client.get(url).query(&[("fen", &fen)]);

        self.to_model::<Endgame>(builder).await
    }

    /// Lookup positions from the atomic endgame tablebase.
    pub async fn tablebase_atomic(&self, fen: &str) -> Result<Endgame> {
        let mut url = self.tablebase_url();
        url.set_path("atomic");
        let fen = fen.replace(' ', "_");
        let builder = self.client.get(url).query(&[("fen", &fen)]);

        self.to_model::<Endgame>(builder).await
    }

    /// Lookup positions from the antichess endgame tablebase.
    pub async fn tablebase_antichess(&self, fen: &str) -> Result<Endgame> {
        let mut url = self.tablebase_url();
        url.set_path("antichess");
        let fen = fen.replace(' ', "_");
        let builder = self.client.get(url).query(&[("fen", &fen)]);

        self.to_model::<Endgame>(builder).await
    }
}
