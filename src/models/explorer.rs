use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::game::Speed;

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Opening {
    pub white: u32,
    pub black: u32,
    pub draws: u32,
    pub moves: Vec<OpeningMove>,
    pub top_games: Vec<HistoricOpening>,
    #[serde(default)]
    pub recent_games: Vec<HistoricOpening>,
    pub opening: Option<OpeningDetails>,
    #[serde(default)]
    pub history: Vec<HistoricMonth>
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct PlayerOpening {
    pub white: u32,
    pub black: u32,
    pub draws: u32,
    pub moves: Vec<PlayerOpeningMove>,
    pub recent_games: Vec<HistoricOpening>,
    pub opening: Option<OpeningDetails>,
    pub queue_position: u8,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct OpeningMove {
    pub uci: String,
    pub san: String,
    pub average_rating: u16,
    pub white: u32,
    pub black: u32,
    pub draws: u32,
    pub game: Option<HistoricOpening>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct PlayerOpeningMove {
    pub uci: String,
    pub san: String,
    pub average_opponent_rating: u16,
    pub performance: u16,
    pub white: u32,
    pub black: u32,
    pub draws: u32,
    pub game: Option<HistoricOpening>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct HistoricOpening {
    pub uci: Option<String>,
    pub id: String,
    pub winner: Option<String>,
    pub speed: Option<Speed>,
    pub mode: Option<String>,
    pub white: PlayerDetails,
    pub black: PlayerDetails,
    pub year: u16,
    pub month: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PlayerDetails {
    pub name: String,
    pub rating: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct OpeningDetails {
    pub eco: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct HistoricMonth {
    pub month: String,
    pub white: u32,
    pub black: u32,
    pub draws: u32,
}