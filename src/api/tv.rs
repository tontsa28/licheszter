use std::pin::Pin;

use futures_util::Stream;
use reqwest::header;

use crate::{
    client::{Licheszter, UrlBase},
    config::tv::{TvChannel, TvChannelOptions},
    error::Result,
    models::{
        game::Game,
        tv::{TvGameEvent, TvGames},
    },
};

impl Licheszter {
    /// Get basic information about the TV games for each speed and variant, including computer and bot games.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn tv_games(&self) -> Result<TvGames> {
        let url = self.req_url(UrlBase::Lichess, "api/tv/channels");
        let builder = self.client.get(url);

        self.to_model::<TvGames>(builder).await
    }

    /// Stream positions and moves of the current TV game.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn tv_connect(&self) -> Result<Pin<Box<dyn Stream<Item = Result<TvGameEvent>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, "api/tv/feed");
        let builder = self.client.get(url);

        self.to_stream::<TvGameEvent>(builder).await
    }

    /// Stream positions and moves of a current TV channel's game.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn tv_channel_connect(
        &self,
        channel: TvChannel,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<TvGameEvent>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, &format!("api/tv/{channel}/feed"));
        let builder = self.client.get(url);

        self.to_stream::<TvGameEvent>(builder).await
    }

    /// Get a list of ongoing games for a given TV channel.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn tv_channel_games(
        &self,
        channel: TvChannel,
        options: Option<&TvChannelOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Game>> + Send>>> {
        let mut url = self.req_url(UrlBase::Lichess, &format!("api/tv/{channel}"));

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.client.get(url).header(header::ACCEPT, "application/json");
        self.to_stream::<Game>(builder).await
    }
}
