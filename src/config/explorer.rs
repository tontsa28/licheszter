use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    explorer::OpeningRatings,
    game::{GameType, Speed, VariantMode},
};

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct MastersOpeningOptions {
    fen: Option<String>,
    play: Option<Vec<String>>,
    since: Option<u16>,
    until: Option<u16>,
    moves: Option<u16>,
    #[serde(rename = "topGames")]
    top_games: Option<u8>,
}

impl MastersOpeningOptions {
    /// Create a new instance of [`MastersOpeningOptions`] with default configuration.
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
    pub fn play(mut self, play: Vec<&str>) -> Self {
        self.play = Some(play.iter().map(|s| s.to_string()).collect::<Vec<String>>());
        self
    }

    /// Include only games from this year or later.
    #[must_use]
    pub fn since(mut self, since: u16) -> Self {
        self.since = Some(since);
        self
    }

    /// Include only games from this year or earlier.
    #[must_use]
    pub fn until(mut self, until: u16) -> Self {
        self.until = Some(until);
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

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename = "camelCase")]
pub struct LichessOpeningOptions {
    variant: Option<VariantMode>,
    fen: Option<String>,
    play: Option<Vec<String>>,
    speeds: Option<Vec<Speed>>,
    ratings: Option<Vec<u16>>,
    since: Option<String>,
    until: Option<String>,
    moves: Option<u16>,
    top_games: Option<u8>,
    recent_games: Option<u8>,
    history: Option<bool>,
}

impl LichessOpeningOptions {
    /// Create a new instance of [`LichessOpeningOptions`] with default configuration.
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
    pub fn play(mut self, play: Vec<&str>) -> Self {
        self.play = Some(play.iter().map(|s| s.to_string()).collect::<Vec<String>>());
        self
    }

    /// Determines the game speeds to filter by.
    #[must_use]
    pub fn speeds(mut self, speeds: Vec<Speed>) -> Self {
        self.speeds = Some(speeds);
        self
    }

    /// Determines the rating groups to filter by.
    /// Each group ranges from its value to the next higher group.
    #[must_use]
    pub fn ratings(mut self, ratings: Vec<OpeningRatings>) -> Self {
        self.ratings = Some(
            ratings
                .iter()
                .map(|r| r.to_owned() as u16)
                .collect::<Vec<u16>>(),
        );
        self
    }

    /// Include only games from this year or later.
    #[must_use]
    pub fn since(mut self, since: &str) -> Self {
        self.since = Some(since.to_string());
        self
    }

    /// Include only games from this year or earlier.
    #[must_use]
    pub fn until(mut self, until: &str) -> Self {
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
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
#[serde(rename = "camelCase")]
pub struct PlayerOpeningOptions {
    variant: Option<VariantMode>,
    fen: Option<String>,
    play: Option<Vec<String>>,
    speeds: Option<Vec<Speed>>,
    modes: Option<GameType>,
    since: Option<String>,
    until: Option<String>,
    moves: Option<u16>,
    recent_games: Option<u8>,
}

impl PlayerOpeningOptions {
    /// Create a new instance of [`LichessOpeningOptions`] with default configuration.
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
    pub fn play(mut self, play: Vec<&str>) -> Self {
        self.play = Some(play.iter().map(|s| s.to_string()).collect::<Vec<String>>());
        self
    }

    /// Determines the game speeds to filter by.
    #[must_use]
    pub fn speeds(mut self, speeds: Vec<Speed>) -> Self {
        self.speeds = Some(speeds);
        self
    }

    /// Determines whether casual or rated games are to be searched for.
    #[must_use]
    pub fn mode(mut self, mode: GameType) -> Self {
        self.modes = Some(mode);
        self
    }

    /// Include only games from this year or later.
    #[must_use]
    pub fn since(mut self, since: &str) -> Self {
        self.since = Some(since.to_string());
        self
    }

    /// Include only games from this year or earlier.
    #[must_use]
    pub fn until(mut self, until: &str) -> Self {
        self.until = Some(until.to_string());
        self
    }

    /// Determines the number of most common moves to display.
    #[must_use]
    pub fn moves(mut self, moves: u16) -> Self {
        self.moves = Some(moves);
        self
    }

    /// Determines the number of recent games to display.
    #[must_use]
    pub fn recent_games(mut self, recent_games: u8) -> Self {
        self.recent_games = Some(recent_games);
        self
    }
}
