use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, IntoEnumIterator};
use trinkets::TrinketOption;

use gear::{
    adventuring_gear::{Gear, HolySymbol, OtherGear},
    armor::Armor,
    currency::Coin,
    tools::{ArtisansTools, GamingSet, MusicalInstrument, Tool},
    vehicles::{Vehicle, VehicleProficiency},
    weapons::WeaponType,
};

use super::proficiencies::{Proficiency, WeaponProficiency};

#[derive(Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Equipment {
    Armor(Armor),
    Gear(Gear),
    Tool(Tool),
    Vehicle(Vehicle),
    Weapon(WeaponType),
    Other(String),
}

impl Equipment {
    fn proficient(&self, proficiencies: &[Proficiency]) -> bool {
        match self {
            Self::Armor(armor) => proficiencies
                .iter()
                .any(|p| matches!(p, Proficiency::Armor(a) if a == &armor.armor_type())),
            Self::Tool(tool) => proficiencies
                .iter()
                .any(|p| matches!(p, Proficiency::Tool(t) if t == tool)),
            Self::Vehicle(vehicle) => proficiencies.iter().any(|p| {
                matches!(p, Proficiency::Vehicle(v) if v == match vehicle {
                    Vehicle::Land(_) | Vehicle::Mount(_) => &VehicleProficiency::Land,
                    Vehicle::Water(_) => &VehicleProficiency::Water,
                })
            }),
            Self::Weapon(weapon) => proficiencies.iter().any(|p| {
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
pub enum EquipmentOption {
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
    pub fn gen(
        &self,
        rng: &mut impl Rng,
        equipment: &[Equipment],
        proficiencies: &[Proficiency],
        trinket_options: &[TrinketOption],
    ) -> Equipment {
        match self {
            Self::From(list) => {
                let list = list.clone().into_iter().filter(|e| !equipment.contains(e));
                // Choose proficient equipment if available
                let mut proficient = list
                    .clone()
                    .filter(|e| e.proficient(proficiencies))
                    .peekable();
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
            .gen(rng, equipment, proficiencies, trinket_options),
            Self::GamingSet => Self::From(
                GamingSet::iter()
                    .map(|m| Equipment::Tool(Tool::GamingSet(m)))
                    .collect(),
            )
            .gen(rng, equipment, proficiencies, trinket_options),
            Self::HolySymbol => Self::From(
                HolySymbol::iter()
                    .map(|h| Equipment::Gear(Gear::HolySymbol(h)))
                    .collect(),
            )
            .gen(rng, equipment, proficiencies, trinket_options),
            Self::MusicalInstrument => Self::From(
                MusicalInstrument::iter()
                    .map(|m| Equipment::Tool(Tool::MusicalInstrument(m)))
                    .collect(),
            )
            .gen(rng, equipment, proficiencies, trinket_options),
            Self::Trinket(label, addl_option, use_all) => {
                let mut options = use_all
                    .then(|| trinket_options.to_vec())
                    .unwrap_or_default();
                if let Some(option) = addl_option {
                    options.push(option.clone());
                }
                Self::From(
                    options
                        .iter()
                        .flat_map(TrinketOption::trinkets)
                        .map(|t| {
                            Equipment::Other(label.map_or(t.clone(), |l| format!("{} ({})", t, l)))
                        })
                        .collect(),
                )
                .gen(rng, equipment, proficiencies, trinket_options)
            }
        }
    }
}

/// Trait to describe starting equipment given by a background or class and any additional choices that can be made
pub trait StartingEquipment {
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

pub enum Pack {
    MonsterHunter,
}

impl StartingEquipment for Pack {
    fn equipment(&self) -> Vec<Equipment> {
        match self {
            Pack::MonsterHunter => vec![
                Equipment::Gear(Gear::Other(OtherGear::Chest)),
                Equipment::Gear(Gear::Other(OtherGear::Crowbar)),
                Equipment::Gear(Gear::Other(OtherGear::Hammer)),
                Equipment::Gear(Gear::Other(OtherGear::HolyWater)),
                Equipment::Gear(Gear::Other(OtherGear::Manacles)),
                Equipment::Gear(Gear::Other(OtherGear::MirrorSteel)),
                Equipment::Gear(Gear::Other(OtherGear::Oil)),
                Equipment::Gear(Gear::Other(OtherGear::Tinderbox)),
                Equipment::Gear(Gear::Other(OtherGear::Torch)),
                Equipment::Gear(Gear::Other(OtherGear::Torch)),
                Equipment::Gear(Gear::Other(OtherGear::Torch)),
                Equipment::Other("three wooden stakes".into()),
            ],
        }
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        match self {
            Pack::MonsterHunter => vec![EquipmentOption::HolySymbol],
        }
    }
}
