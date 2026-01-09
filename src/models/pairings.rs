use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::PrimitiveDateTime;

use crate::models::game::{MiniGame, TimeControl, VariantMode};

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct BulkPairing {
    pub clock: TimeControl,
    pub games: Vec<MiniGame>,
    pub id: String,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub pair_at: PrimitiveDateTime,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub paired_at: Option<PrimitiveDateTime>,
    pub rated: bool,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub scheduled_at: PrimitiveDateTime,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub start_clocks_at: PrimitiveDateTime,
    pub variant: VariantMode,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct BulkPairings {
    pub bulks: Vec<BulkPairing>,
}
