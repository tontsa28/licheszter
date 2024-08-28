use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::game::{Color, CorrespondenceDays, VariantMode};

/// Optional configuration for seeking opponents using [`Licheszter::board_seek()`](fn@crate::client::Licheszter::board_seek).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct SeekOptions {
    rated: Option<bool>,
    time: Option<u8>,
    increment: Option<u8>,
    days: Option<u8>,
    color: Option<Color>,
    variant: Option<VariantMode>,
    #[serde(rename = "ratingRange")]
    rating_range: Option<String>,
}

impl SeekOptions {
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
    /// Invalid time and increment values default to 180.
    /// Defaults to a correspondence game.
    #[must_use]
    pub fn clock(mut self, time: u8, increment: u8) -> Self {
        // Check if the clock limit value is valid
        if time > 180 {
            self.time = Some(180);
        } else {
            self.time = Some(time);
        }

        // Check if the clock increment value is valid
        if increment > 180 {
            self.increment = Some(180);
        } else {
            self.increment = Some(increment);
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

    /// Determines the rating range of the potential opponent.
    /// Does not have a default value.
    pub fn rating_range(mut self, min: u16, max: u16) -> Self {
        let range = format!("{min}-{max}");
        self.rating_range = Some(range);
        self
    }
}
