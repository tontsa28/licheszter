use crate::{client::Licheszter, error::Result, models::simul::Simuls};

impl Licheszter {
    pub async fn simuls_current(&self) -> Result<Simuls> {
        let mut url = self.base_url();
        url.set_path("api/simul");
        let builder = self.client.get(url);

        self.into::<Simuls>(builder).await
    }
}
