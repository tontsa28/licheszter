use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::PrimitiveDateTime;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PerfType {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Chess960,
    Crazyhouse,
    Antichess,
    Atomic,
    Horde,
    KingOfTheHill,
    RacingKings,
    ThreeCheck,
    Puzzle,
    Correspondence,
}

impl Display for PerfType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UltraBullet => write!(f, "ultraBullet"),
            Self::Bullet => write!(f, "bullet"),
            Self::Blitz => write!(f, "blitz"),
            Self::Rapid => write!(f, "rapid"),
            Self::Classical => write!(f, "classical"),
            Self::Chess960 => write!(f, "chess960"),
            Self::Crazyhouse => write!(f, "crazyhouse"),
            Self::Antichess => write!(f, "antichess"),
            Self::Atomic => write!(f, "atomic"),
            Self::Horde => write!(f, "horde"),
            Self::KingOfTheHill => write!(f, "kingOfTheHill"),
            Self::RacingKings => write!(f, "racingKings"),
            Self::ThreeCheck => write!(f, "threeCheck"),
            Self::Puzzle => write!(f, "puzzle"),
            Self::Correspondence => write!(f, "correspondence"),
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPerfs {
    pub bullet: Option<UserPerf>,
    pub blitz: Option<UserPerf>,
    pub rapid: Option<UserPerf>,
    pub classical: Option<UserPerf>,
    pub correspondence: Option<UserPerf>,
    pub chess960: Option<UserPerf>,
    pub antichess: Option<UserPerf>,
    pub atomic: Option<UserPerf>,
    pub king_of_the_hill: Option<UserPerf>,
    pub crazyhouse: Option<UserPerf>,
    pub three_check: Option<UserPerf>,
    pub racing_kings: Option<UserPerf>,
    pub horde: Option<UserPerf>,
    pub puzzle: Option<UserPerf>,
    pub storm: Option<UserPuzzleModePerf>,
    pub racer: Option<UserPuzzleModePerf>,
    pub streak: Option<UserPuzzleModePerf>,
}

#[skip_serializing_none]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct UserPerf {
    pub games: Option<u32>,
    pub rating: u16,
    pub rd: Option<u16>,
    pub prog: i32,
    #[serde(default)]
    pub prov: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct UserPuzzleModePerf {
    #[serde(default)]
    pub runs: u32,
    #[serde(default)]
    pub score: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct RatingHistory {
    pub name: String,
    pub points: Vec<(u16, u8, u8, u16)>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct UserName {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct UserPerformance {
    pub user: UserName,
    pub perf: PerfDetails,
    pub rank: Option<u32>,
    pub percentile: Option<f32>,
    pub stat: PerfStats,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PerfDetails {
    pub glicko: PerfGlicko,
    pub nb: u32,
    pub progress: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PerfGlicko {
    pub rating: f32,
    pub deviation: f32,
    #[serde(default)]
    pub provisional: bool,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct PerfStats {
    pub perf_type: PerfType,
    pub nb_games: u32,
    pub rating: u16,
    pub progress: i32,
    pub rank: Option<u32>,
    pub percentile: Option<f32>,
    pub lowest_rating: Option<PerfPoint>,
    pub highest_rating: Option<PerfPoint>,
    pub count: PerfCount,
    pub result_streak: Option<PerfResultStreak>,
    pub play_streak: Option<PerfPlayStreak>,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PerfPoint {
    pub int: u16,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub at: PrimitiveDateTime,
    pub game_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct PerfCount {
    pub all: u32,
    pub rated: u32,
    pub win: u32,
    pub loss: u32,
    pub draw: u32,
    pub tour: u32,
    pub berserk: u32,
    #[serde(default)]
    pub disconnects: u32,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PerfResultStreak {
    pub win: PerfRecords,
    pub loss: PerfRecords,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PerfRecords {
    pub cur: PerfRecord,
    pub max: PerfRecord,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct PerfRecord {
    pub v: u32,
    pub from: Option<PerfStreakDate>,
    pub to: Option<PerfStreakDate>,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct PerfPlayStreak {
    pub nb: PerfStreak,
    pub time: PerfStreak,
    pub last_date: Option<PerfStreakDate>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PerfStreak {
    pub cur: PerfStreakRecord,
    pub max: PerfStreakRecord,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PerfStreakRecord {
    pub v: Option<u32>,
    pub from: Option<PerfStreakDate>,
    pub to: Option<PerfStreakDate>,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct PerfStreakDate {
    #[serde_as(as = "TimestampMilliSeconds")]
    pub at: PrimitiveDateTime,
    pub game_id: String,
}
