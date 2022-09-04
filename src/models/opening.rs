use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Opening {
    pub white: u32,
    pub black: u32,
    pub draws: u32,
    pub moves: Option<Vec<OpeningMove>>,
    pub top_games: Option<Vec<HistoricOpening>>,
    pub recent_games: Option<Vec<HistoricOpening>>,
    pub opening: Option<OpeningDetails>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOpening {
    pub white: u32,
    pub black: u32,
    pub draws: u32,
    pub moves: Option<Vec<PlayerOpeningMove>>,
    pub recent_games: Option<Vec<HistoricOpening>>,
    pub opening: Option<OpeningDetails>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct HistoricOpening {
    pub uci: Option<String>,
    pub id: String,
    pub winner: Option<String>,
    pub speed: Option<String>,
    pub mode: Option<String>,
    pub white: PlayerDetails,
    pub black: PlayerDetails,
    pub year: u16,
    pub month: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerDetails {
    pub name: String,
    pub rating: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpeningDetails {
    pub eco: String,
    pub name: String,
}