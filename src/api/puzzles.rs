use std::pin::Pin;

use futures_util::Stream;

use crate::{
    client::{LicheszterInner, UrlBase},
    config::puzzles::PuzzleDifficulty,
    error::Result,
    models::puzzle::{Puzzle, PuzzleActivity, PuzzleDashboard, PuzzleRace, PuzzleStormDashboard},
};

use std::sync::Arc;

/// A struct for accessing the Puzzles API endpoints.
#[derive(Debug)]
pub struct PuzzlesApi {
    pub(crate) inner: Arc<LicheszterInner>,
}

impl PuzzlesApi {
    /// Get the daily puzzle.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn daily(&self) -> Result<Puzzle> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/puzzle/daily");
        let builder = self.inner.client.get(url);

        self.inner.to_model::<Puzzle>(builder).await
    }

    /// Get a single puzzle by ID.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn show(&self, id: &str) -> Result<Puzzle> {
        let url = self.inner.req_url(UrlBase::Lichess, &format!("api/puzzle/{id}"));
        let builder = self.inner.client.get(url);

        self.inner.to_model::<Puzzle>(builder).await
    }

    /// Get a random puzzle.
    /// If authenticated, only returns puzzles the user has never seen before.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn next(&self, angle: Option<&str>, difficulty: Option<PuzzleDifficulty>) -> Result<Puzzle> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/puzzle/next");
        let builder = self.inner.client.get(url).query(&(("angle", angle), ("difficulty", difficulty)));

        self.inner.to_model::<Puzzle>(builder).await
    }

    /// Get the puzzle activity of the logged in user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn activity(&self, max: Option<u16>, before: Option<u64>) -> Result<Pin<Box<dyn Stream<Item = Result<PuzzleActivity>> + Send>>> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/puzzle/activity");
        let builder = self.inner.client.get(url).query(&(("max", max), ("before", before)));

        self.inner.to_stream::<PuzzleActivity>(builder).await
    }

    /// Get the puzzle dashboard of the logged in user.
    /// Includes all puzzle themes played, with aggregated results.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn dashboard(&self, days: u8) -> Result<PuzzleDashboard> {
        let url = self.inner.req_url(UrlBase::Lichess, &format!("api/puzzle/dashboard/{days}"));
        let builder = self.inner.client.get(url);

        self.inner.to_model::<PuzzleDashboard>(builder).await
    }

    /// Get the puzzle storm dashboard of any player.
    /// Contains the aggregated highscores and the history of storm runs aggregated by days.
    /// Use `days = 0` if you only care about the highscores.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn dashboard_storm(&self, username: &str, days: Option<u16>) -> Result<PuzzleStormDashboard> {
        let url = self.inner.req_url(UrlBase::Lichess, &format!("api/storm/dashboard/{username}"));
        let builder = self.inner.client.get(url).query(&[("days", days)]);

        self.inner.to_model::<PuzzleStormDashboard>(builder).await
    }

    /// Create a new private puzzle race.
    /// Once the puzzle race has been created, the creator must join the page and manually start the race when enough players have joined.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn race_create(&self) -> Result<PuzzleRace> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/racer");
        let builder = self.inner.client.post(url);

        self.inner.to_model::<PuzzleRace>(builder).await
    }
}
