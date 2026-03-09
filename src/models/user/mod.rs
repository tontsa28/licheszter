//! User-related models and types.
//!
//! This module is organized into submodules for better structure:
//! - [`core`] - Core user types (User, Preferences, etc.)
//! - [`perf`] - Performance and rating types
//! - [`top`] - Leaderboard and trophy types
//! - [`variants`] - User variant types (LightUser, MinimalUser, etc.)
//!
//! All types are re-exported at the module level for convenience, allowing:
//! ```ignore
//! use licheszter::models::user::User;  // Via re-export (recommended)
//! use licheszter::models::user::core::User;  // Direct path (also valid)
//! ```
//!
//! Both patterns are supported to maintain flexibility. The re-exports provide
//! a cleaner API while preserving access to types in their logical groupings.

// Core user types
pub mod core;
pub mod perf;
pub mod top;
pub mod variants;

// Re-export commonly used types for convenience
pub use core::{Email, KidMode, Preferences, User, UserPreferences};
pub use perf::{
    PerfCount, PerfDetails, PerfGlicko, PerfPlayStreak, PerfPoint, PerfRecord, PerfRecords,
    PerfResultStreak, PerfStats, PerfStreak, PerfStreakDate, PerfStreakRecord, PerfType,
    RatingHistory, UserName, UserPerf, UserPerformance, UserPerfs, UserPuzzleModePerf,
};
pub use top::{TopUser, TopUserLeaderboard, TopUserPerf, TopUsers, Trophy};
pub use variants::{
    BasicUser, ChallengeUser, LightUser, MinimalUser, PlayTime, Profile, RealtimeUser,
    RealtimeUserPlaying, StreamingUser, UserAutocomplete,
};

// Activity and social types are currently defined in this module.
// They could be organized into separate submodules (activity.rs and social.rs)
// as a future enhancement if the file grows significantly larger.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::PrimitiveDateTime;

use crate::models::{
    common::FinalColor,
    game::{Pace, Speed, VariantMode},
};

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
        perf: Pace,
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

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct UserNote {
    pub from: MinimalUser,
    pub to: MinimalUser,
    pub text: String,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub date: PrimitiveDateTime,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Crosstable {
    pub users: BTreeMap<String, f32>,
    #[serde(rename = "nbGames")]
    pub nb_games: u32,
    pub matchup: Option<CrosstableMatchup>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct CrosstableMatchup {
    pub users: BTreeMap<String, f32>,
    #[serde(rename = "nbGames")]
    pub nb_games: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct StreamDetails {
    pub service: String,
    pub status: String,
    pub lang: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct StreamerDetails {
    pub name: String,
    pub headline: Option<String>,
    pub description: Option<String>,
    pub twitch: Option<String>,
    #[serde(rename = "youTube")]
    pub youtube: Option<String>,
    pub image: Option<String>,
}

// Activity types
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct UserActivity {
    pub interval: ActivityInterval,
    pub games: Option<GamesActivity>,
    pub correspondence_moves: Option<CorrespondenceMovesActivity>,
    pub correspondence_ends: Option<CorrespondenceEndsActivity>,
    #[serde(default)]
    pub teams: Vec<TeamActivity>,
    pub puzzles: Option<PuzzlesActivity>,
    pub tournaments: Option<TournamentsActivity>,
    #[serde(default)]
    pub practice: Vec<PracticeActivity>,
    pub follows: Option<FollowsActivity>,
    pub racer: Option<UserPuzzleModePerf>,
    pub storm: Option<UserPuzzleModePerf>,
    pub streak: Option<UserPuzzleModePerf>,
    #[serde(default)]
    pub simuls: Vec<String>,
    #[serde(default)]
    pub studies: Vec<StudyActivity>,
    #[serde(default)]
    pub posts: Vec<PostActivity>,
    pub patron: Option<Patron>,
    #[serde(default)]
    pub stream: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct CorrespondenceEndsActivity {
    pub correspondence: CorrespondenceActivity,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct CorrespondenceActivity {
    pub score: ScoreActivity,
    pub games: Vec<GameActivity>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct GameActivity {
    pub id: String,
    pub color: FinalColor,
    pub url: String,
    pub variant: Option<VariantMode>,
    pub speed: Option<Speed>,
    pub perf: Option<PerfType>,
    #[serde(default)]
    pub rated: bool,
    pub opponent: OpponentActivity,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct OpponentActivity {
    pub user: String,
    pub rating: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ScoreActivity {
    pub win: u16,
    pub loss: u16,
    pub draw: u16,
    pub rp: RpActivity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct RpActivity {
    pub before: u16,
    pub after: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct CorrespondenceMovesActivity {
    #[serde(rename = "nb")]
    pub amount: u32,
    pub games: Vec<GameActivity>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct FollowsActivity {
    pub r#in: Option<FollowsActivityList>,
    pub out: Option<FollowsActivityList>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct FollowsActivityList {
    pub ids: Vec<String>,
    #[serde(default)]
    #[serde(rename = "nb")]
    pub amount: u32,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct GamesActivity {
    pub bullet: Option<ScoreActivity>,
    pub blitz: Option<ScoreActivity>,
    pub rapid: Option<ScoreActivity>,
    pub classical: Option<ScoreActivity>,
    pub standard: Option<ScoreActivity>,
    pub chess960: Option<ScoreActivity>,
    pub atomic: Option<ScoreActivity>,
    pub racing_kings: Option<ScoreActivity>,
    pub ultra_bullet: Option<ScoreActivity>,
    pub king_of_the_hill: Option<ScoreActivity>,
    pub correspondence: Option<ScoreActivity>,
    pub horde: Option<ScoreActivity>,
    pub puzzle: Option<ScoreActivity>,
    pub three_check: Option<ScoreActivity>,
    pub crazyhouse: Option<ScoreActivity>,
}

#[serde_as]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ActivityInterval {
    #[serde_as(as = "TimestampMilliSeconds")]
    pub start: PrimitiveDateTime,
    #[serde_as(as = "TimestampMilliSeconds")]
    pub end: PrimitiveDateTime,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzlesActivity {
    pub score: ScoreActivity,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct TeamActivity {
    pub url: String,
    pub name: String,
    pub flair: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct TournamentsActivity {
    #[serde(default)]
    #[serde(rename = "nb")]
    pub amount: u32,
    #[serde(default)]
    pub best: Vec<BestTournament>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct BestTournament {
    pub tournament: Tournament,
    pub nb_games: i64,
    pub score: i64,
    pub rank: i64,
    pub rank_percent: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Tournament {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Patron {
    pub months: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct PostActivity {
    pub topic_url: String,
    pub topic_name: String,
    pub posts: Vec<Post>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Post {
    pub url: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PracticeActivity {
    pub url: String,
    pub name: String,
    #[serde(rename = "nbPositions")]
    pub positions_amount: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct StudyActivity {
    pub id: String,
    pub name: String,
}
