use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::PrimitiveDateTime;

use super::{game::Perf, user::Title};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Simuls {
    pub pending: Vec<Simul>,
    pub created: Vec<Simul>,
    pub started: Vec<Simul>,
    pub finished: Vec<Simul>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Simul {
    pub id: String,
    pub name: String,
    pub full_name: String,
    pub host: SimulHost,
    #[serde(default)]
    pub is_created: bool,
    #[serde(default)]
    pub is_finished: bool,
    #[serde(default)]
    pub is_running: bool,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub estimated_start_at: Option<PrimitiveDateTime>,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub started_at: Option<PrimitiveDateTime>,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub finished_at: Option<PrimitiveDateTime>,
    pub nb_applicants: u16,
    pub nb_pairings: u16,
    pub text: String,
    pub variants: Vec<Perf>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct SimulHost {
    pub id: String,
    pub name: String,
    pub rating: Option<u16>,
    pub title: Option<Title>,
    #[serde(default)]
    pub online: bool,
    #[serde(default)]
    pub provisional: bool,
    pub flair: Option<String>,
}
