use super::{
    chat::ChatLine,
    game::{GameEventInfo, GameFull, GameState, OpponentGone},
};

#[cfg(feature = "challenges")]
use super::challenge::Challenge;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Event {
    GameStart { game: GameEventInfo },
    GameFinish { game: GameEventInfo },
    #[cfg(feature = "challenges")]
    Challenge { challenge: Challenge },
    #[cfg(feature = "challenges")]
    ChallengeCanceled { challenge: Challenge },
    #[cfg(feature = "challenges")]
    ChallengeDeclined { challenge: Challenge },
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
