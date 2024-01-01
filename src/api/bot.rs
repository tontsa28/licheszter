use crate::{
    client::Licheszter,
    error::Result,
    models::{board::BoardState, common::OkResponse, user::BotUser},
};
use futures_util::Stream;

impl Licheszter {
    /// Stream bot game state.
    pub async fn bot_game_stream(
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
    pub async fn bot_play_move(
        &self,
        game_id: &str,
        uci_move: &str,
        draw_offer: bool,
    ) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/{game_id}/move/{uci_move}");
        url.set_path(&path);
        let builder = self.client.post(url).query(&[("offeringDraw", draw_offer)]);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Write to game chat as a bot.
    pub async fn bot_chat_write(&self, game_id: &str, room: &str, text: &str) -> Result<()> {
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
    pub async fn bot_game_abort(&self, game_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/{game_id}/abort");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Resign a bot game.
    pub async fn bot_game_resign(&self, game_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/{game_id}/resign");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Get online bots.
    pub async fn bots_online(
        &self,
        nb_bots: u8,
    ) -> Result<impl Stream<Item = Result<BotUser>>> {
        let mut url = self.base_url();
        url.set_path("api/bot/online");
        let builder = self.client.get(url).query(&[("nb", nb_bots)]);

        self.to_model_stream::<BotUser>(builder).await
    }
}
