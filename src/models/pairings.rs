use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::PrimitiveDateTime;

use crate::models::game::{MiniGame, Rules, VariantMode};

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct BulkPairing {
    #[serde(alias = "correspondence")]
    pub clock: BulkPairingTimeControl,
    pub games: Vec<MiniGame>,
    pub id: String,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub pair_at: PrimitiveDateTime,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub paired_at: Option<PrimitiveDateTime>,
    pub rated: bool,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub scheduled_at: PrimitiveDateTime,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub start_clocks_at: Option<PrimitiveDateTime>,
    pub variant: VariantMode,
    pub message: Option<String>,
    #[serde(default)]
    pub rules: Vec<Rules>,
    pub fen: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct BulkPairings {
    pub bulks: Vec<BulkPairing>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BulkPairingTimeControl {
    Clock {
        limit: u16,
        increment: u8,
    },
    Correspondence {
        #[serde(rename = "daysPerTurn")]
        days_per_turn: u8,
    },
}
