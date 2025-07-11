use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::user::Title;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct FidePlayer {
    pub id: u32,
    pub name: String,
    pub title: Option<Title>,
    pub federation: String,
    pub year: Option<u16>,
    pub inactive: Option<u16>,
    pub standard: Option<u16>,
    pub rapid: Option<u16>,
    pub blitz: Option<u16>,
}
