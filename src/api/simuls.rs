use crate::{client::Licheszter, error::Result, models::simul::Simuls};

impl Licheszter {
    // Get recently created, started and finished simuls.
    // Created and finished simuls are only visible if the host is strong enough.
    // When authenticated, the pending simuls will contain your created, but unstarted simuls.
    pub async fn simuls_current(&self) -> Result<Simuls> {
        let mut url = self.base_url.clone();
        url.set_path("api/simul");
        let builder = self.client.get(url);

        self.into::<Simuls>(builder).await
    }
}
