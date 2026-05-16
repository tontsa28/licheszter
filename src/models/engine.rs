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
