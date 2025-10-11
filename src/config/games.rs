use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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
