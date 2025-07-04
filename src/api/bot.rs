use std::pin::Pin;

use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::{
        board::BoardState,
        chat::{ChatMessage, ChatRoom},
        common::OkResponse,
    },
};
use futures_util::Stream;

impl Licheszter {
    /// Stream game state using the Bot API.
    pub async fn bot_game_connect(
        &self,
        game_id: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<BoardState>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/stream/{game_id}"));
        let builder = self.client.get(url);

        self.into_stream::<BoardState>(builder).await
    }

    /// Make a move in a game using the Bot API.
    /// The move can also contain a draw offer/agreement.
    pub async fn bot_play_move(&self, game_id: &str, uci_move: &str, draw_offer: bool) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/move/{uci_move}"));
        let builder = self.client.post(url).query(&[("offeringDraw", draw_offer)]);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Post a message to the player or spectator chat using the Bot API.
    pub async fn bot_chat_write(&self, game_id: &str, room: ChatRoom, text: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/chat"));
        let builder = self.client.post(url).form(&(("room", room), ("text", text)));

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Fetch the messages posted in the game chat using the Bot API.
    pub async fn bot_chat_read(&self, game_id: &str) -> Result<Vec<ChatMessage>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/chat"));
        let builder = self.client.get(url);

        self.into::<Vec<ChatMessage>>(builder).await
    }

    /// Abort a bot game using the Bot API.
    pub async fn bot_game_abort(&self, game_id: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/abort"));
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Resign a bot game using the Bot API.
    pub async fn bot_game_resign(&self, game_id: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/resign"));
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Create, accept or decline draw offers using the Bot API.
    pub async fn bot_handle_draws(&self, game_id: &str, accept: bool) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/draw/{accept}"));
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Create, accept or decline takeback proposals using the Bot API.
    pub async fn bot_handle_takebacks(&self, game_id: &str, accept: bool) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/bot/game/{game_id}/takeback/{accept}"));
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }
}
