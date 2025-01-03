use futures_util::Stream;

use crate::{
    client::Licheszter,
    config::puzzles::PuzzleDifficulty,
    error::Result,
    models::puzzle::{Puzzle, PuzzleActivity, PuzzleDashboard, PuzzleRace, PuzzleStormDashboard},
};

impl Licheszter {
    /// Get the daily puzzle.
    pub async fn puzzle_daily(&self) -> Result<Puzzle> {
        let mut url = self.base_url.clone();
        url.set_path("api/puzzle/daily");
        let builder = self.client.get(url);

        self.into::<Puzzle>(builder).await
    }

    /// Get a single puzzle by ID.
    pub async fn puzzle_show(&self, id: &str) -> Result<Puzzle> {
        let mut url = self.base_url.clone();
        let path = format!("api/puzzle/{id}");
        url.set_path(&path);
        let builder = self.client.get(url);

        self.into::<Puzzle>(builder).await
    }

    /// Get a random puzzle.
    /// If authenticated, only returns puzzles the user has never seen before.
    pub async fn puzzle_next(
        &self,
        angle: Option<&str>,
        difficulty: Option<PuzzleDifficulty>,
    ) -> Result<Puzzle> {
        let mut url = self.base_url.clone();
        url.set_path("api/puzzle/next");
        let builder = self
            .client
            .get(url)
            .query(&(("angle", angle), ("difficulty", difficulty)));

        self.into::<Puzzle>(builder).await
    }

    /// Get the puzzle activity of the logged in user.
    pub async fn puzzle_activity(
        &self,
        max: Option<u16>,
        before: Option<u64>,
    ) -> Result<impl Stream<Item = Result<PuzzleActivity>>> {
        let mut url = self.base_url.clone();
        url.set_path("api/puzzle/activity");
        let builder = self
            .client
            .get(url)
            .query(&(("max", max), ("before", before)));

        self.into_stream::<PuzzleActivity>(builder).await
    }

    /// Get the puzzle dashboard of the logged in user.
    /// Includes all puzzle themes played, with aggregated results.
    pub async fn puzzle_dashboard(&self, days: u8) -> Result<PuzzleDashboard> {
        let mut url = self.base_url.clone();
        let path = format!("api/puzzle/dashboard/{days}");
        url.set_path(&path);
        let builder = self.client.get(url);

        self.into::<PuzzleDashboard>(builder).await
    }

    /// Get the puzzle storm dashboard of any player.
    /// Contains the aggregated highscores and the history of storm runs aggregated by days.
    /// Use `days = 0` if you only care about the highscores.
    pub async fn puzzle_dashboard_storm(
        &self,
        username: &str,
        days: Option<u16>,
    ) -> Result<PuzzleStormDashboard> {
        let mut url = self.base_url.clone();
        let path = format!("api/storm/dashboard/{username}");
        url.set_path(&path);
        let builder = self.client.get(url).query(&[("days", days)]);

        self.into::<PuzzleStormDashboard>(builder).await
    }

    /// Create a new private puzzle race.
    /// Once the puzzle race has been created, the creator must join the page and manually start the race when enough players have joined.
    pub async fn puzzle_race_create(&self) -> Result<PuzzleRace> {
        let mut url = self.base_url.clone();
        url.set_path("api/racer");
        let builder = self.client.post(url);

        self.into::<PuzzleRace>(builder).await
    }
}
