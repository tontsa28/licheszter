use crate::{
    client::Licheszter,
    error::Result,
    models::{board::BoardState, user::BotUser},
};
use futures_util::Stream;
use serde_json::Value;

impl Licheszter {
    /// Stream bot game state.
    pub async fn stream_game_state(
        &self,
        game_id: &str,
    ) -> Result<impl Stream<Item = Result<BoardState>>> {
        let url = format!("{}/api/bot/game/stream/{}", self.base, game_id);
        let builder = self.client.get(&url);

        self.to_model_stream::<BoardState>(builder).await
    }

    /// Make a move in a bot game.
    pub async fn make_move(&self, game_id: &str, uci_move: &str, draw_offer: bool) -> Result<()> {
        let url = format!("{}/api/bot/game/{}/move/{}", self.base, game_id, uci_move);
        let builder = self
            .client
            .post(&url)
            .query(&[("offeringDraw", draw_offer)]);

        self.to_model::<Value>(builder).await?;
        Ok(())
    }

    /// Write to game chat as a bot.
    pub async fn write_to_chat(&self, game_id: &str, room: &str, text: &str) -> Result<()> {
        let url = format!("{}/api/bot/game/{}/chat", self.base, game_id);
        let builder = self
            .client
            .post(&url)
            .form(&[("room", room), ("text", text)]);

        self.to_model::<Value>(builder).await?;
        Ok(())
    }

    /// Abort a bot game.
    pub async fn abort_game(&self, game_id: &str) -> Result<()> {
        let url = format!("{}/api/bot/game/{}/abort", self.base, game_id);
        let builder = self.client.post(&url);

        self.to_model::<Value>(builder).await?;
        Ok(())
    }

    /// Resign a bot game.
    pub async fn resign_game(&self, game_id: &str) -> Result<()> {
        let url = format!("{}/api/bot/game/{}/resign", self.base, game_id);
        let builder = self.client.post(&url);

        self.to_model::<Value>(builder).await?;
        Ok(())
    }

    /// Get online bots.
    pub async fn get_online_bots(
        &self,
        nb_bots: u8,
    ) -> Result<impl Stream<Item = Result<BotUser>>> {
        let url = format!("{}/api/bot/online", self.base);
        let builder = self.client.get(&url).query(&[("nb", nb_bots)]);

        self.to_model_stream::<BotUser>(builder).await
    }
}
