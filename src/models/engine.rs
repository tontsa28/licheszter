use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ExternalEngine {
    pub client_secret: String,
    pub id: String,
    pub max_hash: u32,
    pub max_threads: u16,
    pub name: String,
    pub user_id: String,
    pub variants: Vec<UciVariant>,
    pub provider_data: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UciVariant {
    Chess,
    Crazyhouse,
    Antichess,
    Atomic,
    Horde,
    KingOfTheHill,
    RacingKings,
    #[serde(rename = "3check")]
    ThreeCheck,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ExternalEngineAnalysis {
    pub depth: u8,
    pub nodes: u64,
    pub pvs: Vec<ExternalEnginePv>,
    pub time: u32,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ExternalEnginePv {
    pub depth: u8,
    pub moves: Vec<String>,
    pub cp: Option<u32>,
    pub mate: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ExternalEngineAnalysisRequest {
    pub id: String,
    pub engine: ExternalEngine,
    pub work: ExternalEngineAnalysisWork,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ExternalEngineAnalysisWork {
    #[serde(flatten)]
    pub search: SearchMethod,
    pub hash: u32,
    pub initial_fen: String,
    pub moves: Vec<String>,
    pub multi_pv: u8,
    pub session_id: String,
    pub threads: u16,
    pub variant: UciVariant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchMethod {
    Movetime(u32),
    Depth(u8),
    Nodes(u64),
}
