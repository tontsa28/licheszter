use crate::{
    client::Licheszter,
    error::Result,
    models::{
        board::{BoardState, ChatMessage, ChatRoom},
        common::OkResponse,
        user::BotUser,
    },
};
use futures_util::Stream;

impl Licheszter {
    /// Get online bots.
    pub async fn bots_online(&self, bots: u8) -> Result<impl Stream<Item = Result<BotUser>>> {
        let mut url = self.base_url();
        url.set_path("api/bot/online");
        let builder = self.client.get(url).query(&[("nb", bots)]);

        self.to_model_stream::<BotUser>(builder).await
    }

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
    /// The move can also contain a draw offer/agreement.
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

    /// Post a message to the player or spectator chat.
    pub async fn bot_chat_write(&self, game_id: &str, room: ChatRoom, text: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/{game_id}/chat");
        url.set_path(&path);
        let builder = self
            .client
            .post(url)
            .form(&[("room", room.to_string().as_str()), ("text", text)]);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Fetch the messages posted in the game chat.
    pub async fn bot_chat_read(&self, game_id: &str) -> Result<ChatMessage> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/{game_id}/chat");
        url.set_path(&path);
        let builder = self.client.get(url);

        self.to_model::<ChatMessage>(builder).await
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

    /// Create, accept or decline draw offers.
    pub async fn bot_game_handle_draws(&self, game_id: &str, accept: bool) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/{game_id}/draw/{accept}");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Create, accept or decline takeback proposals.
    pub async fn bot_game_handle_takebacks(&self, game_id: &str, accept: bool) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/bot/game/{game_id}/takeback/{accept}");
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }
}
