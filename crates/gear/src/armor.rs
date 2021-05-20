use std::fmt;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

/// Classes of armor different items fall under
#[derive(
    Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
}
#[derive(Clone, Copy, Debug, Deserialize, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Armor {
    Padded,
    Leather,
    StuddedLeather,
    Hide,
    ChainShirt,
    ScaleMail,
    Breastplate,
    HalfPlate,
    RingMail,
    ChainMail,
    Splint,
    Plate,
    Shield,
}

impl fmt::Display for Armor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Padded => write!(f, "Padded armor"),
            Self::Leather => write!(f, "Leather armor"),
            Self::StuddedLeather => write!(f, "Studded leather armor"),
            Self::Hide => write!(f, "Hide armor"),
            Self::ChainShirt => write!(f, "Chain shirt"),
            Self::ScaleMail => write!(f, "Scale mail"),
            Self::Breastplate => write!(f, "Breastplate"),
            Self::HalfPlate => write!(f, "Half plate armor"),
            Self::RingMail => write!(f, "Ring mail"),
            Self::ChainMail => write!(f, "Chain mail"),
            Self::Splint => write!(f, "Splint armor"),
            Self::Plate => write!(f, "Plate armor"),
            Self::Shield => write!(f, "Shield"),
        }
    }
}

impl Armor {
    pub fn armor_type(self) -> ArmorType {
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
