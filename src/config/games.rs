use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{game::FinalColor, user::PerfType};

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct GameOptions {
    moves: Option<bool>,
    tags: Option<bool>,
    clocks: Option<bool>,
    evals: Option<bool>,
    accuracy: Option<bool>,
    opening: Option<bool>,
    division: Option<bool>,
    literate: Option<bool>,
}

impl GameOptions {
    /// Create a new instance of [`GameOptions`] with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Include the PGN moves.
    #[must_use]
    pub fn moves(mut self, moves: bool) -> Self {
        self.moves = Some(moves);
        self
    }

    /// Include the PGN tags.
    #[must_use]
    pub fn tags(mut self, tags: bool) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Include the clock status when available.
    #[must_use]
    pub fn clocks(mut self, clocks: bool) -> Self {
        self.clocks = Some(clocks);
        self
    }

    /// Include analysis evaluations and comments when available.
    #[must_use]
    pub fn evals(mut self, evals: bool) -> Self {
        self.evals = Some(evals);
        self
    }

    /// Include accuracy percent of each player when available.
    #[must_use]
    pub fn accuracy(mut self, accuracy: bool) -> Self {
        self.accuracy = Some(accuracy);
        self
    }

    /// Include the opening name.
    #[must_use]
    pub fn opening(mut self, opening: bool) -> Self {
        self.opening = Some(opening);
        self
    }

    /// Plies which mark the beginning of the middlegame and the endgame.
    #[must_use]
    pub fn division(mut self, division: bool) -> Self {
        self.division = Some(division);
        self
    }

