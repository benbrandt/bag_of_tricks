use dice_roller::{Die, RollCmd};
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

#[derive(Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WeaponProperty {
    Ammunition(Ammunition, u16, u16),
    Finesse,
    Heavy,
    Light,
    Loading,
    Reach,
    Special,
    Thrown(u16, u16),
    #[strum(serialize = "Two-Handed")]
    TwoHanded,
    Versatile(RollCmd),
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

    pub fn properties(&self) -> Vec<WeaponProperty> {
        match self {
            Self::Battleaxe => vec![WeaponProperty::Versatile(RollCmd(1, Die::D10))],
            Self::Blowgun => vec![
                WeaponProperty::Ammunition(Ammunition::BlowgunNeedles, 25, 100),
                WeaponProperty::Loading,
            ],
            Self::Club => vec![WeaponProperty::Light],
            Self::CrossbowHand => vec![
                WeaponProperty::Ammunition(Ammunition::CrossbowBolts, 30, 120),
                WeaponProperty::Light,
                WeaponProperty::Loading,
            ],
            Self::CrossbowHeavy => vec![
                WeaponProperty::Ammunition(Ammunition::CrossbowBolts, 100, 400),
                WeaponProperty::Heavy,
                WeaponProperty::Loading,
                WeaponProperty::TwoHanded,
            ],
            Self::CrossbowLight => vec![
                WeaponProperty::Ammunition(Ammunition::CrossbowBolts, 80, 320),
                WeaponProperty::Loading,
                WeaponProperty::TwoHanded,
            ],
            Self::Dagger => vec![
                WeaponProperty::Finesse,
                WeaponProperty::Light,
                WeaponProperty::Thrown(20, 60),
            ],
            Self::Dart => vec![WeaponProperty::Finesse, WeaponProperty::Thrown(20, 60)],
            Self::Flail => vec![],
            Self::Glaive => vec![
                WeaponProperty::Heavy,
                WeaponProperty::Reach,
                WeaponProperty::TwoHanded,
            ],
            Self::Greataxe => vec![WeaponProperty::Heavy, WeaponProperty::TwoHanded],
            Self::Greatsword => vec![WeaponProperty::Heavy, WeaponProperty::TwoHanded],
            Self::Greatclub => vec![WeaponProperty::TwoHanded],
            Self::Halberd => vec![
                WeaponProperty::Heavy,
                WeaponProperty::Reach,
                WeaponProperty::TwoHanded,
            ],
            Self::Handaxe => vec![WeaponProperty::Light, WeaponProperty::Thrown(20, 60)],
            Self::Javelin => vec![WeaponProperty::Thrown(30, 120)],
            Self::Lance => vec![WeaponProperty::Reach, WeaponProperty::Special],
            Self::Longbow => vec![
                WeaponProperty::Ammunition(Ammunition::Arrows, 150, 600),
                WeaponProperty::Heavy,
                WeaponProperty::TwoHanded,
            ],
            Self::Longsword => vec![WeaponProperty::Versatile(RollCmd(1, Die::D10))],
            Self::LightHammer => vec![WeaponProperty::Light, WeaponProperty::Thrown(20, 60)],
            Self::Mace => vec![],
            Self::Maul => vec![WeaponProperty::Heavy, WeaponProperty::TwoHanded],
            Self::Morningstar => vec![],
            Self::Net => vec![WeaponProperty::Special, WeaponProperty::Thrown(5, 15)],
            Self::Pike => vec![
                WeaponProperty::Heavy,
                WeaponProperty::Reach,
                WeaponProperty::TwoHanded,
            ],
            Self::Quarterstaff => vec![WeaponProperty::Versatile(RollCmd(1, Die::D8))],
            Self::Rapier => vec![WeaponProperty::Finesse],
            Self::Scimitar => vec![WeaponProperty::Finesse, WeaponProperty::Light],
            Self::Shortbow => vec![
                WeaponProperty::Ammunition(Ammunition::Arrows, 80, 320),
                WeaponProperty::TwoHanded,
            ],
            Self::Shortsword => vec![WeaponProperty::Finesse, WeaponProperty::Light],
            Self::Sickle => vec![WeaponProperty::Light],
            Self::Sling => vec![WeaponProperty::Ammunition(
                Ammunition::SlingBullets,
                30,
                120,
            )],
            Self::Spear => vec![
                WeaponProperty::Thrown(20, 60),
                WeaponProperty::Versatile(RollCmd(1, Die::D8)),
            ],
            Self::Trident => vec![
                WeaponProperty::Thrown(20, 60),
                WeaponProperty::Versatile(RollCmd(1, Die::D8)),
            ],
            Self::WarPick => vec![],
            Self::Warhammer => vec![WeaponProperty::Versatile(RollCmd(1, Die::D10))],
            Self::Whip => vec![WeaponProperty::Finesse, WeaponProperty::Reach],
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
