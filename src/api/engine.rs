use std::{pin::Pin, sync::Arc};

use futures_util::Stream;

use crate::{
    client::{LicheszterInner, UrlBase},
    config::engine::{
        ExternalEngineAnalysisBody, ExternalEngineAnalysisOptions, ExternalEngineOptions,
    },
    error::Result,
    models::{
        common::OkResponse,
        engine::{ExternalEngine, ExternalEngineAnalysis},
    },
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

    /// Get properties and credentials of an external engine.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn show(&self, id: &str) -> Result<ExternalEngine> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/external-engine/{id}"));
        let builder = self.inner.client.get(url);

        self.inner.to_model::<ExternalEngine>(builder).await
    }

    /// Updates the properties of an external engine.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn update(
        &self,
        id: &str,
        options: &ExternalEngineOptions,
    ) -> Result<ExternalEngine> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/external-engine/{id}"));
        let builder = self.inner.client.put(url).json(options);

        self.inner.to_model::<ExternalEngine>(builder).await
    }

    /// Unregisters an external engine.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn delete(&self, id: &str) -> Result<()> {
        let url = self
            .inner
            .req_url(UrlBase::Lichess, &format!("api/external-engine/{id}"));
        let builder = self.inner.client.delete(url);

        self.inner.to_model::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Request analysis from an external engine.
    /// The properties are based on the UCI specification.
    /// Analysis stops when the client disconnects, the requested limit is reached, or the provider goes away.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn analysis_request(
        &self,
        id: &str,
        secret: &str,
        options: &ExternalEngineAnalysisOptions,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ExternalEngineAnalysis>> + Send>>> {
        let url = self.inner.req_url(
            UrlBase::Engine,
            &format!("api/external-engine/{id}/analyse"),
        );
        let builder = self
            .inner
            .client
            .post(url)
            .json(&ExternalEngineAnalysisBody {
                client_secret: secret.to_string(),
                work: options.clone(),
            });

        self.inner
            .to_stream::<ExternalEngineAnalysis>(builder)
            .await
    }
}
