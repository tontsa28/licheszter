use crate::{client::Licheszter, error::Result, models::tablebase::Endgame};

impl Licheszter {
    /// Search standard tablebase
    pub async fn endgame_standard(&self, fen: &String) -> Result<Endgame> {
        let addr = format!("https:://tablebase.lichess.ovh/standard");
        let fen = fen.replace(" ", "_");
        let builder = self.client.get(&addr).query(&fen);
        self.to_model_full(builder).await
    }

    /// Search atomic tablebase
    pub async fn endgame_atomic(&self, fen: &String) -> Result<Endgame> {
        let addr = format!("https://tablebase.lichess.ovh/atomic");
        let fen = fen.replace(" ", "_");
        let builder = self.client.get(&addr).query(&fen);
        self.to_model_full(builder).await
    }

    /// Search antichess tablebase
    pub async fn endgame_antichess(&self, fen: &String) -> Result<Endgame> {
        let addr = format!("https://tablebase.lichess.ovh/antichess");
        let fen = fen.replace(" ", "_");
        let builder = self.client.get(&addr).query(&fen);
        self.to_model_full(builder).await
    }
}