    /// Insert textual annotations in the PGN about the opening, analysis variations, mistakes, and game termination.
    #[must_use]
    pub fn literate(mut self, literate: bool) -> Self {
        self.literate = Some(literate);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GameSortOrder {
    DateAsc,
    DateDesc,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ExtendedGameOptions {
    since: Option<u64>,
    until: Option<u64>,
    max: Option<u16>,
    vs: Option<String>,
    rated: Option<bool>,
    perf_type: Option<String>,
    color: Option<FinalColor>,
    analysed: Option<bool>,
    moves: Option<bool>,
    tags: Option<bool>,
    clocks: Option<bool>,
    evals: Option<bool>,
    accuracy: Option<bool>,
    opening: Option<bool>,
    division: Option<bool>,
    ongoing: Option<bool>,
    finished: Option<bool>,
    literate: Option<bool>,
    last_fen: Option<bool>,
    with_bookmarked: Option<bool>,
    sort: Option<GameSortOrder>,
}

impl ExtendedGameOptions {
    /// Create a new instance of [`GameOptions`] with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Include games played since this timestamp.
    #[must_use]
    pub fn since(mut self, since: u64) -> Self {
        self.since = Some(since);
        self
    }

    /// Include games played until this timestamp.
    #[must_use]
    pub fn until(mut self, until: u64) -> Self {
        self.until = Some(until);
        self
    }

    /// How many games to download.
    #[must_use]
    pub fn max(mut self, max: u16) -> Self {
        self.max = Some(max);
        self
    }

    /// Include only games played against this opponent.
    #[must_use]
    pub fn vs(mut self, vs: &str) -> Self {
        self.vs = Some(vs.to_string());
        self
    }

    /// Include only rated games.
    #[must_use]
    pub fn rated(mut self, rated: bool) -> Self {
        self.rated = Some(rated);
        self
    }

    /// Include only games in these speeds or variants.
    #[must_use]
    pub fn perf_type(mut self, perf_types: Vec<PerfType>) -> Self {
        let encoded = comma_serde_urlencoded::to_string(perf_types).unwrap_or_default();
        self.perf_type = Some(encoded);
        self
    }

    /// Include only games played as this color.
    #[must_use]
    pub fn color(mut self, color: FinalColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Include only games with computer analysis available.
    #[must_use]
    pub fn analysed(mut self, analysed: bool) -> Self {
        self.analysed = Some(analysed);
        self
    }

    /// Include the PGN moves.
    #[must_use]
    pub fn moves(mut self, moves: bool) -> Self {
        self.moves = Some(moves);
        self
    }

    /// Include the PGN tags.
    #[must_use]
    pub fn tags(mut self, tags: bool) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Include the clock status when available.
    #[must_use]
    pub fn clocks(mut self, clocks: bool) -> Self {
        self.clocks = Some(clocks);
        self
    }

    /// Include analysis evaluations and comments when available.
    #[must_use]
    pub fn evals(mut self, evals: bool) -> Self {
        self.evals = Some(evals);
        self
    }

    /// Include accuracy percent of each player when available.
    #[must_use]
    pub fn accuracy(mut self, accuracy: bool) -> Self {
        self.accuracy = Some(accuracy);
        self
    }

    /// Include the opening name.
    #[must_use]
    pub fn opening(mut self, opening: bool) -> Self {
        self.opening = Some(opening);
        self
    }

    /// Plies which mark the beginning of the middlegame and the endgame.
    #[must_use]
    pub fn division(mut self, division: bool) -> Self {
        self.division = Some(division);
        self
    }

    /// Include ongoing games.
    /// Ongoing games are delayed by a few seconds ranging from 3 to 60 depending on the time control to prevent cheat bots from using this setting.
    #[must_use]
    pub fn ongoing(mut self, ongoing: bool) -> Self {
        self.ongoing = Some(ongoing);
        self
    }

    /// Include finished games.
    #[must_use]
    pub fn finished(mut self, finished: bool) -> Self {
        self.finished = Some(finished);
        self
    }

    /// Insert textual annotations in the PGN about the opening, analysis variations, mistakes, and game termination.
    #[must_use]
    pub fn literate(mut self, literate: bool) -> Self {
        self.literate = Some(literate);
        self
    }

    /// Include the FEN notation of the last position of the game.
    #[must_use]
    pub fn last_fen(mut self, last_fen: bool) -> Self {
        self.last_fen = Some(last_fen);
        self
    }

    /// Include a `bookmarked` field in the response when the logged in user has bookmarked the game.
    #[must_use]
    pub fn with_bookmarked(mut self, bookmarked: bool) -> Self {
        self.with_bookmarked = Some(bookmarked);
        self
    }

    /// Sort order of the games.
    #[must_use]
    pub fn sort(mut self, sort: GameSortOrder) -> Self {
        self.sort = Some(sort);
        self
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct BookmarkedGameOptions {
    since: Option<u64>,
    until: Option<u64>,
    max: Option<u16>,
    moves: Option<bool>,
    tags: Option<bool>,
    clocks: Option<bool>,
    evals: Option<bool>,
    accuracy: Option<bool>,
    opening: Option<bool>,
    division: Option<bool>,
    literate: Option<bool>,
    last_fen: Option<bool>,
    sort: Option<GameSortOrder>,
}

impl BookmarkedGameOptions {
    /// Create a new instance of [`GameOptions`] with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Include games played since this timestamp.
    #[must_use]
    pub fn since(mut self, since: u64) -> Self {
        self.since = Some(since);
        self
    }

    /// Include games played until this timestamp.
    #[must_use]
    pub fn until(mut self, until: u64) -> Self {
        self.until = Some(until);
        self
    }

    /// How many games to download.
    #[must_use]
    pub fn max(mut self, max: u16) -> Self {
        self.max = Some(max);
        self
    }

    /// Include the PGN moves.
    #[must_use]
    pub fn moves(mut self, moves: bool) -> Self {
        self.moves = Some(moves);
        self
    }

    /// Include the PGN tags.
    #[must_use]
    pub fn tags(mut self, tags: bool) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Include the clock status when available.
    #[must_use]
    pub fn clocks(mut self, clocks: bool) -> Self {
        self.clocks = Some(clocks);
        self
    }

    /// Include analysis evaluations and comments when available.
    #[must_use]
    pub fn evals(mut self, evals: bool) -> Self {
        self.evals = Some(evals);
        self
    }

    /// Include accuracy percent of each player when available.
    #[must_use]
    pub fn accuracy(mut self, accuracy: bool) -> Self {
        self.accuracy = Some(accuracy);
        self
    }

    /// Include the opening name.
    #[must_use]
    pub fn opening(mut self, opening: bool) -> Self {
        self.opening = Some(opening);
        self
    }

    /// Plies which mark the beginning of the middlegame and the endgame.
    #[must_use]
    pub fn division(mut self, division: bool) -> Self {
        self.division = Some(division);
        self
    }

    /// Insert textual annotations in the PGN about the opening, analysis variations, mistakes, and game termination.
    #[must_use]
    pub fn literate(mut self, literate: bool) -> Self {
        self.literate = Some(literate);
        self
    }

    /// Include the FEN notation of the last position of the game.
    #[must_use]
    pub fn last_fen(mut self, last_fen: bool) -> Self {
        self.last_fen = Some(last_fen);
        self
    }

    /// Sort order of the games.
    #[must_use]
    pub fn sort(mut self, sort: GameSortOrder) -> Self {
        self.sort = Some(sort);
        self
    }
}
