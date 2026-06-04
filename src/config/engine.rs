use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::engine::{SearchMethod, UciVariant};

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
    ///
    /// API constraints:
    /// - Valid range for `max_hash` is from 1 to 1048576.
    /// - `max_threads` has to be at least 1.
    /// - `name` must be between 3 and 200 characters long.
    /// - `secret` must be between 16 and 1024 characters long.
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

/// Mandatory configuration for using external engines for analysis using [`external_engine().analyse()`](fn@crate::api::engine::ExternalEngineApi::analyse).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalEngineAnalysisOptions {
    #[serde(flatten)]
    search: SearchMethod,
    hash: u32,
    initial_fen: String,
    moves: Vec<String>,
    multi_pv: u8,
    session_id: String,
    threads: u16,
    variant: UciVariant,
}

impl ExternalEngineAnalysisOptions {
    /// Create a new instance of [`ExternalEngineAnalysisOptions`] with provided configuration.
    ///
    /// API constraints:
    /// - `search` must be at least 1 (milliseconds in movetime).
    /// - `hash` must be at least 1 (MiB).
    /// - `initial_fen`, `moves`, `session_id`, and `variant` are required.
    /// - `multi_pv` must be between 1 and 5.
    /// - `threads` must be at least 1.
    #[must_use]
    pub fn new(
        search: SearchMethod,
        hash: u32,
        initial_fen: &str,
        moves: &[&str],
        multi_pv: u8,
        session_id: &str,
        threads: u16,
        variant: UciVariant,
    ) -> Self {
        ExternalEngineAnalysisOptions {
            search,
            hash,
            initial_fen: initial_fen.to_string(),
            moves: moves.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            multi_pv,
            session_id: session_id.to_string(),
            threads,
            variant,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct ExternalEngineAnalysisBody {
    #[serde(rename = "clientSecret")]
    pub(crate) client_secret: String,
    pub(crate) work: ExternalEngineAnalysisOptions,
}
