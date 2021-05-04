use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

/// Classes of armor different items fall under
#[derive(
    Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub(crate) enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
}
#[derive(
    Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub(crate) enum Armor {
    Padded,
    Leather,
    #[strum(serialize = "Studded leather")]
    StuddedLeather,
    Hide,
    #[strum(serialize = "Chain shirt")]
    ChainShirt,
    #[strum(serialize = "Scale mail")]
    ScaleMail,
    Breastplate,
    #[strum(serialize = "Half plate")]
    HalfPlate,
    #[strum(serialize = "Ring mail")]
    RingMail,
    #[strum(serialize = "Chain mail")]
    ChainMail,
    Splint,
    Plate,
    Shield,
}

impl Armor {
    pub(crate) fn armor_type(self) -> ArmorType {
        match self {
            Self::Padded | Self::Leather | Self::StuddedLeather => ArmorType::Light,
            Self::Hide
            | Self::ChainShirt
            | Self::ScaleMail
            | Self::Breastplate
            | Self::HalfPlate => ArmorType::Medium,
            Self::RingMail | Self::ChainMail | Self::Splint | Self::Plate => ArmorType::Heavy,
            Self::Shield => ArmorType::Shield,
        }
    }
}
