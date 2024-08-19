use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{game::Color, user::{PerfType, Title}};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Puzzle {
    pub game: PuzzleGame,
    pub puzzle: PuzzleDetails,
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
    pub rating: u16,
    pub title: Option<Title>,
}