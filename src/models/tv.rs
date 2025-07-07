use serde::{Deserialize, Serialize};

use crate::models::{game::FinalColor, user::MinimalUser};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct TvGames {
    pub bot: TvGame,
    pub blitz: TvGame,
    pub racing_kings: TvGame,
    pub ultra_bullet: TvGame,
    pub bullet: TvGame,
    pub classical: TvGame,
    pub three_check: TvGame,
    pub antichess: TvGame,
    pub computer: TvGame,
    pub horde: TvGame,
    pub rapid: TvGame,
    pub atomic: TvGame,
    pub crazyhouse: TvGame,
    pub chess960: TvGame,
    pub king_of_the_hill: TvGame,
    pub best: TvGame,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct TvGame {
    pub user: MinimalUser,
    pub rating: u16,
    #[serde(rename = "gameId")]
    pub game_id: String,
    pub color: FinalColor,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
#[serde(tag = "t")]
pub enum TvGameEvent {
    Featured {
        #[serde(rename = "d")]
        event: FeaturedEvent,
    },
    Fen {
        #[serde(rename = "d")]
        event: FenEvent,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct FeaturedEvent {
    pub id: String,
    pub orientation: FinalColor,
    pub players: Vec<TvPlayer>,
    pub fen: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct FenEvent {
    pub fen: String,
    pub lm: String,
    pub wc: u16,
    pub bc: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct TvPlayer {
    pub color: FinalColor,
    pub user: MinimalUser,
    pub rating: u16,
    pub seconds: u16,
}
