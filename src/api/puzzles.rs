use futures_util::Stream;

use crate::{
    client::Licheszter,
    error::Result,
    models::puzzle::{Puzzle, PuzzleActivity},
};

impl Licheszter {
    /// Get the daily puzzle.
    pub async fn puzzle_daily(&self) -> Result<Puzzle> {
        let mut url = self.base_url();
        url.set_path("api/puzzle/daily");
        let builder = self.client.get(url);

        self.to_model::<Puzzle>(builder).await
    }

    /// Get a single puzzle by ID.
    pub async fn puzzle_show(&self, id: &str) -> Result<Puzzle> {
        let mut url = self.base_url();
        let path = format!("api/puzzle/{id}");
        url.set_path(&path);
        let builder = self.client.get(url);

        self.to_model::<Puzzle>(builder).await
    }

    /// Get the puzzle activity of the logged in user.
    pub async fn puzzle_activity(
        &self,
        max: Option<u16>,
        before: Option<u64>,
    ) -> Result<impl Stream<Item = Result<PuzzleActivity>>> {
        let mut url = self.base_url();
        url.set_path("api/puzzle/activity");
        let builder = self
            .client
            .get(url)
            .query(&(("max", max), ("before", before)));

        self.to_model_stream::<PuzzleActivity>(builder).await
    }
}
