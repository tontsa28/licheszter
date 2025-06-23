use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserStatusOptions {
    with_signal: Option<bool>,
    with_game_ids: Option<bool>,
    with_game_metas: Option<bool>,
}

impl UserStatusOptions {
    /// Create a new instance of [`UserStatusOptions`] with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Determines whether the network signal of a player is included in the response or not.
    #[must_use]
    pub fn signal(mut self, signal: bool) -> Self {
        self.with_signal = Some(signal);
        self
    }

    /// Determines whether the ID of the game being played is included in the response or not.
    #[must_use]
    pub fn game_ids(mut self, game_ids: bool) -> Self {
        self.with_game_ids = Some(game_ids);
        self
    }

    /// Include metadata from the game being played.
    /// Does not work if `game_ids(true)` is also used.
    #[must_use]
    pub fn game_metas(mut self, game_metas: bool) -> Self {
        self.with_game_metas = Some(game_metas);
        self
    }
}
