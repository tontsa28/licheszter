use super::{
    game::FullGameStatus,
    game::{Color, Computer, FinalColor, Perf, Rules, Speed, TimeControl, Variant},
    user::{ChallengeUser, LightUser, PerfType},
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::PrimitiveDateTime;

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
    pub status: FullGameStatus,
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
#[serde(rename_all = "camelCase")]
pub enum ChallengeDirection {
    In,
    Out,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChallengeSource {
    Lobby,
    Friend,
    #[serde(rename = "ai")]
    AI,
    #[serde(rename = "api")]
    API,
    Tournament,
    Position,
    Import,
    Importlive,
    Simul,
    Relay,
    Pool,
    Swiss,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(untagged)]
pub enum ChallengeComplete {
    Challenge(Box<Challenge>),
    Done { done: ChallengeStatus },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(untagged)]
pub enum Challenger {
    LightUser(Box<LightUser>),
    Computer(Computer),
}
