use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Debug, Deserialize, Display, Serialize)]
pub(crate) enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
}
