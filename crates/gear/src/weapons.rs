use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(
    Clone, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum WeaponCategory {
    Simple,
    Martial,
}

#[derive(Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WeaponClassification {
    Melee,
    Ranged,
}

#[allow(dead_code)]
#[derive(
    Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum Weapon {
    Battleaxe,
    Blowgun,
    Club,
    #[strum(serialize = "Hand crossbow")]
    CrossbowHand,
    #[strum(serialize = "Heavy crossbow")]
    CrossbowHeavy,
    #[strum(serialize = "Light crossbow")]
    CrossbowLight,
    Dagger,
    Dart,
    Flail,
    Glaive,
    Greataxe,
    Greatsword,
    Greatclub,
    Halberd,
    Handaxe,
    Javelin,
    Lance,
    Longbow,
    Longsword,
    #[strum(serialize = "Light hammer")]
    LightHammer,
    Mace,
    Maul,
    Morningstar,
    Net,
    Pike,
    Quarterstaff,
    Rapier,
    Scimitar,
    Shortbow,
    Shortsword,
    Sickle,
    Sling,
    Spear,
    Trident,
    #[strum(serialize = "War pick")]
    WarPick,
    Warhammer,
    Whip,
}

impl Weapon {
    pub fn category(self) -> WeaponCategory {
        match self {
            Self::Club
            | Self::Dagger
            | Self::Greatclub
            | Self::Handaxe
            | Self::Javelin
            | Self::LightHammer
            | Self::Mace
            | Self::Quarterstaff
            | Self::Sickle
            | Self::Spear
            | Self::CrossbowLight
            | Self::Dart
            | Self::Shortbow
            | Self::Sling => WeaponCategory::Simple,
            Self::Battleaxe
            | Self::Flail
            | Self::Glaive
            | Self::Greataxe
            | Self::Greatsword
            | Self::Halberd
            | Self::Lance
            | Self::Longsword
            | Self::Maul
            | Self::Morningstar
            | Self::Pike
            | Self::Rapier
            | Self::Scimitar
            | Self::Shortsword
            | Self::Trident
            | Self::WarPick
            | Self::Warhammer
            | Self::Whip
            | Self::Blowgun
            | Self::CrossbowHand
            | Self::CrossbowHeavy
            | Self::Longbow
            | Self::Net => WeaponCategory::Martial,
        }
    }

    pub fn classification(self) -> WeaponClassification {
        match self {
            Self::Club
            | Self::Dagger
            | Self::Greatclub
            | Self::Handaxe
            | Self::Javelin
            | Self::LightHammer
            | Self::Mace
            | Self::Quarterstaff
            | Self::Sickle
            | Self::Spear
            | Self::Battleaxe
            | Self::Flail
            | Self::Glaive
            | Self::Greataxe
            | Self::Greatsword
            | Self::Halberd
            | Self::Lance
            | Self::Longsword
            | Self::Maul
            | Self::Morningstar
            | Self::Pike
            | Self::Rapier
            | Self::Scimitar
            | Self::Shortsword
            | Self::Trident
            | Self::WarPick
            | Self::Warhammer
            | Self::Whip => WeaponClassification::Melee,
            Self::CrossbowLight
            | Self::Dart
            | Self::Shortbow
            | Self::Sling
            | Self::Blowgun
            | Self::CrossbowHand
            | Self::CrossbowHeavy
            | Self::Longbow
            | Self::Net => WeaponClassification::Ranged,
        }
    }
}

/// Types of ammunition available
#[derive(Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Ammunition {
    Arrows,
    #[strum(serialize = "Blowgun needles")]
    BlowgunNeedles,
    #[strum(serialize = "Crossbow bolts")]
    CrossbowBolts,
    #[strum(serialize = "Sling bullets")]
    SlingBullets,
}

impl Ammunition {
    pub fn weapons(&self) -> Vec<Weapon> {
        match self {
            Ammunition::Arrows => vec![Weapon::Shortbow, Weapon::Longbow],
            Ammunition::BlowgunNeedles => vec![Weapon::Blowgun],
            Ammunition::CrossbowBolts => vec![
                Weapon::CrossbowLight,
                Weapon::CrossbowHand,
                Weapon::CrossbowHeavy,
            ],
            Ammunition::SlingBullets => vec![Weapon::Sling],
        }
    }
}
