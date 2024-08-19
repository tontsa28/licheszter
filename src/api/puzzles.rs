use crate::{client::Licheszter, error::Result, models::puzzle::Puzzle};

impl Licheszter {
    pub async fn puzzles_daily(&self) -> Result<Puzzle> {
        let mut url = self.base_url();
        url.set_path("api/puzzle/daily");
        let builder = self.client.get(url);

        self.to_model::<Puzzle>(builder).await
    }
}