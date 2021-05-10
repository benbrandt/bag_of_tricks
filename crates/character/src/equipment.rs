pub(crate) mod adventuring_gear;
pub(crate) mod armor;
pub(crate) mod currency;
pub(crate) mod tools;
pub(crate) mod vehicles;
pub(crate) mod weapons;

use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::{Display, IntoEnumIterator};

use tools::{ArtisansTools, MusicalInstrument};
use trinkets::{TrinketOption, Trinkets};
use vehicles::Vehicle;

use self::{
    adventuring_gear::{Gear, HolySymbol},
    armor::Armor,
    currency::Coin,
    tools::{GamingSet, Tool},
    vehicles::VehicleProficiency,
    weapons::WeaponType,
};

use super::{
    proficiencies::{Proficiency, WeaponProficiency},
    Character,
};

#[derive(Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum Equipment {
    Armor(Armor),
    Gear(Gear),
    Tool(Tool),
    Vehicle(Vehicle),
    Weapon(WeaponType),
    Other(String),
}

impl Equipment {
    fn proficient(&self, character: &Character) -> bool {
        match self {
            Self::Armor(armor) => character
                .proficiencies
                .iter()
                .any(|p| matches!(p, Proficiency::Armor(a) if a == &armor.armor_type())),
            Self::Tool(tool) => character
                .proficiencies
                .iter()
                .any(|p| matches!(p, Proficiency::Tool(t) if t == tool)),
            Self::Vehicle(vehicle) => character.proficiencies.iter().any(|p| {
                matches!(p, Proficiency::Vehicle(v) if v == match vehicle {
                    Vehicle::Land(_) | Vehicle::Mount(_) => &VehicleProficiency::Land,
                    Vehicle::Water(_) => &VehicleProficiency::Water,
                })
            }),
            Self::Weapon(weapon) => character.proficiencies.iter().any(|p| {
                matches!(p, Proficiency::Weapon(w) if match w {
                    WeaponProficiency::Category(category) => category == &weapon.category(),
                    WeaponProficiency::Specific(weapon_type) => weapon_type == weapon,
                })
            }),
            Self::Gear(_) | Self::Other(_) => true,
        }
    }
}

/// A way to encapsulate a equipment that needs to be chosen for a character.
#[derive(Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum EquipmentOption {
    /// Choose from a given list of equipment options.
    From(Vec<Equipment>),
    /// Choose a random artisan's tools.
    ArtisansTools,
    /// Choose a random gaming set.
    GamingSet,
    /// Choose a random holy symbol.
    HolySymbol,
    /// Choose a random musical instrument.
    MusicalInstrument,
    /// Choose a random trinket.
    Trinket(Option<&'static str>, Option<TrinketOption>, bool),
}

impl EquipmentOption {
    /// Randomly choose a given proficiency option, avoiding already existing proficiencies.
    pub(crate) fn gen(&self, rng: &mut impl Rng, character: &Character) -> Equipment {
        match self {
            Self::From(list) => {
                let list = list
                    .clone()
                    .into_iter()
                    .filter(|e| !character.equipment().contains(e));
                // Choose proficient equipment if available
                let mut proficient = list.clone().filter(|e| e.proficient(character)).peekable();
                if proficient.peek().is_some() {
                    proficient.choose(rng).unwrap()
                } else {
                    list.choose(rng).unwrap()
                }
            }
            Self::ArtisansTools => Self::From(
                ArtisansTools::iter()
                    .map(|t| Equipment::Tool(Tool::ArtisansTools(t)))
                    .collect(),
            )
            .gen(rng, character),
            Self::GamingSet => Self::From(
                GamingSet::iter()
                    .map(|m| Equipment::Tool(Tool::GamingSet(m)))
                    .collect(),
            )
            .gen(rng, character),
            Self::HolySymbol => Self::From(
                HolySymbol::iter()
                    .map(|h| Equipment::Gear(Gear::HolySymbol(h)))
                    .collect(),
            )
            .gen(rng, character),
            Self::MusicalInstrument => Self::From(
                MusicalInstrument::iter()
                    .map(|m| Equipment::Tool(Tool::MusicalInstrument(m)))
                    .collect(),
            )
            .gen(rng, character),
            Self::Trinket(label, addl_option, use_all) => {
                let mut options = use_all
                    .then(|| character.trinket_options())
                    .unwrap_or_default();
                if let Some(option) = addl_option {
                    options.push(option.clone());
                }
                Self::From(
                    options
                        .choose(rng)
                        .unwrap()
                        .trinkets()
                        .iter()
                        .map(|t| {
                            Equipment::Other(label.map_or(t.clone(), |l| format!("{} ({})", t, l)))
                        })
                        .collect(),
                )
                .gen(rng, character)
            }
        }
    }
}

/// Trait to describe starting equipment given by a background or class and any additional choices that can be made
pub(crate) trait StartingEquipment {
    /// Starting coins
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 0)
    }

    /// Equiment given by an entity/object
    fn equipment(&self) -> Vec<Equipment> {
        vec![]
    }

    /// Equipment options given by an entity/object that need to be chosen.
    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![]
    }
}