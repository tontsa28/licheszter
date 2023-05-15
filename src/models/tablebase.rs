use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Endgame {
    pub dtz: Option<i16>,
    pub precise_dtz: Option<i16>,
    pub dtm: Option<i16>,
    pub checkmate: bool,
    pub stalemate: bool,
    pub variant_win: bool,
    pub variant_loss: bool,
    pub insufficient_material: bool,
    pub category: String,
    pub moves: Vec<EndgameMove>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EndgameMove {
    pub uci: String,
    pub san: String,
    pub dtz: Option<i16>,
    pub precise_dtz: Option<i16>,
    pub dtm: Option<i16>,
    pub zeroing: bool,
    pub checkmate: bool,
    pub stalemate: bool,
    pub variant_win: bool,
    pub variant_loss: bool,
    pub insufficient_material: bool,
    pub category: String,
}
