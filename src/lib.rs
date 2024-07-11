//#![warn(clippy::pedantic)]

//! # licheszter
//! Licheszter is a Rust library that aims to wrap the entire Lichess API.
//! Currently, the library is under active development and more features are being added.
//! The goal is to eventually be the most complete and overall #1 Lichess API wrapper written in Rust.
//! Whether you're looking for your first open source Rust project to contribute to or
//! you're just generally interested in the project, check the [Contributions](#contributions) section
//! for more information.
//!
//! ### WARNING:
//! **The project is unstable to use in production until the version 1.0.0
//! since no guarantees about breaking changes can be made.
//! Use at your own risk and prepare to face breaking changes more or less often.**
//!
//! For additional information, check the [GitHub repository](https://github.com/tontsa28/licheszter).

pub mod api;
pub mod client;
pub mod config;
pub mod error;
pub mod models;
