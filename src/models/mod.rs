// Core models used across multiple APIs
pub mod board;
pub mod chat;
pub mod common;
pub mod game;
pub mod user;

// API-specific models
#[cfg(feature = "analysis")]
pub mod analysis;

#[cfg(feature = "challenges")]
pub mod challenge;

#[cfg(feature = "fide")]
pub mod fide;

#[cfg(feature = "pairings")]
pub mod pairings;

#[cfg(feature = "puzzles")]
pub mod puzzle;

#[cfg(feature = "simuls")]
pub mod simul;

#[cfg(feature = "tv")]
pub mod tv;

#[cfg(feature = "openings")]
pub mod openings;

#[cfg(feature = "tablebase")]
pub mod tablebase;
