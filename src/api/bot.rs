use crate::{
    client::Licheszter,
    error::Result,
    models::{board::BoardState, common::OkResponse, user::BotUser},
};
use futures_util::Stream;

impl Licheszter {
    /// Stream bot game state.
    pub async fn stream_bot_game_state(
        &self,
        game_id: &str,
    ) -> Result<impl Stream<Item = Result<BoardState>>> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/stream/{game_id}");
        url.set_path(&path);
        let builder = self.client.get(url);

        self.to_model_stream::<BoardState>(builder).await
    }

    /// Make a move in a bot game.
    pub async fn make_bot_move(
        &self,
        game_id: &str,
        uci_move: &str,
        draw_offer: bool,
    ) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/{game_id}/move/{uci_move}");
        url.set_path(&path);
        let builder = self
            .client
            .post(url)
            .query(&[("offeringDraw", draw_offer)]);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Write to game chat as a bot.
    pub async fn write_to_bot_chat(&self, game_id: &str, room: &str, text: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/{game_id}/chat");
        url.set_path(&path);
        let builder = self
            .client
            .post(url)
            .form(&[("room", room), ("text", text)]);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Abort a bot game.
    pub async fn abort_bot_game(&self, game_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/{game_id}/abort");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Resign a bot game.
    pub async fn resign_bot_game(&self, game_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/{game_id}/resign");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Get online bots.
    pub async fn get_online_bots(
        &self,
        nb_bots: u8,
    ) -> Result<impl Stream<Item = Result<BotUser>>> {
        let mut url = self.base_url();
        url.set_path("api/bot/online");
        let builder = self.client.get(url).query(&[("nb", nb_bots)]);

        self.to_model_stream::<BotUser>(builder).await
    }
}
