use crate::{
    client::{Licheszter, UrlBase},
    config::board::SeekOptions,
    error::Result,
    models::{
        board::BoardState,
        chat::{ChatMessage, ChatRoom},
        common::OkResponse,
    },
};
use futures_util::Stream;
use reqwest::header;

impl Licheszter {
    /// Create a public seek to start a game with a random player.
    pub async fn board_seek_create(
        &self,
        options: Option<&SeekOptions>,
    ) -> Result<impl Stream<Item = Result<()>>> {
        let url = self.req_url(UrlBase::Lichess, "api/board/seek");
        let mut builder = self.client.post(url);

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            builder = builder
                .body(encoded)
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        }

        self.into_stream::<()>(builder).await
    }

    /// Stream game state using the Board API.
    pub async fn board_game_connect(
        &self,
        game_id: &str,
    ) -> Result<impl Stream<Item = Result<BoardState>>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/board/game/stream/{game_id}"));
        let builder = self.client.get(url);

        self.into_stream::<BoardState>(builder).await
    }

    /// Make a move in a game using the Board API.
    /// The move can also contain a draw offer/agreement.
    pub async fn board_play_move(&self, game_id: &str, uci_move: &str, draw_offer: bool) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/move/{uci_move}"));
        let builder = self.client.post(url).query(&[("offeringDraw", draw_offer)]);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Post a message to the player or spectator chat using the Board API.
    pub async fn board_chat_write(&self, game_id: &str, room: ChatRoom, text: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/chat"));
        let builder = self.client.post(url).form(&(("room", room), ("text", text)));

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Fetch the messages posted in the game chat using the Board API.
    pub async fn board_chat_read(&self, game_id: &str) -> Result<Vec<ChatMessage>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/chat"));
        let builder = self.client.get(url);

        self.into::<Vec<ChatMessage>>(builder).await
    }

    /// Abort a bot game using the Board API.
    pub async fn board_game_abort(&self, game_id: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/abort"));
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Resign a bot game using the Board API.
    pub async fn board_game_resign(&self, game_id: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/resign"));
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Create, accept or decline draw offers using the Board API.
    pub async fn board_handle_draws(&self, game_id: &str, accept: bool) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/draw/{accept}"));
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Create, accept or decline takeback proposals using the Board API.
    pub async fn board_handle_takebacks(&self, game_id: &str, accept: bool) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/takeback/{accept}"));
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Claim victory when the opponent has left the game for a while using the Board API.
    pub async fn board_claim_victory(&self, game_id: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/claim-victory"));
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Go berserk on an arena tournament game using the Board API.
    /// Halves the clock time while granting an extra point upon winning.
    /// Only available in arena tournaments that allow berserk, and before each player has made a move.
    pub async fn board_berserk(&self, game_id: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/board/game/{game_id}/berserk"));
        let builder = self.client.post(url);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }
}
