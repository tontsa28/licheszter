use crate::models::{
    common::date_dot,
    tv::FenEvent,
    user::{LightUser, MinimalUser, PerfType},
};
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as, skip_serializing_none};
use time::{Date, PrimitiveDateTime};

use super::{challenge::ChallengeSource, user::Title};

#[skip_serializing_none]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PlayerAnalysis {
    pub inaccuracy: u16,
    pub mistake: u16,
    pub blunder: u16,
    pub acpl: u16,
    pub accuracy: Option<u16>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Human {
    pub user: MinimalUser,
    pub rating: u16,
    pub rating_diff: Option<i16>,
    pub name: Option<String>,
    #[serde(default)]
    pub provisional: bool,
    pub analysis: Option<PlayerAnalysis>,
    pub team: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct StreamPlayer {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub rating: u16,
    #[serde(default)]
    pub provisional: bool,
}

#[skip_serializing_none]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Computer {
    #[serde(rename = "aiLevel")]
    pub ai_level: u8,
    pub analysis: Option<PlayerAnalysis>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(untagged)]
pub enum Player {
    Human(Box<Human>),
    Computer(Computer),
    Simple { name: String, rating: u16 },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Players {
    pub white: Player,
    pub black: Player,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct StreamPlayers {
    pub white: StreamPlayer,
    pub black: StreamPlayer,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Opening {
    pub eco: String,
    pub name: String,
    pub ply: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum TimeControl {
    Clock {
        limit: u16,
        increment: u16,
        show: String,
    },
    Correspondence {
        #[serde(rename = "daysPerTurn")]
        days_per_turn: u8,
    },
    Unlimited,
}

#[skip_serializing_none]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Clock {
    pub initial: u32,
    pub increment: u32,
    #[serde(rename = "totalTime")]
    pub total_time: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Speed {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Correspondence,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Judgement {
    pub name: JudgementTier,
    pub comment: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub enum JudgementTier {
    Inaccuracy,
    Mistake,
    Blunder,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct MoveAnalysis {
    pub mate: Option<u8>,
    pub eval: Option<i16>,
    pub best: Option<String>,
    pub variation: Option<String>,
    pub judgment: Option<Judgement>,
}

#[skip_serializing_none]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Division {
    pub middle: Option<u16>,
    pub end: Option<u16>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub full_id: Option<String>,
    pub id: String,
    pub rated: bool,
    pub variant: VariantMode,
    pub speed: Speed,
    pub perf: PerfType,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub created_at: PrimitiveDateTime,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub last_move_at: PrimitiveDateTime,
    pub status: GameStatus,
    pub source: Option<ChallengeSource>,
    pub players: Players,
    pub initial_fen: Option<String>,
    pub winner: Option<FinalColor>,
    pub opening: Option<Opening>,
    pub moves: Option<String>,
    pub days_per_turn: Option<u8>,
    #[serde(default)]
    pub analysis: Vec<MoveAnalysis>,
    pub tournament: Option<String>,
    pub swiss: Option<String>,
    pub clock: Option<Clock>,
    #[serde(default)]
    pub clocks: Vec<u32>,
    pub division: Option<Division>,
    pub import: Option<ImportDetails>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct StreamGame {
    pub id: String,
    pub rated: bool,
    pub variant: VariantMode,
    pub speed: Speed,
    pub perf: PerfType,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub created_at: PrimitiveDateTime,
    pub status: u8,
    pub status_name: GameStatus,
    pub players: StreamPlayers,
    pub clock: Option<Clock>,
    pub initial_fen: Option<String>,
    pub winner: Option<FinalColor>,
    pub days_per_turn: Option<u8>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct StreamMovesGame {
    pub id: String,
    pub variant: Variant,
    pub speed: Speed,
    pub perf: PerfType,
    pub rated: bool,
    pub initial_fen: Option<String>,
    pub fen: String,
    pub player: FinalColor,
    pub turns: u16,
    pub started_at_turn: Option<u16>,
    pub source: ChallengeSource,
    pub status: FullGameStatus,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub created_at: PrimitiveDateTime,
    pub last_move: Option<String>,
    pub players: Players,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum StreamMoves {
    Game(StreamMovesGame),
    Move(FenEvent),
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct GameState {
    // This field is useless and only present to prevent errors
    #[serde(skip_serializing)]
    r#type: Option<String>,
    pub moves: String,
    pub wtime: u64,
    pub btime: u64,
    pub winc: u32,
    pub binc: u32,
    #[serde(default)]
    pub wdraw: bool,
    #[serde(default)]
    pub bdraw: bool,
    #[serde(default)]
    pub wtakeback: bool,
    #[serde(default)]
    pub btakeback: bool,
    pub status: GameStatus,
    pub winner: Option<Color>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct GameFull {
    pub id: String,
    pub rated: bool,
    pub variant: Variant,
    pub clock: Option<Clock>,
    pub days_per_turn: Option<u8>,
    pub speed: Speed,
    pub perf: Perf,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub created_at: PrimitiveDateTime,
    pub white: GameEventPlayer,
    pub black: GameEventPlayer,
    pub initial_fen: String,
    pub state: GameState,
    pub tournament_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct OpponentGone {
    pub gone: bool,
    #[serde(rename = "claimWinInSeconds")]
    pub claim_win_in_seconds: Option<u8>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Variant {
    pub key: VariantMode,
    pub short: Option<String>,
    pub name: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Perf {
    pub icon: Option<String>,
    pub key: Option<String>,
    pub name: String,
    pub position: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct UserGames {
    #[serde(rename = "nowPlaying")]
    pub now_playing: Vec<UserGame>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct UserGame {
    pub full_id: String,
    pub game_id: String,
    pub fen: String,
    pub color: Color,
    pub last_move: String,
    pub source: ChallengeSource,
    pub variant: Variant,
    pub speed: String,
    pub perf: PerfType,
    pub rated: bool,
    pub has_moved: bool,
    pub opponent: LightUser,
    pub is_my_turn: bool,
    pub seconds_left: Option<u32>,
    pub status: FullGameStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VariantMode {
    Standard,
    Chess960,
    Crazyhouse,
    Antichess,
    Atomic,
    Horde,
    KingOfTheHill,
    RacingKings,
    ThreeCheck,
    FromPosition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Color {
    Black,
    Random,
    White,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FinalColor {
    Black,
    White,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct GameEventPlayer {
    #[serde(rename = "aiLevel")]
    pub ai_level: Option<u8>,
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
    pub rating: u16,
    #[serde(default)]
    pub provisional: bool,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct GameEventInfo {
    pub id: String,
    pub full_id: String,
    pub game_id: String,
    pub fen: String,
    pub color: Color,
    pub last_move: String,
    pub source: ChallengeSource,
    pub variant: Variant,
    pub speed: Speed,
    pub perf: String,
    pub rated: bool,
    pub has_moved: bool,
    pub opponent: LightUser,
    pub is_my_turn: bool,
    pub seconds_left: Option<u32>,
    pub status: FullGameStatus,
    pub compat: Option<GameCompatibility>,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CorrespondenceDays {
    One = 1,
    Two = 2,
    Three = 3,
    Five = 5,
    Seven = 7,
    Ten = 10,
    Fourteen = 14,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Rules {
    NoAbort,
    NoRematch,
    NoGiveTime,
    NoClaimWin,
    NoEarlyDraw,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AILevel {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GameType {
    Casual,
    Rated,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GameStatus {
    Created = 10,
    Started = 20,
    Aborted = 25,
    Mate = 30,
    Resign = 31,
    Stalemate = 32,
    Timeout = 33,
    Draw = 34,
    #[serde(rename = "outoftime")]
    OutOfTime = 35,
    Cheat = 36,
    NoStart = 37,
    UnknownFinish = 38,
    InsufficientMaterialClaim = 39,
    VariantEnd = 60,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct FullGameStatus {
    pub id: u8,
    pub name: GameStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct GameCompatibility {
    pub bot: bool,
    pub board: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct GameCount {
    pub all: u32,
    pub rated: u32,
    #[serde(default)]
    pub ai: u32,
    pub draw: u32,
    #[serde(default)]
    pub draw_h: u32,
    pub loss: u32,
    #[serde(default)]
    pub loss_h: u32,
    pub win: u32,
    #[serde(default)]
    pub win_h: u32,
    pub bookmark: u32,
    pub playing: u32,
    pub import: u32,
    pub me: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ImportGame {
    pub id: String,
    pub url: String,
}

#[serde_as]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ImportDetails {
    #[serde(with = "date_dot")]
    pub date: Date,
}
