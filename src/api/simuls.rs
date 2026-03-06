use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::simul::Simuls,
};

/// A struct for accessing the Simuls API endpoints.
pub struct SimulsApi<'a> {
    pub(crate) client: &'a Licheszter,
}

impl SimulsApi<'_> {
    /// Get recently created, started and finished simuls.
    /// Created and finished simuls are only visible if the host is strong enough.
    /// When authenticated, the pending simuls will contain your created, but unstarted simuls.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn current(&self) -> Result<Simuls> {
        let url = self.client.req_url(UrlBase::Lichess, "api/simul");
        let builder = self.client.client.get(url);

        self.client.to_model::<Simuls>(builder).await
    }
}
