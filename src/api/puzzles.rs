use crate::{client::Licheszter, error::Result, models::puzzle::Puzzle};

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
}