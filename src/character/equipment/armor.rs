use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Clone, Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
pub(crate) enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
}