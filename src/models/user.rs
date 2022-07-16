use std::collections::HashMap;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
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

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserPerf {
    pub games: Option<u32>,
    pub rating: u16,
    pub rd: Option<u16>,
    #[serde(alias = "progress")]
    pub prog: i32,
    pub prov: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LightUser {
    pub id: Option<String>,
    #[serde(alias = "name")]
    pub username: String,
    pub ai: Option<u8>,
    pub perfs: Option<HashMap<PerfType, UserPerf>>,
    pub title: Option<String>,
    pub online: Option<bool>,
    pub playing: Option<bool>,
    pub streaming: Option<bool>,
    pub patron: Option<bool>,
    pub rating: Option<u16>,
    pub provisional: Option<bool>,
    pub lag: Option<u16>,
    #[serde(rename = "gameId")]
    pub game_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct BotUser {
    pub id: String,
    pub username: String,
    pub perfs: BotPerfs,
    #[serde(deserialize_with = "ts_milliseconds::deserialize")]
    pub created_at: DateTime<Utc>,
    pub disabled: Option<bool>,
    pub tos_violation: Option<bool>,
    pub profile: Option<BotProfile>,
    #[serde(deserialize_with = "ts_milliseconds::deserialize")]
    pub seen_at: DateTime<Utc>,
    pub patron: Option<bool>,
    pub verified: Option<bool>,
    pub play_time: BotPlayTime,
    pub title: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct BotPerfs {
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
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct BotProfile {
    pub country: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub links: Option<String>,
    pub fide_rating: Option<u16>,
    pub uscf_rating: Option<u16>,
    pub ecf_rating: Option<u16>,
    pub rcf_rating: Option<u16>,
    pub cfc_rating: Option<u16>,
    pub dsb_rating: Option<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BotPlayTime {
    pub total: u32,
    pub tv: u32,
}