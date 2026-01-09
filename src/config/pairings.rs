use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::game::{CorrespondenceDays, Rules, VariantMode};

/// Configuration for creating bulk pairings using [`Licheszter::bulk_pairings_create()`](fn@crate::client::Licheszter::bulk_pairings_create).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BulkPairingOptions {
    #[serde(rename = "clock.limit")]
    clock_limit: Option<u16>,
    #[serde(rename = "clock.increment")]
    clock_increment: Option<u8>,
    days: Option<u8>,
    fen: Option<String>,
    message: Option<String>,
    pair_at: Option<u64>,
    players: Option<Vec<String>>,
    rated: Option<bool>,
    rules: Option<Vec<Rules>>,
    start_clocks_at: Option<u64>,
    variant: Option<VariantMode>,
}

impl BulkPairingOptions {
    /// Create a new instance of [`BulkPairingOptions`] with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Determines the clock settings for the game.
    /// Invalid clock limit values default to 0 and clock increment values over 180 default to 180.
    /// Defaults to a correspondence game.
    #[must_use]
    pub fn clock(mut self, clock_limit: u16, clock_increment: u8) -> Self {
        // Check if the clock limit value is valid
        match clock_limit {
            0 | 15 | 30 | 45 | 60 | 90 => self.clock_limit = Some(clock_limit),
            x if x % 60 == 0 && x <= 10800 => self.clock_limit = Some(clock_limit),
            _ => self.clock_limit = Some(0),
        }

        // Check if the clock increment value is valid
        if clock_increment > 180 {
            self.clock_increment = Some(180);
        } else {
            self.clock_increment = Some(clock_increment);
        }

        self
    }

    /// Determines the length of a correspondence game in days.
    /// Clock settings must be omitted.
    /// Defaults to unlimited.
    #[must_use]
    pub fn days(mut self, days: CorrespondenceDays) -> Self {
        self.days = Some(days as u8);
        self
    }

    /// Determines a custom FEN string for the game.
    /// Requires the variant to be set as Standard, FromPosition or Chess960.
    /// Also requires the challenge *NOT* to be rated.
    /// Defaults to the default chess starting position.
    #[must_use]
    pub fn fen(mut self, fen: &str) -> Self {
        self.fen = Some(fen.to_string());
        self
    }

    /// Set a custom message that is sent to each player when the game is created.
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    /// Determines when the games will be created.
    /// The timestamp is in MILLISECONDS.
    /// Up to 7 days into the future.
    /// If not set, the games will start immediately.
    pub fn pair_at(mut self, timestamp: u64) -> Self {
        self.pair_at = Some(timestamp);
        self
    }

    /// Authentication tokens of all players to be paired.
    /// The correct order is `vec!["white1", "black1", "white2", "black2"]`, where the number represents the game.
    pub fn players(mut self, players: Vec<(&str, &str)>) -> Self {
        self.players = Some(
            players
                .iter()
                .map(|(white, black)| format!("{white}:{black}"))
                .collect(),
        );
        self
    }

    /// Determines whether the game is rated or not.
    /// Defaults to false.
    #[must_use]
    pub fn rated(mut self, rated: bool) -> Self {
        self.rated = Some(rated);
        self
    }

    /// Determines if any extra game rules will be set.
    /// Does not have a default value.
    #[must_use]
    pub fn rules(mut self, rules: Vec<Rules>) -> Self {
        self.rules = Some(rules);
        self
    }

    /// Determines when the clocks will be automatically started.
    /// The timestamp is in MILLISECONDS.
    /// Up to 7 days into the future.
    /// If not set, the clocks will not start automatically.
    /// Note that clocks can start regardless of this setting if players start making moves in the game.
    #[must_use]
    pub fn start_clocks_at(mut self, timestamp: u64) -> Self {
        self.start_clocks_at = Some(timestamp);
        self
    }

    /// Determines the game variant.
    /// Defaults to Standard.
    #[must_use]
    pub fn variant(mut self, variant: VariantMode) -> Self {
        self.variant = Some(variant);
        self
    }
}
