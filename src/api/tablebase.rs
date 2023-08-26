use crate::{client::Licheszter, error::Result, models::tablebase::Endgame};

impl Licheszter {
    /// Search standard tablebase
    pub async fn endgame_standard(&self, fen: &str) -> Result<Endgame> {
        let url = format!("https:://tablebase.lichess.ovh/standard");
        let fen = fen.replace(" ", "_");
        let builder = self.client.get(&url).query(&fen);

        self.to_model::<Endgame>(builder).await
    }

    /// Search atomic tablebase
    pub async fn endgame_atomic(&self, fen: &str) -> Result<Endgame> {
        let url = format!("https://tablebase.lichess.ovh/atomic");
        let fen = fen.replace(" ", "_");
        let builder = self.client.get(&url).query(&fen);

        self.to_model::<Endgame>(builder).await
    }

    /// Search antichess tablebase
    pub async fn endgame_antichess(&self, fen: &str) -> Result<Endgame> {
        let url = format!("https://tablebase.lichess.ovh/antichess");
        let fen = fen.replace(" ", "_");
        let builder = self.client.get(&url).query(&fen);

        self.to_model::<Endgame>(builder).await
    }
}
