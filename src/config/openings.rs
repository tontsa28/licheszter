use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    game::{GameType, Speed, VariantMode},
    openings::OpeningRatings,
};

/// Optional configuration for querying Masters openings using [`openings().masters()`](fn@crate::client::OpeningsApi::masters).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct MastersOpeningsOptions {
    fen: Option<String>,
    play: Option<Vec<String>>,
    since: Option<String>,
    until: Option<String>,
    moves: Option<u16>,
    #[serde(rename = "topGames")]
    top_games: Option<u8>,
}

impl MastersOpeningsOptions {
    /// Create a new instance of [`MastersOpeningsOptions`] with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Determines the FEN of the root position.
    #[must_use]
    pub fn fen(mut self, fen: &str) -> Self {
        self.fen = Some(fen.to_string());
        self
    }

    /// Determines the sequence of legal moves in UCI notation.
    /// Play additional moves starting from the FEN position.
    /// Required to find an opening game, if the FEN is not an exact match for a named position.
    #[must_use]
    pub fn play(mut self, play: &[&str]) -> Self {
        self.play = Some(play.iter().map(|s| s.to_string()).collect::<Vec<String>>());
        self
    }

    /// Include only games from this year or later.
    #[must_use]
    pub fn since(mut self, since: u16) -> Self {
        self.since = Some(since.to_string());
        self
    }

    // Internal helper to use strings to include only games from this year or later.
    #[must_use]
    fn since_str(mut self, since: &str) -> Self {
        self.since = Some(since.to_string());
        self
    }

    /// Include only games from this year or earlier.
    #[must_use]
    pub fn until(mut self, until: u16) -> Self {
        self.until = Some(until.to_string());
        self
    }

    // Internal helper to use strings to include only games from this year or earlier.
    #[must_use]
    fn until_str(mut self, until: &str) -> Self {
        self.until = Some(until.to_string());
        self
    }

    /// Determines the number of most common moves to display.
    #[must_use]
    pub fn moves(mut self, moves: u16) -> Self {
        self.moves = Some(moves);
        self
    }

    /// Determines the number of top games to display.
    #[must_use]
    pub fn top_games(mut self, top_games: u8) -> Self {
        self.top_games = Some(top_games);
        self
    }
}

/// Optional configuration for querying Lichess openings using [`openings().lichess()`](fn@crate::client::OpeningsApi::lichess).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename = "camelCase")]
pub struct LichessOpeningsOptions {
    variant: Option<VariantMode>,
    speeds: Option<Vec<Speed>>,
    ratings: Option<Vec<u16>>,
    recent_games: Option<u8>,
    history: Option<bool>,
    #[serde(flatten)]
    inner: MastersOpeningsOptions,
}

impl LichessOpeningsOptions {
    /// Create a new instance of [`LichessOpeningsOptions`] with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Determines the game variant.
    /// Defaults to Standard.
    #[must_use]
    pub fn variant(mut self, variant: VariantMode) -> Self {
        self.variant = Some(variant);
        self
    }

    /// Determines the game speeds to filter by.
    #[must_use]
    pub fn speeds(mut self, speeds: &[Speed]) -> Self {
        self.speeds = Some(speeds.into());
        self
    }

    /// Determines the rating groups to filter by.
    /// Each group ranges from its value to the next higher group.
    #[must_use]
    pub fn ratings(mut self, ratings: &[OpeningRatings]) -> Self {
        self.ratings = Some(
            ratings
                .iter()
                .map(|r| r.to_owned() as u16)
                .collect::<Vec<u16>>(),
        );
        self
    }

    /// Determines the number of recent games to display.
    #[must_use]
    pub fn recent_games(mut self, recent_games: u8) -> Self {
        self.recent_games = Some(recent_games);
        self
    }

    /// Determines whether history will be retrieved or not.
    #[must_use]
    pub fn history(mut self, history: bool) -> Self {
        self.history = Some(history);
        self
    }

    /// Determines the FEN of the root position.
    #[must_use]
    pub fn fen(mut self, fen: &str) -> Self {
        self.inner = self.inner.fen(fen);
        self
    }

    /// Determines the sequence of legal moves in UCI notation.
    /// Play additional moves starting from the FEN position.
    /// Required to find an opening game, if the FEN is not an exact match for a named position.
    #[must_use]
    pub fn play(mut self, play: &[&str]) -> Self {
        self.inner = self.inner.play(play);
        self
    }

    /// Include only games from this month or later.
    #[must_use]
    pub fn since(mut self, since: &str) -> Self {
        self.inner = self.inner.since_str(since);
        self
    }

    /// Include only games from this month or earlier.
    #[must_use]
    pub fn until(mut self, until: &str) -> Self {
        self.inner = self.inner.until_str(until);
        self
    }

    /// Determines the number of most common moves to display.
    #[must_use]
    pub fn moves(mut self, moves: u16) -> Self {
        self.inner = self.inner.moves(moves);
        self
    }

    /// Determines the number of top games to display.
    #[must_use]
    pub fn top_games(mut self, top_games: u8) -> Self {
        self.inner = self.inner.top_games(top_games);
        self
    }
}

/// Optional configuration for querying player openings using [`openings().player()`](fn@crate::client::OpeningsApi::player).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename = "camelCase")]
pub struct PlayerOpeningsOptions {
    variant: Option<VariantMode>,
    speeds: Option<Vec<Speed>>,
    modes: Option<GameType>,
    recent_games: Option<u8>,
    #[serde(flatten)]
    inner: MastersOpeningsOptions,
}

impl PlayerOpeningsOptions {
    /// Create a new instance of [`PlayerOpeningsOptions`] with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Determines the game variant.
    /// Defaults to Standard.
    #[must_use]
    pub fn variant(mut self, variant: VariantMode) -> Self {
        self.variant = Some(variant);
        self
    }

    /// Determines the game speeds to filter by.
    #[must_use]
    pub fn speeds(mut self, speeds: &[Speed]) -> Self {
        self.speeds = Some(speeds.into());
        self
    }

    /// Determines whether casual or rated games are to be searched for.
    #[must_use]
    pub fn mode(mut self, mode: GameType) -> Self {
        self.modes = Some(mode);
        self
    }

    /// Determines the number of recent games to display.
    #[must_use]
    pub fn recent_games(mut self, recent_games: u8) -> Self {
        self.recent_games = Some(recent_games);
        self
    }

    /// Determines the FEN of the root position.
    #[must_use]
    pub fn fen(mut self, fen: &str) -> Self {
        self.inner = self.inner.fen(fen);
        self
    }

    /// Determines the sequence of legal moves in UCI notation.
    /// Play additional moves starting from the FEN position.
    /// Required to find an opening game, if the FEN is not an exact match for a named position.
    #[must_use]
    pub fn play(mut self, play: &[&str]) -> Self {
        self.inner = self.inner.play(play);
        self
    }

    /// Include only games from this month or later.
    #[must_use]
    pub fn since(mut self, since: &str) -> Self {
        self.inner = self.inner.since_str(since);
        self
    }

    /// Include only games from this month or earlier.
    #[must_use]
    pub fn until(mut self, until: &str) -> Self {
        self.inner = self.inner.until_str(until);
        self
    }

    /// Determines the number of most common moves to display.
    #[must_use]
    pub fn moves(mut self, moves: u16) -> Self {
        self.inner = self.inner.moves(moves);
        self
    }
}
