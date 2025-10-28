use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct Endgame {
    pub dtz: Option<i16>,
    pub precise_dtz: Option<i16>,
    pub dtm: Option<i16>,
    pub dtw: Option<i16>,
    pub dtc: Option<i16>,
    pub checkmate: bool,
    pub stalemate: bool,
    pub variant_win: bool,
    pub variant_loss: bool,
    pub insufficient_material: bool,
    pub category: EndgameCategory,
    pub moves: Vec<EndgameMove>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "serde-strict", serde(deny_unknown_fields))]
pub struct EndgameMove {
    pub uci: String,
    pub san: String,
    pub dtz: Option<i16>,
    pub precise_dtz: Option<i16>,
    pub dtm: Option<i16>,
    pub dtw: Option<i16>,
    pub dtc: Option<i16>,
    pub zeroing: bool,
    pub conversion: bool,
    pub checkmate: bool,
    pub stalemate: bool,
    pub variant_win: bool,
    pub variant_loss: bool,
    pub insufficient_material: bool,
    pub category: EndgameCategory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EndgameCategory {
    Win,
    Unknown,
    SyzygyWin,
    MaybeWin,
    CursedWin,
    Draw,
    BlessedLoss,
    MaybeLoss,
    SyzygyLoss,
    Loss,
}
