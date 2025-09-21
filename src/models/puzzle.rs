use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as, skip_serializing_none};
use time::PrimitiveDateTime;

use super::{
    game::Color,
    user::{PerfType, Title},
};

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Puzzle {
    pub game: PuzzleGame,
    pub puzzle: PuzzleDetails,
    pub user: Option<PuzzleAuthUser>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleGame {
    pub id: String,
    pub clock: String,
    pub perf: PuzzlePerf,
    pub pgn: String,
    pub players: Vec<PuzzleUser>,
    pub rated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleDetails {
    pub id: String,
    #[serde(rename = "initialPly")]
    pub initial_ply: u16,
    pub plays: u32,
    pub rating: u16,
    pub solution: Vec<String>,
    pub themes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzlePerf {
    pub key: PerfType,
    pub name: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleUser {
    pub id: String,
    pub name: String,
    pub color: Color,
    pub flair: Option<String>,
    #[serde(default)]
    pub patron: bool,
    #[serde(rename = "patronTier")]
    pub patron_tier: Option<String>,
    pub rating: u16,
    pub title: Option<Title>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleAuthUser {
    pub id: String,
    pub rating: u16,
    #[serde(default)]
    pub provisional: bool,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleActivity {
    #[serde_as(as = "TimestampMilliSeconds")]
    pub date: PrimitiveDateTime,
    pub puzzle: PuzzleActivityDetails,
    pub win: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleActivityDetails {
    pub id: String,
    pub fen: String,
    #[serde(rename = "lastMove")]
    pub last_move: String,
    pub plays: u32,
    pub rating: u16,
    pub solution: Vec<String>,
    pub themes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleDashboard {
    pub days: u8,
    pub global: PuzzlePerformance,
    pub themes: BTreeMap<String, PuzzleTheme>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct PuzzlePerformance {
    pub first_wins: u32,
    pub nb: u32,
    pub performance: u16,
    pub puzzle_rating_avg: u16,
    pub replay_wins: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleTheme {
    pub results: PuzzlePerformance,
    pub theme: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleStormDashboard {
    pub days: Vec<PuzzleStormDay>,
    pub high: PuzzleStormRecord,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleStormDay {
    #[serde(rename = "_id")]
    pub id: String,
    pub combo: u16,
    pub errors: u16,
    pub highest: u16,
    pub moves: u32,
    pub runs: u16,
    pub score: u8,
    pub time: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleStormRecord {
    #[serde(rename = "allTime")]
    pub all_time: u8,
    pub day: u8,
    pub month: u8,
    pub week: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzleRace {
    pub id: String,
    pub url: String,
}
