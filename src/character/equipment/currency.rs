use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum Coin {
    #[strum(serialize = "cp")]
    Copper,
    #[strum(serialize = "sp")]
    Silver,
    #[strum(serialize = "ep")]
    Electrum,
    #[strum(serialize = "gp")]
    Gold,
    #[strum(serialize = "pp")]
    Platinum,
}
