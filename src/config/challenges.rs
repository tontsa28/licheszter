use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::game::{Color, CorrespondenceDays, Rules, VariantMode};

/// Optional configuration for creating challenges using [`Licheszter::challenge_create()`](fn@crate::client::Licheszter::challenge_create).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct ChallengeOptions {
    rated: Option<bool>,
    clock_limit: Option<u16>,
    clock_increment: Option<u8>,
    days: Option<u8>,
    color: Option<Color>,
    variant: Option<VariantMode>,
    fen: Option<String>,
    rules: Option<Vec<Rules>>,
}

impl ChallengeOptions {
    /// Create a new instance of [`ChallengeOptions`] with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Determines whether the game is rated or not.
    /// Defaults to false.
    #[must_use]
    pub fn rated(mut self, rated: bool) -> Self {
        self.rated = Some(rated);
        self
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

    /// Determines the color the challenger will get to play.
    /// Defaults to random.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Determines the game variant.
    /// Defaults to Standard.
    #[must_use]
    pub fn variant(mut self, variant: VariantMode) -> Self {
        self.variant = Some(variant);
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

    /// Determines if any extra game rules will be set.
    /// Does not have a default value.
    #[must_use]
    pub fn rules(mut self, rules: Vec<Rules>) -> Self {
        self.rules = Some(rules);
        self
    }
}

/// Optional configuration for creating challenges using [`Licheszter::challenge_ai()`](fn@crate::client::Licheszter::challenge_ai).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct AIChallengeOptions {
    clock_limit: Option<u16>,
    clock_increment: Option<u8>,
    days: Option<u8>,
    color: Option<Color>,
    variant: Option<VariantMode>,
    fen: Option<String>,
}

impl AIChallengeOptions {
    /// Create a new instance of [`AIChallengeOptions`] with default configuration.
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

    /// Determines the color the challenger will get to play.
    /// Defaults to random.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Determines the game variant.
    /// Defaults to Standard.
    #[must_use]
    pub fn variant(mut self, variant: VariantMode) -> Self {
        self.variant = Some(variant);
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
}

/// Optional configuration for creating challenges using [`Licheszter::challenge_create_open()`](fn@crate::client::Licheszter::challenge_create_open).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct OpenChallengeOptions {
    rated: Option<bool>,
    clock_limit: Option<u16>,
    clock_increment: Option<u8>,
    days: Option<u8>,
    variant: Option<VariantMode>,
    fen: Option<String>,
    name: Option<String>,
    rules: Option<Vec<Rules>>,
    users: Option<Vec<String>>,
    #[serde(rename = "expiresAt")]
    expires_at: Option<u64>,
}

impl OpenChallengeOptions {
    /// Create a new instance of [`OpenChallengeOptions`] with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Determines whether the game is rated or not.
    /// Defaults to false.
    #[must_use]
    pub fn rated(mut self, rated: bool) -> Self {
        self.rated = Some(rated);
        self
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

    /// Determines the game variant.
    /// Defaults to Standard.
    #[must_use]
    pub fn variant(mut self, variant: VariantMode) -> Self {
        self.variant = Some(variant);
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

    /// Determines an optional name for the challenge.
    /// This name will be displayed on the challenge page.
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Determines if any extra game rules will be set.
    /// Does not have a default value.
    #[must_use]
    pub fn rules(mut self, rules: Vec<Rules>) -> Self {
        self.rules = Some(rules);
        self
    }

    /// Determines an optional pair of usernames.
    /// If set, only these users will be allowed to join the game.
    /// The first username gets the white pieces.
    pub fn users(mut self, users: Vec<&str>) -> Self {
        self.users = Some(users.iter().map(|s| s.to_string()).collect::<Vec<String>>());
        self
    }

    /// Determines when the challenge will expire.
    /// The timestamp is in MILLISECONDS.
    /// Can't be more than 2 weeks.
    pub fn expires_at(mut self, timestamp: u64) -> Self {
        self.expires_at = Some(timestamp);
        self
    }
}
