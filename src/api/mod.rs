pub mod challenges;
pub mod misc;
pub mod relations;

#[cfg(feature = "openings")]
pub mod openings;

#[cfg(feature = "tablebase")]
pub mod tablebase;

#[cfg(feature = "board")]
pub mod board;

#[cfg(feature = "bot")]
pub mod bot;
