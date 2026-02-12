use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::PrimitiveDateTime;

use crate::models::common::{PatronTier, Title};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct TopUsers {
    pub bullet: Vec<TopUser>,
    pub blitz: Vec<TopUser>,
    pub rapid: Vec<TopUser>,
    pub classical: Vec<TopUser>,
    pub ultra_bullet: Vec<TopUser>,
    pub chess960: Vec<TopUser>,
    pub crazyhouse: Vec<TopUser>,
    pub antichess: Vec<TopUser>,
    pub atomic: Vec<TopUser>,
    pub horde: Vec<TopUser>,
    pub king_of_the_hill: Vec<TopUser>,
    pub racing_kings: Vec<TopUser>,
    pub three_check: Vec<TopUser>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct TopUser {
    pub id: String,
    pub username: String,
    pub perfs: BTreeMap<String, TopUserPerf>,
    pub title: Option<Title>,
    #[serde(default)]
    pub online: bool,
    #[serde(default)]
    pub patron: bool,
    pub patron_tier: Option<PatronTier>,
    pub patron_color: Option<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct TopUserPerf {
    pub rating: u16,
    pub progress: i16,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct TopUserLeaderboard {
    pub users: Vec<TopUser>,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Trophy {
    PerfTop {
        perf: String,
        top: u8,
        name: String,
    },
    Moderator {
        name: String,
        #[serde_as(as = "TimestampMilliSeconds")]
        date: PrimitiveDateTime,
        icon: String,
        url: String,
    },
    Developer {
        name: String,
        #[serde_as(as = "TimestampMilliSeconds")]
        date: PrimitiveDateTime,
        icon: String,
        url: String,
    },
    Verified {
        name: String,
        #[serde_as(as = "TimestampMilliSeconds")]
        date: PrimitiveDateTime,
        icon: String,
    },
    ContentTeam {
        name: String,
        #[serde_as(as = "TimestampMilliSeconds")]
        date: PrimitiveDateTime,
        icon: String,
    },
    #[serde(
        alias = "marathonWinner",
        alias = "marathonTopTen",
        alias = "marathonTopFifty",
        alias = "marathonTopHundred"
    )]
    MarathonTop {
        name: String,
        #[serde_as(as = "TimestampMilliSeconds")]
        date: PrimitiveDateTime,
        icon: String,
    },
    ZugMiracle {
        name: String,
        #[serde_as(as = "TimestampMilliSeconds")]
        date: PrimitiveDateTime,
        url: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct RatingHistory {
    pub name: String,
    pub points: Vec<(u16, u8, u8, u16)>,
}
