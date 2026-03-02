use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    common::Color,
    game::{CorrespondenceDays, Rules, VariantMode},
};

/// Optional configuration for creating challenges using [`Licheszter::challenge_create()`](fn@crate::client::Licheszter::challenge_create).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
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

    /// Determines the color the challenger will get to play.
    /// Defaults to random.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
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

impl_clock_game_methods!(ChallengeOptions);

/// Optional configuration for creating challenges using [`Licheszter::challenge_ai()`](fn@crate::client::Licheszter::challenge_ai).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
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

    /// Determines the color the challenger will get to play.
    /// Defaults to random.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl_clock_game_methods!(AIChallengeOptions);

/// Optional configuration for creating challenges using [`Licheszter::challenge_create_open()`](fn@crate::client::Licheszter::challenge_create_open).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
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

    /// Determines an optional name for the challenge.
    /// This name will be displayed on the challenge page.
    #[must_use]
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
    #[must_use]
    pub fn users(mut self, users: Vec<&str>) -> Self {
        self.users = Some(users.iter().map(ToString::to_string).collect::<Vec<String>>());
        self
    }

    /// Determines when the challenge will expire.
    /// The timestamp is in MILLISECONDS.
    /// Can't be more than 2 weeks.
    #[must_use]
    pub fn expires_at(mut self, timestamp: u64) -> Self {
        self.expires_at = Some(timestamp);
        self
    }
}

impl_clock_game_methods!(OpenChallengeOptions);
