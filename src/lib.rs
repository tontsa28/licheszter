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
//! *NOTE:* it is forbidden to use the Board API (crate feature `board`) for projects that involve use of chess engines or other things that can be interpreted as cheating.
//! The feature `bot` is enabled by default to prevent accidents.
//! If you're not developing anything that uses external chess assistance, you can enable `board` feature if you wish not to use the Bot API.
//! You can also choose to use neither if you simply don't need that functionality by disabling default features.
//! In this case, do bear in mind that you may need to opt in to other, normally default features manually as well.
//! This project and its developers are NOT responsible for any account bans that may occur from the misuse of the Board API.
//!
//! For additional information, check the [GitHub repository](https://github.com/tontsa28/licheszter).

pub mod api;
pub mod client;
pub mod config;
pub mod error;
pub mod models;
