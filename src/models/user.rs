use std::{collections::BTreeMap, fmt::Display};

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
    pub play_time: PlayTime,
    pub title: Option<Title>,
    pub url: String,
    pub playing: Option<String>,
    pub count: Option<GameCount>,
    #[serde(default)]
    pub trophies: Vec<Trophy>,
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

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    pub dark: bool,
    pub transp: bool,
    pub bg_img: Option<String>,
    pub is_3d: bool,
    pub theme: String,
    pub piece_set: String,
    pub theme_3d: String,
    pub piece_set_3d: String,
    pub sound_set: String,
    pub blindfold: Option<u8>,
    pub auto_queen: u8,
    pub auto_threefold: u8,
    pub takeback: u8,
    pub moretime: u8,
    pub clock_tenths: u8,
    pub clock_bar: bool,
    pub clock_sound: bool,
    pub premove: bool,
    pub animation: u8,
    pub piece_notation: u8,
    pub captured: bool,
    pub follow: bool,
    pub highlight: bool,
    pub destination: bool,
    pub coords: u8,
    pub replay: u8,
    pub challenge: u8,
    pub message: u8,
    pub coord_color: Option<u8>,
    pub submit_move: u8,
    pub confirm_resign: u8,
    pub insight_share: u8,
    pub keyboard_move: u8,
    pub voice_move: bool,
    pub zen: u8,
    pub ratings: u8,
    pub move_event: u8,
    pub rook_castle: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Preferences {
    pub prefs: UserPreferences,
    pub language: String,
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
    #[serde(default)]
    pub online: bool,
    #[serde(default)]
    pub playing: bool,
    #[serde(default)]
    pub streaming: bool,
    #[serde(default)]
    pub patron: bool,
    pub rating: Option<u16>,
    #[serde(default)]
    pub provisional: bool,
    pub lag: Option<u16>,
    #[serde(rename = "gameId")]
    pub game_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct MinimalUser {
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
    #[serde(default)]
    pub patron: bool,
    pub flair: Option<String>,
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
    pub play_time: PlayTime,
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
pub struct PlayTime {
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct KidMode {
    pub kid: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Timeline {
    pub entries: Vec<TimelineEvent>,
    pub users: BTreeMap<String, MinimalUser>,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct TimelineEvent {
    pub r#type: TimelineEventType,
    pub data: Option<TimelineEventData>,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub date: PrimitiveDateTime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimelineEventType {
    Follow,
    TeamJoin,
    TeamCreate,
    ForumPost,
    UblogPost,
    TourJoin,
    GameEnd,
    SimulCreate,
    SimulJoin,
    StudyLike,
    PlanStart,
    PlanRenew,
    BlogPost,
    UblogPostLike,
    StreamStart,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(untagged)]
pub enum TimelineEventData {
    Follow {
        u1: String,
        u2: String,
    },
    Team {
        #[serde(rename = "userId")]
        user_id: String,
        #[serde(rename = "teamId")]
        team_id: String,
    },
    ForumPost {
        #[serde(rename = "userId")]
        user_id: String,
        #[serde(rename = "topicId")]
        topic_id: String,
        #[serde(rename = "topicName")]
        topic_name: String,
        #[serde(rename = "postId")]
        post_id: String,
    },
    UblogPost {
        #[serde(rename = "userId")]
        user_id: String,
        id: String,
        slug: String,
        title: String,
    },
    TourJoin {
        #[serde(rename = "userId")]
        user_id: String,
        #[serde(rename = "tourId")]
        tour_id: String,
        #[serde(rename = "tourName")]
        tour_name: String,
    },
    GameEnd {
        #[serde(rename = "fullId")]
        full_id: String,
        perf: PerfType,
        opponent: String,
        win: Option<bool>,
    },
    Simul {
        #[serde(rename = "userId")]
        user_id: String,
        #[serde(rename = "simulId")]
        simul_id: String,
        #[serde(rename = "simulName")]
        simul_name: String,
    },
    StudyLike {
        #[serde(rename = "userId")]
        user_id: String,
        #[serde(rename = "studyId")]
        study_id: String,
        #[serde(rename = "studyName")]
        study_name: String,
    },
    PlanStart {
        #[serde(rename = "userId")]
        user_id: String,
    },
    PlanRenew {
        #[serde(rename = "userId")]
        user_id: String,
        months: u16,
    },
    BlogPost {
        id: String,
        slug: String,
        title: String,
    },
    UblogPostLike {
        #[serde(rename = "userId")]
        user_id: String,
        id: String,
        title: String,
    },
    StreamStart {
        #[serde(rename = "userId")]
        user_id: String,
        title: String,
    },
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct RealtimeUser {
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
    pub flair: Option<String>,
    #[serde(rename = "playingId")]
    pub playing_id: Option<String>,
    #[serde(default)]
    pub online: bool,
    #[serde(default)]
    pub playing: RealtimeUserPlaying,
    #[serde(default)]
    pub streaming: bool,
    #[serde(default)]
    pub patron: bool,
    #[serde(default)]
    pub signal: Option<u8>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(untagged)]
pub enum RealtimeUserPlaying {
    Playing(bool),
    PlayingDetails {
        id: String,
        clock: Option<String>,
        variant: Option<String>,
    },
}

impl Default for RealtimeUserPlaying {
    fn default() -> Self {
        Self::Playing(false)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct TopUsers {
    pub bullet: Vec<TopUser>,
    pub blitz: Vec<TopUser>,
    pub rapid: Vec<TopUser>,
    pub classical: Vec<TopUser>,
    pub ultra_bullet: Vec<TopUser>,
    pub chess960: Vec<TopUser>,
    pub crazyhouse: Vec<TopUser>,
    pub antichess: Vec<TopUser>,
    pub atomic: Vec<TopUser>,
    pub horde: Vec<TopUser>,
    pub king_of_the_hill: Vec<TopUser>,
    pub racing_kings: Vec<TopUser>,
    pub three_check: Vec<TopUser>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct TopUser {
    pub id: String,
    pub username: String,
    pub perfs: BTreeMap<String, TopUserPerf>,
    pub title: Option<Title>,
    #[serde(default)]
    pub online: bool,
    #[serde(default)]
    pub patron: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct TopUserPerf {
    pub rating: u16,
    pub progress: i16,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct TopUserLeaderboard {
    pub users: Vec<TopUser>,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Trophy {
    PerfTop {
        perf: String,
        top: u8,
        name: String,
    },
    Moderator {
        name: String,
        #[serde_as(as = "TimestampMilliSeconds")]
        date: PrimitiveDateTime,
        icon: String,
        url: String,
    },
    Developer {
        name: String,
        #[serde_as(as = "TimestampMilliSeconds")]
        date: PrimitiveDateTime,
        icon: String,
        url: String,
    },
    Verified {
        name: String,
        #[serde_as(as = "TimestampMilliSeconds")]
        date: PrimitiveDateTime,
        icon: String,
    },
    ContentTeam {
        name: String,
        #[serde_as(as = "TimestampMilliSeconds")]
        date: PrimitiveDateTime,
        icon: String,
    },
    #[serde(
        alias = "marathonWinner",
        alias = "marathonTopTen",
        alias = "marathonTopFifty",
        alias = "marathonTopHundred"
    )]
    MarathonTop {
        name: String,
        #[serde_as(as = "TimestampMilliSeconds")]
        date: PrimitiveDateTime,
        icon: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct RatingHistory {
    pub name: String,
    pub points: Vec<(u16, u8, u8, u16)>,
}
