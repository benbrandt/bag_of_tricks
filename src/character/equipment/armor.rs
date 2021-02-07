use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

/// Classes of armor different items fall under
#[derive(
    Clone, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub(crate) enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
}
