use std::pin::Pin;

use crate::{
    client::{LicheszterInner, UrlBase},
    config::board::SeekOptions,
    error::Result,
    models::{
        board::BoardState,
        chat::{ChatMessage, ChatRoom},
    },
};

use futures_util::Stream;
use reqwest::header;
use std::sync::Arc;

/// A struct for accessing the Board API endpoints.
#[derive(Debug)]
pub struct BoardApi {
    pub(crate) inner: Arc<LicheszterInner>,
}

impl BoardApi {
    /// Create a public seek to start a game with a random player.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn seek_create(
        &self,
        options: Option<&SeekOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<()>> + Send>>> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/board/seek");
        let mut builder = self.inner.client.post(url);

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            builder = builder
                .body(encoded)
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        }

        self.inner.to_stream::<()>(builder).await
    }

    /// Stream game state using the Board API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn game_connect(
        &self,
        game_id: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<BoardState>> + Send>>> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/board/game/stream/{game_id}"));
        let builder = self.inner.client.get(url);

        self.inner.to_stream::<BoardState>(builder).await
    }

    /// Make a move in a game using the Board API.
    /// The move can also contain a draw offer/agreement.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn play_move(&self, game_id: &str, uci_move: &str, draw_offer: bool) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/move/{uci_move}"));
        let builder = self.inner.client.post(url).query(&[("offeringDraw", draw_offer)]);

        self.inner.execute(builder).await
    }

    /// Post a message to the player or spectator chat using the Board API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn chat_write(&self, game_id: &str, room: ChatRoom, text: &str) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/chat"));
        let builder = self
            .inner
            .client
            .post(url)
            .form(&(("room", room), ("text", text)));

        self.inner.execute(builder).await
    }

    /// Fetch the messages posted in the game chat using the Board API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn chat_read(&self, game_id: &str) -> Result<Vec<ChatMessage>> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/chat"));
        let builder = self.inner.client.get(url);

        self.inner.to_model::<Vec<ChatMessage>>(builder).await
    }

    /// Abort a bot game using the Board API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn game_abort(&self, game_id: &str) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/abort"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }

    /// Resign a bot game using the Board API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn game_resign(&self, game_id: &str) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/resign"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }

    /// Create, accept or decline draw offers using the Board API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn handle_draws(&self, game_id: &str, accept: bool) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/draw/{accept}"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }

    /// Create, accept or decline takeback proposals using the Board API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn handle_takebacks(&self, game_id: &str, accept: bool) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/takeback/{accept}"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }

    /// Claim victory when the opponent has left the game for a while using the Board API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn claim_victory(&self, game_id: &str) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/claim-victory"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }

    /// Claim draw when the opponent has left the game for a while using the Board API.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn claim_draw(&self, game_id: &str) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/claim-draw"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }

    /// Go berserk on an arena tournament game using the Board API.
    /// Halves the clock time while granting an extra point upon winning.
    /// Only available in arena tournaments that allow berserk, and before each player has made a move.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn berserk(&self, game_id: &str) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/berserk"));
        let builder = self.inner.client.post(url);

        self.inner.execute(builder).await
    }
}
