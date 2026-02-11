#[cfg(feature = "account")]
pub mod account;

#[cfg(feature = "analysis")]
pub mod analysis;

#[cfg(feature = "challenges")]
pub mod challenges;

#[cfg(feature = "fide")]
pub mod fide;

#[cfg(feature = "games")]
pub mod games;

#[cfg(feature = "messaging")]
pub mod messaging;

pub mod misc;

#[cfg(feature = "pairings")]
pub mod pairings;

#[cfg(feature = "puzzles")]
pub mod puzzles;

#[cfg(feature = "relations")]
pub mod relations;

#[cfg(feature = "simuls")]
pub mod simuls;

#[cfg(feature = "tv")]
pub mod tv;

#[cfg(feature = "users")]
pub mod users;

#[cfg(feature = "openings")]
pub mod openings;

#[cfg(feature = "tablebase")]
pub mod tablebase;

#[cfg(feature = "board")]
pub mod board;

#[cfg(feature = "bot")]
pub mod bot;
