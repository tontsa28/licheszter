use std::sync::Arc;

use crate::{
    client::{LicheszterInner, UrlBase},
    config::engine::ExternalEngineOptions,
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

    /// Registers a new external engine for the user.
    /// It can then be selected and used on the analysis board.
    /// After registering, the provider should start waiting for analysis requests.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn create(&self, options: &ExternalEngineOptions) -> Result<ExternalEngine> {
        let url = self.inner.req_url(UrlBase::Lichess, "api/external-engine");
        let builder = self.inner.client.post(url).json(options);

        self.inner.to_model::<ExternalEngine>(builder).await
    }
}
