use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PuzzleDifficulty {
    Easiest,
    Easier,
    Normal,
    Harder,
    Hardest,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PuzzleSolution {
    id: String,
    rated: bool,
    win: bool,
}

impl PuzzleSolution {
    /// Create a new instance of [`PuzzleSolution`] with given parameters.
    pub fn new(id: &str, rated: bool, win: bool) -> Self {
        Self {
            id: id.to_string(),
            rated,
            win,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct PuzzleSolutions {
    pub(crate) solutions: Vec<PuzzleSolution>,
}
