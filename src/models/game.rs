use crate::models::user::{LightUser, PerfType};
use chrono::{serde::{ts_milliseconds, ts_milliseconds_option}, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerAnalysis {
    pub inaccuracy: u16,
    pub mistake: u16,
    pub blunder: u16,
    pub acpl: u16
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub user: Option<LightUser>,
    pub user_id: Option<String>,
    pub rating: u16,
    pub rating_diff: Option<i16>,
    pub provisional: Option<bool>,
    pub analysis: Option<PlayerAnalysis>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StockFish {
    #[serde(rename = "aiLevel")]
    pub ai_level: u8,
    pub analysis: Option<PlayerAnalysis>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Player {
    Entity(Entity),
    StockFish(StockFish)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Players {
    pub white: Player,
    pub black: Player
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Opening {
    pub eco: String,
    pub name: String,
    pub ply: u16
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clock {
    pub initial: Option<u32>,
    pub increment: Option<u16>,
    pub total_time: Option<u16>,
    pub limit: Option<u16>,
    pub days_per_turn: Option<u8>,
    pub show: Option<String>,
    pub r#type: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Judgement {
    pub name: String,
    pub comment: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoveAnalysis {
    pub mate: Option<u8>,
    pub eval: Option<i16>,
    pub best: Option<String>,
    pub variation: Option<String>,
    pub judgment: Option<Judgement>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: String,
    pub rated: bool,
    pub variant: String,
    pub speed: String,
    pub perf: PerfType,
    #[serde(deserialize_with = "ts_milliseconds::deserialize")]
    pub created_at: DateTime<Utc>,
    #[serde(default, deserialize_with = "ts_milliseconds_option::deserialize")]
    pub last_move_at: Option<DateTime<Utc>>,
    pub status: String,
    pub players: Players,
    pub initial_fen: Option<String>,
    pub winner: Option<String>,
    pub opening: Option<Opening>,
    pub moves: Option<String>,
    pub pgn: Option<String>,
    pub days_per_turn: Option<u8>,
    pub analysis: Option<Vec<MoveAnalysis>>,
    pub tournament: Option<String>,
    pub clock: Option<Clock>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Variant {
    pub key: String,
    pub short: Option<String>,
    pub name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Perf {
    pub icon: Option<String>,
    pub key: Option<String>,
    pub name: String,
    pub position: Option<u8>
}