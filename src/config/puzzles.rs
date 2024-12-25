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
