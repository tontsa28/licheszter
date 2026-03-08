use crate::{
    client::{LicheszterInner, UrlBase},
    error::Result,
    models::simul::Simuls,
};

use std::sync::Arc;

/// A struct for accessing the Simuls API endpoints.
#[derive(Debug)]
pub struct SimulsApi {
    pub(crate) inner: Arc<LicheszterInner>,
}

impl SimulsApi {
    /// Get recently created, started and finished simuls.
    /// Created and finished simuls are only visible if the host is strong enough.
    /// When authenticated, the pending simuls will contain your created, but unstarted simuls.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn current(&self) -> Result<Simuls> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/simul");
        let builder = self.inner.client.get(url);

        self.inner.to_model::<Simuls>(builder).await
    }
}
