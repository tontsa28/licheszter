use std::pin::Pin;

use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::{
        board::BoardState,
        chat::{ChatMessage, ChatRoom},
    },
};
use futures_util::Stream;

impl Licheszter {
    /// Stream game state using the Bot API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn bot_game_connect(
        &self,
        game_id: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<BoardState>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/stream/{game_id}"));
        let builder = self.client.get(url);

        self.to_stream::<BoardState>(builder).await
    }

    /// Make a move in a game using the Bot API.
    /// The move can also contain a draw offer/agreement.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn bot_play_move(&self, game_id: &str, uci_move: &str, draw_offer: bool) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/move/{uci_move}"));
        let builder = self.client.post(url).query(&[("offeringDraw", draw_offer)]);

        self.execute(builder).await
    }

    /// Post a message to the player or spectator chat using the Bot API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn bot_chat_write(&self, game_id: &str, room: ChatRoom, text: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/chat"));
        let builder = self.client.post(url).form(&(("room", room), ("text", text)));

        self.execute(builder).await
    }

    /// Fetch the messages posted in the game chat using the Bot API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn bot_chat_read(&self, game_id: &str) -> Result<Vec<ChatMessage>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/chat"));
        let builder = self.client.get(url);

        self.to_model::<Vec<ChatMessage>>(builder).await
    }

    /// Abort a bot game using the Bot API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn bot_game_abort(&self, game_id: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/abort"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }

    /// Resign a bot game using the Bot API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn bot_game_resign(&self, game_id: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/resign"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }

    /// Create, accept or decline draw offers using the Bot API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn bot_handle_draws(&self, game_id: &str, accept: bool) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/draw/{accept}"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }

    /// Create, accept or decline takeback proposals using the Bot API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn bot_handle_takebacks(&self, game_id: &str, accept: bool) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/takeback/{accept}"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }

    /// Claim victory when the opponent has left the game for a while using the Bot API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn bot_claim_victory(&self, game_id: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/claim-victory"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }

    /// Claim draw when the opponent has left the game for a while using the Bot API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn bot_claim_draw(&self, game_id: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/claim-draw"));
        let builder = self.client.post(url);

        self.execute(builder).await
    }
}
