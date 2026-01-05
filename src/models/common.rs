use serde::{Deserialize, Serialize};
use time::serde::format_description;

format_description!(pub(crate) date_dot, Date, "[year].[month].[day]");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OkResponse {
    pub ok: bool,
}
