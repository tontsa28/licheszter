use std::fmt::Display;

use super::{
    game::{
        Clock, Color, Computer, FinalColor, GameEventPlayer, Perf, Rules, Speed, TimeControl,
        Variant,
    },
    user::{ChallengeUser, LightUser, PerfType},
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::PrimitiveDateTime;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameID {
    pub id: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub id: String,
    pub url: String,
    pub final_color: FinalColor,
    pub color: Color,
    pub direction: Option<ChallengeDirection>,
    pub time_control: TimeControl,
    pub variant: Variant,
    pub challenger: ChallengeUser,
    pub dest_user: ChallengeUser,
    pub initial_fen: Option<String>,
    pub decline_reason: Option<String>,
    pub decline_reason_key: Option<ChallengeDeclineReason>,
    pub perf: Perf,
    pub rated: bool,
    pub speed: Speed,
    pub status: ChallengeStatus,
    #[serde(default)]
    pub rules: Vec<Rules>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Challenges {
    pub r#in: Vec<Challenge>,
    pub out: Vec<Challenge>,
}

// TODO: Will potentially be removed in the future
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct EntityChallenge {
    pub challenge: Challenge,
    pub socket_version: Option<u8>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct AIChallenge {
    pub id: String,
    pub variant: Variant,
    pub speed: Speed,
    pub perf: PerfType,
    pub rated: bool,
    pub initial_fen: Option<String>,
    pub fen: String,
    pub player: FinalColor,
    pub turns: u8,
    pub source: String,
    pub status: Status,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub created_at: PrimitiveDateTime,
    pub started_at_turn: Option<u8>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct OpenChallenge {
    pub id: String,
    pub url: String,
    pub status: ChallengeStatus,
    pub challenger: Option<Challenger>,
    pub dest_user: Option<Challenger>,
    pub variant: Variant,
    pub rated: bool,
    pub speed: Speed,
    pub time_control: TimeControl,
    pub color: Color,
    pub final_color: FinalColor,
    pub perf: Perf,
    pub open: OpenChallengePlayers,
    pub url_white: String,
    pub url_black: String,
    #[serde(default)]
    pub rules: Vec<Rules>,
    pub initial_fen: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct OpenChallengePlayers {
    #[serde(default)]
    #[serde(rename = "userIds")]
    pub user_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChallengeStatus {
    Created,
    Offline,
    Canceled,
    Declined,
    Accepted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChallengeDeclineReason {
    Generic,
    Later,
    TooFast,
    TooSlow,
    TimeControl,
    Rated,
    Casual,
    Standard,
    Variant,
    NoBot,
    OnlyBot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub enum ChallengeDirection {
    In,
    Out,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Status {
    pub id: u8,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Event {
    GameStart {
        game: GameID,
    },
    GameFinish {
        game: GameID,
    },
    Challenge {
        challenge: Challenge,
        compat: Option<Compat>,
    },
    ChallengeCanceled {
        challenge: Challenge,
    },
    ChallengeDeclined {
        challenge: Challenge,
    },
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GameStatus {
    Created,
    Started,
    Aborted,
    Mate,
    Resign,
    Stalemate,
    Timeout,
    Draw,
    #[serde(rename = "outoftime")]
    OutOfTime,
    Cheat,
    NoStart,
    UnknownFinish,
    VariantEnd,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Challenger {
    LightUser(Box<LightUser>),
    Computer(Computer),
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ChatLine {
    pub username: String,
    pub text: String,
    pub room: ChatRoom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChatRoom {
    Player,
    Spectator,
}

// TODO: Possibly temporary solution
impl Display for ChatRoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player => write!(f, "player"),
            Self::Spectator => write!(f, "spectator"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ChatMessage {
    pub text: String,
    pub user: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct OpponentGone {
    pub gone: bool,
    #[serde(rename = "claimWinInSeconds")]
    pub claim_win_in_seconds: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum BoardState {
    GameFull(Box<GameFull>),
    GameState(GameState),
    ChatLine(ChatLine),
    OpponentGone(OpponentGone),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Compat {
    pub bot: bool,
    pub board: bool,
}
