use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::PrimitiveDateTime;

use crate::models::common::{PatronTier, Title};

use super::perf::UserPerfs;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct LightUser {
    pub id: Option<String>,
    #[serde(alias = "name")]
    pub username: String,
    pub ai: Option<u8>,
    pub perfs: Option<UserPerfs>,
    pub title: Option<Title>,
    #[serde(default)]
    pub online: bool,
    #[serde(default)]
    pub playing: bool,
    #[serde(default)]
    pub streaming: bool,
    #[serde(default)]
    pub patron: bool,
    pub patron_tier: Option<PatronTier>,
    pub patron_color: Option<u8>,
    pub rating: Option<u16>,
    #[serde(default)]
    pub provisional: bool,
    pub lag: Option<u16>,
    pub game_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MinimalUser {
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
    #[serde(default)]
    pub patron: bool,
    pub patron_tier: Option<PatronTier>,
    pub patron_color: Option<u8>,
    #[serde(default)]
    pub online: bool,
    pub flair: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
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
    pub patron_tier: Option<PatronTier>,
    pub patron_color: Option<u8>,
    pub flair: Option<String>,
    pub lag: Option<u16>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct BasicUser {
    pub id: String,
    pub username: String,
    pub perfs: UserPerfs,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub created_at: PrimitiveDateTime,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub tos_violation: bool,
    pub profile: Option<Profile>,
    #[serde(default)]
    pub seen_at: Option<PrimitiveDateTime>,
    pub patron: Option<bool>,
    pub verified: Option<bool>,
    pub play_time: Option<PlayTime>,
    pub title: Option<Title>,
    pub url: String,
    pub playing: Option<String>,
    pub count: GameCount,
    #[serde(default)]
    pub streaming: bool,
    #[serde(default)]
    pub followable: bool,
    #[serde(default)]
    pub following: bool,
    #[serde(default)]
    pub blocking: bool,
    #[serde(default)]
    pub follows_you: bool,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub country: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub fide_rating: Option<u16>,
    pub uscf_rating: Option<u16>,
    pub ecf_rating: Option<u16>,
    pub rcf_rating: Option<u16>,
    pub cfc_rating: Option<u16>,
    pub dsb_rating: Option<u16>,
    pub links: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PlayTime {
    pub total: u32,
    pub tv: u32,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct RealtimeUser {
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
    pub online: Option<bool>,
    #[serde(default)]
    pub playing: bool,
    pub patron: Option<bool>,
    pub patron_tier: Option<PatronTier>,
    pub patron_color: Option<u8>,
    pub play_time: PlayTime,
    pub perfs: UserPerfs,
    pub language: Option<String>,
    pub profile: Option<Profile>,
    #[serde(default)]
    pub url: String,
    pub user_playing: Option<RealtimeUserPlaying>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub enum RealtimeUserPlaying {
    NowPlaying(Vec<NowPlaying>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct NowPlaying {
    pub full_id: String,
    pub game_id: String,
    pub fen: String,
    pub color: Color,
    pub last_move: String,
    pub variant: Variant,
    pub speed: Speed,
    pub perf: String,
    pub rated: bool,
    pub has_moved: bool,
    pub is_my_turn: bool,
    pub opponent: NowPlayingOpponent,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct NowPlayingOpponent {
    pub id: Option<String>,
    pub username: String,
    pub rating: Option<u16>,
    pub ai: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct UserAutocomplete {
    pub result: Vec<MinimalUser>,
}

// Re-export types needed for variants
use crate::models::{
    common::Color,
    game::{GameCount, Speed, Variant},
};

#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct StreamingUser {
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
    pub patron: Option<bool>,
    pub patron_tier: Option<PatronTier>,
    pub patron_color: Option<u8>,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub created_at: PrimitiveDateTime,
    #[serde_as(as = "Option<TimestampMilliSeconds>")]
    pub seen_at: Option<PrimitiveDateTime>,
    pub rating: u16,
    #[serde(default)]
    pub provisional: bool,
}
