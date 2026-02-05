use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::simul::Simuls,
};

impl Licheszter {
    /// Get recently created, started and finished simuls.
    /// Created and finished simuls are only visible if the host is strong enough.
    /// When authenticated, the pending simuls will contain your created, but unstarted simuls.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn simuls_current(&self) -> Result<Simuls> {
        let url = self.req_url(UrlBase::Lichess, "api/simul");
        let builder = self.client.get(url);

        self.to_model::<Simuls>(builder).await
    }
}
