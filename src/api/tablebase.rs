use crate::{client::Licheszter, error::Result, models::tablebase::Endgame};

impl Licheszter {
    /// Search standard tablebase
    pub async fn endgame_standard(&self, fen: &str) -> Result<Endgame> {
        let mut url = self.tablebase_url();
        url.set_path("standard");
        let fen = fen.replace(" ", "_");
        let builder = self.client.get(url).query(&vec![("fen", &fen)]);

        self.to_model::<Endgame>(builder).await
    }

    /// Search atomic tablebase
    pub async fn endgame_atomic(&self, fen: &str) -> Result<Endgame> {
        let mut url = self.tablebase_url();
        url.set_path("atomic");
        let fen = fen.replace(" ", "_");
        let builder = self.client.get(url).query(&vec![("fen", &fen)]);

        self.to_model::<Endgame>(builder).await
    }

    /// Search antichess tablebase
    pub async fn endgame_antichess(&self, fen: &str) -> Result<Endgame> {
        let mut url = self.tablebase_url();
        url.set_path("antichess");
        let fen = fen.replace(" ", "_");
        let builder = self.client.get(url).query(&vec![("fen", &fen)]);

        self.to_model::<Endgame>(builder).await
    }
}
