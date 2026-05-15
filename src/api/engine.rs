use std::sync::Arc;

use crate::{
    client::{LicheszterInner, UrlBase},
    error::Result,
    models::engine::ExternalEngine,
};

/// A struct for accessing the External engine API endpoints.
#[derive(Debug)]
pub struct ExternalEngineApi {
    pub(crate) inner: Arc<LicheszterInner>,
}

impl ExternalEngineApi {
    /// List all external engines that have been registered for the user,
    /// and the credentials required to use them.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn list(&self) -> Result<Vec<ExternalEngine>> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/external-engine");
        let builder = self.inner.client.get(url);

        self.inner.to_model::<Vec<ExternalEngine>>(builder).await
    }
}
