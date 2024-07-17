use crate::{
    client::Licheszter,
    error::Result,
    models::{
        board::{BoardState, ChatMessage, ChatRoom},
        common::OkResponse,
    },
};
use futures_util::Stream;

impl Licheszter {
    /// Stream bot game state.
    pub async fn board_stream(
        &self,
        game_id: &str,
    ) -> Result<impl Stream<Item = Result<BoardState>>> {
        let mut url = self.base_url();
        let path = format!("api/{}/game/stream/{game_id}", self.kind());
        url.set_path(&path);
        let builder = self.client.get(url);

        self.to_model_stream::<BoardState>(builder).await
    }

    /// Make a move in a bot game.
    /// The move can also contain a draw offer/agreement.
    pub async fn board_play_move(
        &self,
        game_id: &str,
        uci_move: &str,
        draw_offer: bool,
    ) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/{}/game/{game_id}/move/{uci_move}", self.kind());
        url.set_path(&path);
        let builder = self.client.post(url).query(&[("offeringDraw", draw_offer)]);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Post a message to the player or spectator chat.
    pub async fn chat_write(&self, game_id: &str, room: ChatRoom, text: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/{}/game/{game_id}/chat", self.kind());
        url.set_path(&path);
        let builder = self
            .client
            .post(url)
            .form(&[("room", room.to_string().as_str()), ("text", text)]);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Fetch the messages posted in the game chat.
    pub async fn chat_read(&self, game_id: &str) -> Result<ChatMessage> {
        let mut url = self.base_url();
        let path = format!("api/{}/game/{game_id}/chat", self.kind());
        url.set_path(&path);
        let builder = self.client.get(url);

        self.to_model::<ChatMessage>(builder).await
    }

    /// Abort a bot game.
    pub async fn board_abort(&self, game_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/{}/game/{game_id}/abort", self.kind());
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Resign a bot game.
    pub async fn board_resign(&self, game_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/{}/game/{game_id}/resign", self.kind());
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Create, accept or decline draw offers.
    pub async fn board_handle_draws(&self, game_id: &str, accept: bool) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/{}/game/{game_id}/draw/{accept}", self.kind());
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Create, accept or decline takeback proposals.
    pub async fn board_handle_takebacks(&self, game_id: &str, accept: bool) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/{}/game/{game_id}/takeback/{accept}", self.kind());
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Claim victory when the oppopnent has left the game for a while.
    pub async fn board_claim_victory(&self, game_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/{}/game/{game_id}/claim-victory", self.kind());
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Go berserk on an arena tournament game.
    /// Halves the clock time while granting an extra point upon winning.
    /// Only available in arena tournaments that allow berserk, and before each player has made a move.
    pub async fn board_berserk(&self, game_id: &str) -> Result<()> {
        let mut url = self.base_url();
        let path = format!("api/{}/game/{game_id}/berserk", self.kind());
        url.set_path(&path);
        let builder = self.client.post(url);

        self.to_model::<OkResponse>(builder).await?;
        Ok(())
    }
}
