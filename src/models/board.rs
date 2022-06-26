use crate::models::game::{Clock, Perf, StockFish, Variant};
use crate::models::user::LightUser;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameID {
    pub id: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub id: String,
    pub url: String,
    pub final_color: String,
    pub color: String,
    pub direction: Option<String>,
    pub time_control: Clock,
    pub variant: Variant,
    pub challenger: Option<LightUser>,
    pub dest_user: Option<LightUser>,
    pub initial_fen: Option<String>,
    pub decline_reason: Option<String>,
    pub perf: Perf,
    pub rated: bool,
    pub speed: String,
    pub status: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct EntityChallenge {
    pub challenge: Option<Challenge>,
    pub game: Option<ChallengeGame>,
    pub socket_version: Option<u8>,
    pub url_white: Option<String>,
    pub url_black: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeGame {
    pub id: String,
    pub variant: Variant,
    pub speed: String,
    pub perf: String,
    pub rated: bool,
    pub initial_fen: String,
    pub fen: String,
    pub player: String,
    pub turns: u8,
    pub started_at_turn: u8,
    pub source: String,
    pub status: Status,
    #[serde(deserialize_with = "ts_milliseconds::deserialize")]
    pub created_at: DateTime<Utc>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub id: u8,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Event {
    GameStart { game: GameID },
    GameFinish { game: GameID },
    Challenge { challenge: Challenge },
    ChallengeCanceled { challenge: Challenge },
    ChallengeDeclined { challenge: Challenge },
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameState {
    pub r#type: Option<String>,
    pub moves: String,
    pub wtime: u32,
    pub btime: u32,
    pub winc: u16,
    pub binc: u16,
    pub status: String,
    pub winner: Option<String>,
    pub rematch: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Challenger {
    LightUser(LightUser),
    StockFish(StockFish),
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct GameFull {
    pub id: String,
    pub rated: bool,
    pub variant: Variant,
    pub clock: Option<Clock>,
    pub speed: String,
    pub perf: Perf,
    #[serde(deserialize_with = "ts_milliseconds::deserialize")]
    pub created_at: DateTime<Utc>,
    pub white: Challenger,
    pub black: Challenger,
    pub initial_fen: String,
    pub state: GameState,
    pub tournament_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatLine {
    pub username: String,
    pub text: String,
    pub room: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum BoardState {
    GameFull(GameFull),
    GameState(GameState),
    ChatLine(ChatLine),
}