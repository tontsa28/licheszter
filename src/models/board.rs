use crate::models::game::{Clock, Perf, StockFish, Variant};
use crate::models::user::LightUser;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameID {
    pub id: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub id: String,
    pub url: String,
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
    pub status: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityChallenge {
    pub challenge: Option<Challenge>,
    pub game: Option<ChallengeGame>,
    pub socket_version: Option<u8>,
    pub url_white: Option<String>,
    pub url_black: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub url: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub id: u8,
    pub name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Event {
    /// A new game is starting
    GameStart { game: GameID },
    /// An ongoing game is finished
    GameFinish { game: GameID },
    /// Seek a new enemy
    Challenge { challenge: Challenge },
    /// Challenger withdrew
    ChallengeCanceled { challenge: Challenge },
    /// Challenge withdrew
    ChallengeDeclined { challenge: Challenge }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub r#type: Option<String>,
    pub moves: String,
    pub wtime: u32,
    pub btime: u32,
    pub winc: u16,
    pub binc: u16,
    pub wdraw: bool,
    pub bdraw: bool,
    pub status: String,
    pub winner: Option<String>,
    pub rematch: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Challengee {
    LightUser(LightUser),
    StockFish(StockFish)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub white: Challengee,
    pub black: Challengee,
    pub initial_fen: String,
    pub state: GameState,
    pub tournament_id: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatLine {
    pub username: String,
    pub text: String,
    pub room: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum BoardState {
    GameFull(GameFull),
    GameState(GameState),
    ChatLine(ChatLine)
}