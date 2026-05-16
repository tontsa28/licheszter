use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::engine::UciVariant;

/// Partially optional configuration for creating external engines using [`external_engine().create()`](fn@crate::api::engine::ExternalEngineApi::create).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalEngineOptions {
    max_hash: u32,
    max_threads: u16,
    name: String,
    provider_secret: String,
    provider_data: Option<String>,
    variants: Option<Vec<UciVariant>>,
}

impl ExternalEngineOptions {
    /// Create a new instance of [`ExternalEngineOptions`] with provided configuration.
    /// Valid range for `max_hash` is from 1 to 1048576.
    /// `max_threads` has to be at least 1.
    /// `name` must be between 3 and 200 characters long.
    /// `secret` must be between 16 and 1024 characters long.
    #[must_use]
    pub fn new(max_hash: u32, max_threads: u16, name: &str, secret: &str) -> Self {
        ExternalEngineOptions {
            max_hash,
            max_threads,
            name: name.to_string(),
            provider_secret: secret.to_string(),
            provider_data: None,
            variants: None,
        }
    }

    /// Set optional, arbitrary data that the engine provider can use for identification or bookkeeping.
    #[must_use]
    pub fn provider_data(mut self, data: &str) -> Self {
        self.provider_data = Some(data.to_string());
        self
    }

    /// Set optional list of supported chess variants.
    #[must_use]
    pub fn variants(mut self, variants: &[UciVariant]) -> Self {
        self.variants = Some(variants.into());
        self
    }
}
