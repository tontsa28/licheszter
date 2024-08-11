use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::PrimitiveDateTime;

use super::game::GameCount;

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub perfs: UserPerfs,
    pub flair: Option<String>,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub created_at: PrimitiveDateTime,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub tos_violation: bool,
    pub profile: Option<Profile>,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub seen_at: PrimitiveDateTime,
    #[serde(default)]
    pub patron: bool,
    #[serde(default)]
    pub verified: bool,
    pub play_time: BotPlayTime,
    pub title: Option<Title>,
    pub url: String,
    pub playing: Option<String>,
    pub count: Option<GameCount>,
    #[serde(default)]
    pub streaming: bool,
    pub streamer: Option<Streamer>,
    #[serde(default)]
    pub followable: bool,
    #[serde(default)]
    pub following: bool,
    #[serde(default)]
    pub blocking: bool,
    #[serde(default)]
    pub follows_you: bool,
}

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

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct LightUser {
    pub id: Option<String>,
    #[serde(alias = "name")]
    pub username: String,
    pub ai: Option<u8>,
    pub perfs: Option<UserPerfs>,
    pub title: Option<Title>,
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ChallengeUser {
    pub rating: Option<u16>,
    #[serde(default)]
    pub provisional: bool,
    #[serde(default)]
    pub online: bool,
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
    #[serde(default)]
    pub patron: bool,
    pub flair: Option<String>,
    pub lag: Option<u16>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct BotUser {
    pub id: String,
    pub username: String,
    pub perfs: BotPerfs,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub created_at: PrimitiveDateTime,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub tos_violation: bool,
    pub profile: Profile,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub seen_at: PrimitiveDateTime,
    #[serde(default)]
    pub patron: bool,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub play_time: BotPlayTime,
    pub title: Title,
    pub flair: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub location: Option<String>,
    pub bio: Option<String>,
    pub flag: Option<String>,
    pub real_name: Option<String>,
    pub links: Option<String>,
    pub fide_rating: Option<u16>,
    pub uscf_rating: Option<u16>,
    pub ecf_rating: Option<u16>,
    pub rcf_rating: Option<u16>,
    pub cfc_rating: Option<u16>,
    pub dsb_rating: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(default)]
pub struct BotPlayTime {
    pub total: u32,
    pub tv: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Title {
    GM,
    WGM,
    IM,
    WIM,
    FM,
    WFM,
    NM,
    CM,
    WCM,
    WNM,
    LM,
    BOT,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Streamer {
    pub twitch: Option<StreamerChannel>,
    #[serde(rename = "youTube")]
    pub youtube: Option<StreamerChannel>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct StreamerChannel {
    pub channel: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Email {
    pub email: String,
}