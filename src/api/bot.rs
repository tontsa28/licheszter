use futures_util::Stream;
use serde_json::{from_value, Value};
use crate::{client::{Licheszter, LicheszterResult}, models::{board::BoardState, user::BotUser}};

impl Licheszter {
    /// Stream bot game state.
    pub async fn stream_game_state(&self, game_id: &str) -> LicheszterResult<impl Stream<Item = LicheszterResult<BoardState>>> {
        let addr = format!("{}/api/bot/game/stream/{}", self.base, game_id);
        let builder = self.client.get(&addr);
        self.to_model_stream(builder).await
    }

    /// Make a move in a bot game.
    pub async fn make_move(&self, game_id: &str, uci_move: &str, draw_offer: bool) -> LicheszterResult<()> {
        let addr = format!("{}/api/bot/game/{}/move/{}", self.base, game_id, uci_move);
        let builder = self.client.post(&addr).query(&[("offeringDraw", draw_offer)]);
        let ok_json = self.to_model_full::<Value>(builder);
        assert!(from_value::<bool>(ok_json.await?["ok"].take())?);
        Ok(())
    }

    /// Write to game chat as a bot.
    pub async fn write_to_chat(&self, game_id: &str, room: &str, text: &str) -> LicheszterResult<()> {
        let addr = format!("{}/api/bot/game/{}/chat", self.base, game_id);
        let builder = self.client.post(&addr).form(&[("room", room), ("text", text)]);
        let ok_json = self.to_model_full::<Value>(builder);
        assert!(from_value::<bool>(ok_json.await?["ok"].take())?);
        Ok(())
    }

    /// Abort a bot game.
    pub async fn abort_game(&self, game_id: &str) -> LicheszterResult<()> {
        let addr = format!("{}/api/bot/game/{}/abort", self.base, game_id);
        let builder = self.client.post(&addr);
        let ok_json = self.to_model_full::<Value>(builder);
        assert!(from_value::<bool>(ok_json.await?["ok"].take())?);
        Ok(())
    }

    /// Resign a bot game.
    pub async fn resign_game(&self, game_id: &str) -> LicheszterResult<()> {
        let addr = format!("{}/api/bot/game/{}/resign", self.base, game_id);
        let builder = self.client.post(&addr);
        let ok_json = self.to_model_full::<Value>(builder);
        assert!(from_value::<bool>(ok_json.await?["ok"].take())?);
        Ok(())
    }

    /// Get online bots.
    pub async fn get_online_bots(&self, nb_bots: u8) -> LicheszterResult<impl Stream<Item = LicheszterResult<BotUser>>> {
        let addr = format!("{}/api/bot/online", self.base);
        let builder = self.client.get(&addr).query(&[("nb", nb_bots)]);
        self.to_model_stream(builder).await
    }
}