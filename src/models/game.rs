use crate::models::user::{LightUser, PerfType};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use time::PrimitiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerAnalysis {
    pub inaccuracy: u16,
    pub mistake: u16,
    pub blunder: u16,
    pub acpl: u16,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub user: Option<LightUser>,
    pub user_id: Option<String>,
    pub rating: u16,
    pub rating_diff: Option<i16>,
    pub provisional: Option<bool>,
    pub analysis: Option<PlayerAnalysis>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Computer {
    #[serde(rename = "aiLevel")]
    pub ai_level: u8,
    pub analysis: Option<PlayerAnalysis>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Player {
    Entity(Entity),
    Computer(Computer),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Players {
    pub white: Player,
    pub black: Player,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Opening {
    pub eco: String,
    pub name: String,
    pub ply: u16,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Clock {
    pub initial: Option<u32>,
    pub increment: Option<u16>,
    pub total_time: Option<u16>,
    pub limit: Option<u16>,
    pub days_per_turn: Option<u8>,
    pub show: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Judgement {
    pub name: String,
    pub comment: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MoveAnalysis {
    pub mate: Option<u8>,
    pub eval: Option<i16>,
    pub best: Option<String>,
    pub variation: Option<String>,
    pub judgment: Option<Judgement>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: String,
    pub rated: bool,
    pub variant: String,
    pub speed: String,
    pub perf: PerfType,
    pub created_at: PrimitiveDateTime,
    pub last_move_at: Option<PrimitiveDateTime>,
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
    pub clock: Option<Clock>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub key: String,
    pub short: Option<String>,
    pub name: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Perf {
    pub icon: Option<String>,
    pub key: Option<String>,
    pub name: String,
    pub position: Option<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserGames {
    #[serde(rename = "nowPlaying")]
    pub now_playing: Vec<UserGame>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct UserGame {
    pub full_id: String,
    pub game_id: String,
    pub fen: String,
    pub color: String,
    pub last_move: String,
    pub source: String,
    pub variant: Variant,
    pub speed: String,
    pub perf: PerfType,
    pub rated: bool,
    pub has_moved: bool,
    pub opponent: LightUser,
    pub is_my_turn: bool,
    pub seconds_left: u32,
}
