use std::pin::Pin;

use futures_util::Stream;
use reqwest::header;

use crate::{
    client::{Licheszter, UrlBase},
    config::games::{ExtendedGameOptions, GameOptions},
    error::Result,
    models::{
        common::OkResponse,
        game::{Game, StreamGame, UserGame, UserGames},
    },
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
        self.into::<Game>(builder).await
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
        self.into::<Game>(builder).await
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
        self.into_stream::<Game>(builder).await
    }

    /// Download games by IDs.
    /// Games are delivered in reverse chronological order (most recent first).
    /// Up to 300 game IDs can be submitted at a time.
    /// Ongoing games are delayed by a few seconds ranging from 3 to 60 depending on the time control to prevent cheat bots from using this endpoint.
    pub async fn games_export(
        &self,
        game_ids: Vec<&str>,
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
        self.into_stream::<Game>(builder).await
    }

    /// Stream the games played between a list of users in real time.
    /// Only games where both players are part of the list are included.
    /// The stream emits an event each time a game is started or finished.
    /// To get all current ongoing games at the beginning of the stream, use the `with_current_games` option.
    /// Up to 300 users can be listed.
    pub async fn games_users_connect(
        &self,
        user_ids: Vec<&str>,
        with_current_games: bool,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamGame>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, "api/stream/games-by-users");
        let builder = self
            .client
            .post(url)
            .query(&[("withCurrentGames", with_current_games)])
            .body(user_ids.join(","));

        self.into_stream::<StreamGame>(builder).await
    }

    /// Create a stream of games with a custom ID.
    /// The stream first outputs the games that already exist, then emits an event each time a game is started or finished.
    /// Up to 500 games for anonymous requests or 1000 games for authenticated requests can be streamed at a time.
    /// It is possible to add new games to the stream while it is open using [`games_connect_add`](fn@Licheszter::games_connect_add).
    pub async fn games_connect(
        &self,
        stream_id: &str,
        game_ids: Vec<&str>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamGame>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/stream/games/{stream_id}"));
        let builder = self.client.post(url).body(game_ids.join(","));

        self.into_stream::<StreamGame>(builder).await
    }

    /// Add new games to an existing stream.
    /// The stream will immediately output the games that already exist, then emit an event each time a game is started or finished.
    pub async fn games_connect_add(&self, stream_id: &str, game_ids: Vec<&str>) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/stream/games/{stream_id}/add"));
        let builder = self.client.post(url).body(game_ids.join(","));

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Get the ongoing games of the current user.
    /// The most urgent games are listed first.
    pub async fn games_ongoing(&self, games: u8) -> Result<Vec<UserGame>> {
        let url = self.req_url(UrlBase::Lichess, "api/account/playing");
        let builder = self.client.get(url).query(&[("nb", games)]);

        Ok(self.into::<UserGames>(builder).await?.now_playing)
    }
}
