use std::pin::Pin;

use crate::{
    client::{Licheszter, UrlBase},
    config::openings::{LichessOpeningsOptions, MastersOpeningsOptions, PlayerOpeningsOptions},
    error::Result,
    models::{
        game::Color,
        openings::{Opening, PlayerOpening},
    },
};
use futures_util::Stream;

impl Licheszter {
    /// Lookup positions from the Masters opening database.
    pub async fn openings_masters(&self, options: Option<&MastersOpeningsOptions>) -> Result<Opening> {
        let mut url = self.req_url(UrlBase::Openings, "masters");

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.client.get(url);
        self.to_model::<Opening>(builder).await
    }

    /// Lookup positions from the Lichess opening database.
    pub async fn openings_lichess(&self, options: Option<&LichessOpeningsOptions>) -> Result<Opening> {
        let mut url = self.req_url(UrlBase::Openings, "lichess");

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.client.get(url);
        self.to_model::<Opening>(builder).await
    }

    /// Lookup positions from the Player opening database.
    pub async fn openings_player(
        &self,
        player: &str,
        color: Color,
        options: Option<&PlayerOpeningsOptions>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<PlayerOpening>> + Send>>> {
        let mut url = self.req_url(UrlBase::Openings, "player");
        let encoded = comma_serde_urlencoded::to_string((("player", player), ("color", color)))?;
        url.set_query(Some(&encoded));

        // Add the options to the request if they are present
        if let Some(options) = options {
            let encoded = encoded + "&" + &comma_serde_urlencoded::to_string(options)?;
            url.set_query(Some(&encoded));
        }

        let builder = self.client.get(url);
        self.to_stream::<PlayerOpening>(builder).await
    }

    /// Get an OTB (over the board) master game in PGN format.
    /// # NOTE:
    /// This method does NOT deserialize the PGN data, it must be manually parsed.
    pub async fn openings_masters_otb_game(&self, game_id: &str) -> Result<String> {
        let url = self.req_url(UrlBase::Openings, &format!("master/pgn/{game_id}"));
        let builder = self.client.get(url);

        self.to_string(builder).await
    }
}
