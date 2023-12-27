use super::{
    game::{Color, Computer, FinalColor, Perf, Speed, TimeControl, Variant},
    user::{ChallengeUser, LightUser},
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
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
    pub dest_user: Option<ChallengeUser>,
    pub initial_fen: Option<String>,
    pub decline_reason: Option<String>,
    pub decline_reason_key: Option<String>,
    pub perf: Perf,
    pub rated: bool,
    pub speed: Speed,
    pub status: ChallengeStatus,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Challenges {
    pub r#in: Vec<Challenge>,
    pub out: Vec<Challenge>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct EntityChallenge {
    pub challenge: Option<Challenge>,
    pub game: Option<ChallengeGame>,
    pub socket_version: Option<u8>,
    pub url_white: Option<String>,
    pub url_black: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct ChallengeGame {
    pub id: String,
    pub variant: Variant,
    pub speed: Speed,
    pub perf: String,
    pub rated: bool,
    pub initial_fen: String,
    pub fen: String,
    pub player: String,
    pub turns: u8,
    pub started_at_turn: u8,
    pub source: String,
    pub status: Status,
    pub created_at: PrimitiveDateTime,
    pub url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub enum ChallengeStatus {
    Created,
    Offline,
    Canceled,
    Declined,
    Accepted,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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
    pub r#type: Option<String>,
    pub moves: String,
    pub wtime: u32,
    pub btime: u32,
    pub winc: u16,
    pub binc: u16,
    pub wdraw: Option<bool>,
    pub bdraw: Option<bool>,
    pub status: String,
    pub winner: Option<String>,
    pub rematch: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Challenger {
    LightUser(Box<LightUser>),
    Computer(Computer),
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct GameFull {
    pub id: String,
    pub rated: bool,
    pub variant: Variant,
    pub clock: Option<TimeControl>,
    pub speed: Speed,
    pub perf: Perf,
    pub created_at: PrimitiveDateTime,
    pub white: Challenger,
    pub black: Challenger,
    pub initial_fen: String,
    pub state: GameState,
    pub tournament_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ChatLine {
    pub username: String,
    pub text: String,
    pub room: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Compat {
    pub bot: bool,
    pub board: bool,
}
