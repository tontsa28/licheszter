use std::pin::Pin;

use futures_util::Stream;
use reqwest::header;

use crate::{
    client::{Licheszter, UrlBase},
    config::games::{BookmarkedGameOptions, ExtendedGameOptions, GameOptions},
    error::Result,
    models::game::{Game, ImportGame, StreamGame, StreamMoves, UserGame, UserGames},
};

impl Licheszter {
    /// Download one game.
    /// Ongoing games are delayed by a few seconds ranging from 3 to 60 depending on the time control to prevent cheat bots from using this endpoint.
    pub async fn games_export_one(&self, game_id: &str, options: Option<&GameOptions>) -> Result<Game> {
        let mut url = self.req_url(UrlBase::Lichess, &format!("game/export/{game_id}"));

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.client.get(url).header(header::ACCEPT, "application/json");
        self.to_model::<Game>(builder).await
    }

    /// Download the ongoing game, or the last game played, of a user.
    /// Ongoing games are delayed by a few seconds ranging from 3 to 60 depending on the time control to prevent cheat bots from using this endpoint.
    pub async fn games_export_ongoing_user(
        &self,
        username: &str,
        options: Option<&GameOptions>,
    ) -> Result<Game> {
        let mut url = self.req_url(UrlBase::Lichess, &format!("api/user/{username}/current-game"));

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.client.get(url).header(header::ACCEPT, "application/json");
        self.to_model::<Game>(builder).await
    }

    /// Download all games of any user.
    /// By default, games are delivered in reverse chronological order (most recent first).
    pub async fn games_export_user(
        &self,
        username: &str,
        options: Option<&ExtendedGameOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Game>> + Send>>> {
        let mut url = self.req_url(UrlBase::Lichess, &format!("api/games/user/{username}"));

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self
            .client
            .get(url)
            .header(header::ACCEPT, "application/x-ndjson");
        self.to_stream::<Game>(builder).await
    }

    /// Download games by IDs.
    /// Games are delivered in reverse chronological order (most recent first).
    /// Up to 300 game IDs can be submitted at a time.
    /// Ongoing games are delayed by a few seconds ranging from 3 to 60 depending on the time control to prevent cheat bots from using this endpoint.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn games_export(
        &self,
        game_ids: &[&str],
        options: Option<&GameOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Game>> + Send>>> {
        let mut url = self.req_url(UrlBase::Lichess, "api/games/export/_ids");

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self
            .client
            .post(url)
            .header(header::ACCEPT, "application/x-ndjson")
            .body(game_ids.join(","));
        self.to_stream::<Game>(builder).await
    }

    /// Stream the games played between a list of users in real time.
    /// Only games where both players are part of the list are included.
    /// The stream emits an event each time a game is started or finished.
    /// To get all current ongoing games at the beginning of the stream, use the `with_current_games` option.
    /// Up to 300 users can be listed.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn games_users_connect(
        &self,
        user_ids: &[&str],
        with_current_games: bool,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamGame>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, "api/stream/games-by-users");
        let builder = self
            .client
            .post(url)
            .query(&[("withCurrentGames", with_current_games)])
            .body(user_ids.join(","));

        self.to_stream::<StreamGame>(builder).await
    }

    /// Create a stream of games with a custom ID.
    /// The stream first outputs the games that already exist, then emits an event each time a game is started or finished.
    /// Up to 500 games using anonymous requests or 1000 games using authenticated requests can be streamed at a time.
    /// It is possible to add new games to the stream while it is open using [`games_connect_add`](fn@Licheszter::games_connect_add).
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn games_connect(
        &self,
        stream_id: &str,
        game_ids: &[&str],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamGame>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/stream/games/{stream_id}"));
        let builder = self.client.post(url).body(game_ids.join(","));

        self.to_stream::<StreamGame>(builder).await
    }

    /// Add new games to an existing stream.
    /// The stream will immediately output the games that already exist, then emit an event each time a game is started or finished.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn games_connect_add(&self, stream_id: &str, game_ids: &[&str]) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/stream/games/{stream_id}/add"));
        let builder = self.client.post(url).body(game_ids.join(","));

        self.execute(builder).await
    }

    /// Get the ongoing games of the current user.
    /// The most urgent games are listed first.
    pub async fn games_ongoing(&self, games: u8) -> Result<Vec<UserGame>> {
        let url = self.req_url(UrlBase::Lichess, "api/account/playing");
        let builder = self.client.get(url).query(&[("nb", games)]);

        Ok(self.to_model::<UserGames>(builder).await?.now_playing)
    }

    /// Stream positions and moves of any ongoing game.
    /// A description of the game is sent first.
    /// Then, an update is sent each time a move is played.
    /// Finally, a description is sent once the game is finished and the stream is closed.
    /// Ongoing games are delayed by a few seconds ranging from 3 to 60 depending on the time control to prevent cheat bots from using this endpoint.
    /// A maximum of 8 game streams can be opened from the same IP address at the same time.
    pub async fn games_moves_connect(
        &self,
        game_id: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamMoves>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/stream/game/{game_id}"));
        let builder = self.client.get(url);

        self.to_stream::<StreamMoves>(builder).await
    }

    /// Import a game from PGN.
    /// Up to 100 games using anonymous requests or 200 games using authenticated requests can be imported hourly.
    /// To broadcast ongoing games, consider pushing to a broadcast instead.
    pub async fn games_import_one(&self, pgn: &str) -> Result<ImportGame> {
        let url = self.req_url(UrlBase::Lichess, "api/import");
        let builder = self.client.post(url).form(&[("pgn", pgn)]);

        self.to_model::<ImportGame>(builder).await
    }

    /// Download all games imported by you.
    /// Games are exported in PGN format.
    /// # NOTE:
    /// This method does NOT deserialize the PGN data, it must be manually parsed.
    pub async fn games_export_imported(&self) -> Result<String> {
        let url = self.req_url(UrlBase::Lichess, "api/games/export/imports");
        let builder = self.client.get(url);

        self.to_string(builder).await
    }

    /// Download all games bookmarked by you.
    /// By default, games are delivered in reverse chronological order (most recent first).
    pub async fn games_export_bookmarked(
        &self,
        options: Option<&BookmarkedGameOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Game>> + Send>>> {
        let mut url = self.req_url(UrlBase::Lichess, "api/games/export/bookmarks");

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self
            .client
            .get(url)
            .header(header::ACCEPT, "application/x-ndjson");
        self.to_stream::<Game>(builder).await
    }
}
