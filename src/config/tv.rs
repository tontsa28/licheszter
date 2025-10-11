use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TvChannel {
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Chess960,
    KingOfTheHill,
    ThreeCheck,
    Antichess,
    Atomic,
    Horde,
    RacingKings,
    Crazyhouse,
    UltraBullet,
    Bot,
    Computer,
}

impl Display for TvChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bullet => write!(f, "bullet"),
            Self::Blitz => write!(f, "blitz"),
            Self::Rapid => write!(f, "rapid"),
            Self::Classical => write!(f, "classical"),
            Self::Chess960 => write!(f, "chess960"),
            Self::KingOfTheHill => write!(f, "kingOfTheHill"),
            Self::ThreeCheck => write!(f, "threeCheck"),
            Self::Antichess => write!(f, "antichess"),
            Self::Atomic => write!(f, "atomic"),
            Self::Horde => write!(f, "horde"),
            Self::RacingKings => write!(f, "racingKings"),
            Self::Crazyhouse => write!(f, "crazyhouse"),
            Self::UltraBullet => write!(f, "ultraBullet"),
            Self::Bot => write!(f, "bot"),
            Self::Computer => write!(f, "computer"),
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct TvChannelOptions {
    nb: Option<u8>,
    moves: Option<bool>,
    tags: Option<bool>,
    clocks: Option<bool>,
    opening: Option<bool>,
}

impl TvChannelOptions {
    /// Create a new instance of [`TvChannelOptions`] with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Determines how many games will be fetched.
    #[must_use]
    pub fn amount(mut self, amount: u8) -> Self {
        self.nb = Some(amount);
        self
    }

    /// Include the game moves.
    #[must_use]
    pub fn moves(mut self, moves: bool) -> Self {
        self.moves = Some(moves);
        self
    }

    /// Include the game tags.
    #[must_use]
    pub fn tags(mut self, tags: bool) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Include the game's clock status when available.
    #[must_use]
    pub fn clocks(mut self, clocks: bool) -> Self {
        self.clocks = Some(clocks);
        self
    }

    /// Include the game's opening name.
    #[must_use]
    pub fn opening(mut self, opening: bool) -> Self {
        self.opening = Some(opening);
        self
    }
}
