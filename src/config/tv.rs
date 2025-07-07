use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
