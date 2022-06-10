use serde_json::{from_value, Value};
use crate::client::{Licheszter, LicheszterResult};

// Implement bot functions for Licheszter
impl Licheszter {
    /// Make a move in a bot game
    pub async fn make_move(&self, game_id: &str, uci_move: &str, draw_offer: bool) -> LicheszterResult<()> {
        let addr = format!("{}/api/bot/game/{}/move/{}", self.base, game_id, uci_move);
        let builder = self.client.post(&addr).query(&[("offeringDraw", draw_offer)]);
        let ok_json = self.to_model_full::<Value>(builder);
        assert!(from_value::<bool>(ok_json.await?["ok"].take())?);
        Ok(())
    }
}