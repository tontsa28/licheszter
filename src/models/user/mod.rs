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
    BasicUser, ChallengeUser, LightUser, MinimalUser, NowPlaying, NowPlayingOpponent, PlayTime,
    Profile, RealtimeUser, RealtimeUserPlaying, StreamingUser, UserAutocomplete,
};

// Temporarily keep these types in this file until they are extracted
// TODO: Extract to activity.rs and social.rs in future refactoring

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, TimestampMilliSeconds};
use time::PrimitiveDateTime;

use crate::models::game::Pace;

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
    Study {
        #[serde(rename = "userId")]
        user_id: String,
        #[serde(rename = "studyId")]
        study_id: String,
        #[serde(rename = "studyName")]
        study_name: String,
    },
    Plan {
        #[serde(rename = "userId")]
        user_id: String,
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
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct UserNote {
    pub from: MinimalUser,
    pub to: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Crosstable {
    pub users: BTreeMap<String, f32>,
    pub matchup: Option<CrosstableMatchup>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct CrosstableMatchup {
    pub users: BTreeMap<String, u32>,
    pub nbgames: u32,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct StreamDetails {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub description: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct StreamerDetails {
    pub headline: String,
    pub description: String,
    pub twitch: Option<String>,
    pub you_tube: Option<String>,
    pub image: Option<String>,
}

// Activity types
#[skip_serializing_none]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct UserActivity {
    pub puzzles: Option<PuzzlesActivity>,
    pub games: Option<GamesActivity>,
    pub posts: Option<PostActivity>,
    pub practice: Option<PracticeActivity>,
    pub simuls: Option<u32>,
    pub correspondences: Option<Vec<CorrespondenceActivity>>,
    pub correspondenceends: Option<Vec<CorrespondenceEndsActivity>>,
    pub correspondencemoves: Option<Vec<CorrespondenceMovesActivity>>,
    pub follows: Option<FollowsActivity>,
    pub teams: Option<Vec<TeamActivity>>,
    pub tournaments: Option<Vec<TournamentsActivity>>,
    pub patron: Option<Patron>,
    pub stream: Option<bool>,
    pub study: Option<StudyActivity>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct CorrespondenceEndsActivity {
    pub nb: u32,
    pub score: ScoreActivity,
    pub rp: Option<RpActivity>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct CorrespondenceActivity {
    pub nb: u32,
    pub moves: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct GameActivity {
    pub rp: Option<RpActivity>,
    pub score: ScoreActivity,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct OpponentActivity {
    pub user: MinimalUser,
    pub nb: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ScoreActivity {
    pub win: u32,
    pub loss: u32,
    pub draw: u32,
    pub rp: Option<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct RpActivity {
    pub before: i32,
    pub after: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct CorrespondenceMovesActivity {
    #[serde(default)]
    pub nb: u32,
    #[serde(default)]
    pub games: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct FollowsActivity {
    pub in_: FollowsActivityList,
    pub out: FollowsActivityList,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct FollowsActivityList {
    pub ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(untagged)]
pub enum GamesActivity {
    Games(Vec<BTreeMap<PerfType, GameActivity>>),
    Opponents(Vec<Vec<OpponentActivity>>),
}

impl GamesActivity {
    pub fn games(&self) -> Option<&Vec<BTreeMap<PerfType, GameActivity>>> {
        match self {
            Self::Games(games) => Some(games),
            _ => None,
        }
    }

    pub fn opponents(&self) -> Option<&Vec<Vec<OpponentActivity>>> {
        match self {
            Self::Opponents(opponents) => Some(opponents),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ActivityInterval {
    pub start: i64,
    pub end: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct PuzzlesActivity {
    pub score: BTreeMap<String, UserPuzzleModePerf>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct TeamActivity {
    pub team_id: String,
    pub team_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct TournamentsActivity {
    pub nb: u32,
    pub best: Vec<BestTournament>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct BestTournament {
    pub tour: Tournament,
    pub nb_games: u32,
    pub score: u32,
    pub rank: u32,
    pub rank_percent: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Patron {
    pub months: Option<u32>,
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
#[serde(rename_all = "camelCase")]
pub struct PracticeActivity {
    pub nb: u32,
    pub time: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct StudyActivity {
    pub nb: u32,
}
