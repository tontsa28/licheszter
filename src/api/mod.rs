pub mod account;
pub mod analysis;
pub mod challenges;
pub mod messaging;
pub mod misc;
pub mod puzzles;
pub mod relations;
pub mod simuls;

#[cfg(feature = "openings")]
pub mod openings;

#[cfg(feature = "tablebase")]
pub mod tablebase;

#[cfg(feature = "board")]
pub mod board;

#[cfg(feature = "bot")]
pub mod bot;
