use std::fmt;

use characteristics::Size;
use itertools::Itertools;
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use trinkets::TrinketOption;

use gear::{
    adventuring_gear::{Gear, HolySymbol, OtherGear},
    armor::Armor,
    currency::Coin,
    tools::{ArtisansTools, GamingSet, MusicalInstrument, Tool},
    vehicles::{Vehicle, VehicleProficiency},
    weapons::{Ammunition, Weapon, WeaponCategory, WeaponClassification, WeaponProperty},
};

use crate::ability::AbilityScores;

use super::proficiencies::{Proficiency, WeaponProficiency};

#[derive(Clone, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Item {
    Ammunition(Ammunition),
    Armor(Armor),
    Gear(Gear),
    Tool(Tool),
    Vehicle(Vehicle),
    Weapon(Weapon),
    Other(String),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ammunition(i) => write!(f, "{}", i),
            Self::Armor(i) => write!(f, "{}", i),
            Self::Gear(i) => write!(f, "{}", i),
            Self::Tool(i) => write!(f, "{}", i),
            Self::Vehicle(i) => write!(f, "{}", i),
            Self::Weapon(i) => write!(f, "{}", i),
            Self::Other(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Clone, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Equipment {
    item: Item,
    amount: usize,
}

impl Equipment {
    pub fn new(item: Item, amount: usize) -> Self {
        Self { item, amount }
    }

    fn proficient(&self, proficiencies: &[Proficiency]) -> bool {
        match &self.item {
            Item::Ammunition(ammunition) => ammunition
                .weapons()
                .iter()
                .any(|w| Self::weapon_proficiency(w, proficiencies)),
            Item::Armor(armor) => proficiencies
                .iter()
                .any(|p| matches!(p, Proficiency::Armor(a) if a == &armor.armor_type())),
            Item::Tool(tool) => proficiencies
                .iter()
                .any(|p| matches!(p, Proficiency::Tool(t) if t == tool)),
            Item::Vehicle(vehicle) => proficiencies.iter().any(|p| {
                matches!(p, Proficiency::Vehicle(v) if v == match vehicle {
                    Vehicle::Land(_) | Vehicle::Mount(_) => &VehicleProficiency::Land,
                    Vehicle::Water(_) => &VehicleProficiency::Water,
                })
            }),
            Item::Weapon(weapon) => Self::weapon_proficiency(&weapon, proficiencies),
            Item::Gear(_) | Item::Other(_) => true,
        }
    }

    fn weapon_proficiency(weapon: &Weapon, proficiencies: &[Proficiency]) -> bool {
        proficiencies.iter().any(|p| {
            matches!(p, Proficiency::Weapon(w) if match w {
                WeaponProficiency::Category(category) => category == &weapon.category(),
                WeaponProficiency::Specific(weapon_type) => weapon_type == weapon,
            })
        })
    }
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.item)?;
        if self.amount > 1 {
            write!(f, " ({})", self.amount)?;
        }
        write!(f, "")
    }
}

/// A way to encapsulate a equipment that needs to be chosen for a character.
#[derive(Clone, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EquipmentOption {
    /// Choose from a given list of equipment options.
    From(Vec<Equipment>, usize),
    /// Choose from multiple options
    FromOptions(Vec<EquipmentOption>, usize),
    /// Choose a random artisan's tools.
    ArtisansTools,
    /// Choose a random gaming set.
    GamingSet,
    /// Choose a random holy symbol.
    HolySymbol,
    /// Choose a random musical instrument.
    MusicalInstrument,
    /// Choose a random pack
    Pack(Option<Vec<Pack>>),
    /// Choose a random trinket.
    Trinket(Option<&'static str>, Option<TrinketOption>, bool),
    /// Choose a random weapon
    Weapon(Option<WeaponCategory>, Option<WeaponClassification>, usize),
}

impl EquipmentOption {
    /// Randomly choose a given proficiency option, avoiding already existing proficiencies.
    pub fn gen(
        &self,
        rng: &mut impl Rng,
        ability_scores: &AbilityScores,
        equipment: &[Equipment],
        proficiencies: &[Proficiency],
        size: &Option<&Size>,
        trinket_options: &[TrinketOption],
    ) -> Vec<Equipment> {
        match self {
            Self::From(list, amount) => {
                let list = list.clone().into_iter().filter(|e| !equipment.contains(e));
                // Choose proficient equipment if available
                let mut proficient = list
                    .clone()
                    .filter(|e| e.proficient(proficiencies))
                    .peekable();
                if proficient.peek().is_some() {
                    proficient.choose_multiple(rng, *amount)
                } else {
                    list.choose_multiple(rng, *amount)
                }
            }
            Self::FromOptions(choices, amount) => {
                let mut options = choices
                    .choose_multiple(rng, *amount)
                    .flat_map(|c| {
                        c.gen(
                            rng,
                            ability_scores,
                            equipment,
                            proficiencies,
                            size,
                            trinket_options,
                        )
                    })
                    .collect_vec();
                // Add more if we didn't get enough
                let remaining = *amount - options.len();
                if remaining > 0 {
                    options.extend(Self::FromOptions(choices.clone(), remaining).gen(
                        rng,
                        ability_scores,
                        equipment,
                        proficiencies,
                        size,
                        trinket_options,
                    ))
                }
                options
            }
            Self::ArtisansTools => Self::From(
                ArtisansTools::iter()
                    .map(|t| Equipment::new(Item::Tool(Tool::ArtisansTools(t)), 1))
                    .collect(),
                1,
            )
            .gen(
                rng,
                ability_scores,
                equipment,
                proficiencies,
                size,
                trinket_options,
            ),
            Self::GamingSet => Self::From(
                GamingSet::iter()
                    .map(|m| Equipment::new(Item::Tool(Tool::GamingSet(m)), 1))
                    .collect(),
                1,
            )
            .gen(
                rng,
                ability_scores,
                equipment,
                proficiencies,
                size,
                trinket_options,
            ),
            Self::HolySymbol => Self::From(
                HolySymbol::iter()
                    .map(|h| Equipment::new(Item::Gear(Gear::HolySymbol(h)), 1))
                    .collect(),
                1,
            )
            .gen(
                rng,
                ability_scores,
                equipment,
                proficiencies,
                size,
                trinket_options,
            ),
            Self::MusicalInstrument => Self::From(
                MusicalInstrument::iter()
                    .map(|m| Equipment::new(Item::Tool(Tool::MusicalInstrument(m)), 1))
                    .collect(),
                1,
            )
            .gen(
                rng,
                ability_scores,
                equipment,
                proficiencies,
                size,
                trinket_options,
            ),
            Self::Pack(packs) => packs
                .clone()
                .unwrap_or_else(|| Pack::iter().collect_vec())
                .choose(rng)
                .unwrap()
                .equipment(),
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
                            Equipment::new(
                                Item::Other(label.map_or(t.clone(), |l| format!("{} ({})", t, l))),
                                1,
                            )
                        })
                        .collect(),
                    1,
                )
                .gen(
                    rng,
                    ability_scores,
                    equipment,
                    proficiencies,
                    size,
                    trinket_options,
                )
            }
            Self::Weapon(category, classification, amount) => {
                let options = Weapon::iter()
                    .filter(|w| {
                        if let Some(c) = category {
                            if c != &w.category() {
                                return false;
                            }
                        }
                        if let Some(c) = classification {
                            if c != &w.classification() {
                                return false;
                            }
                        }
                        if let Some(s) = size {
                            if matches!(s, Size::Small)
                                && w.properties().contains(&WeaponProperty::Heavy)
                            {
                                return false;
                            }
                        }
                        true
                    })
                    .map(|w| Equipment::new(Item::Weapon(w), 1))
                    .collect_vec();
                Self::From(options, *amount).gen(
                    rng,
                    ability_scores,
                    equipment,
                    proficiencies,
                    size,
                    trinket_options,
                )
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

#[derive(Clone, Copy, Deserialize, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Pack {
    Burglar,
    Diplomat,
    Dungeoneer,
    Entertainer,
    Explorer,
    MonsterHunter,
    Priest,
    Scholar,
}

impl StartingEquipment for Pack {
    fn equipment(&self) -> Vec<Equipment> {
        match self {
            Pack::Burglar => vec![
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Backpack)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::BallBearings)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Bell)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Candle)), 5),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Crowbar)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Hammer)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::LanternHooded)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Oil)), 2),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Piton)), 10),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Rations)), 5),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::RopeHempen)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Tinderbox)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Waterskin)), 1),
                Equipment::new(Item::Other("10 feet of string".to_string()), 1),
            ],
            Pack::Diplomat => vec![
                Equipment::new(Item::Gear(Gear::Other(OtherGear::CaseMapOrScroll)), 2),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Chest)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::ClothesFine)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Ink)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::InkPen)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Lamp)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Oil)), 2),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Paper)), 5),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Perfume)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::SealingWax)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Soap)), 1),
            ],
            Pack::Dungeoneer => vec![
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Backpack)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Crowbar)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Hammer)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Piton)), 10),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Rations)), 10),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::RopeHempen)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Tinderbox)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Torch)), 10),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Waterskin)), 1),
            ],
            Pack::Entertainer => vec![
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Backpack)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Bedroll)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Candle)), 5),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::ClothesCostume)), 2),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Rations)), 5),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Waterskin)), 1),
                Equipment::new(Item::Tool(Tool::DisguiseKit), 1),
            ],
            Pack::Explorer => vec![
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Backpack)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Bedroll)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::MessKit)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Rations)), 10),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::RopeHempen)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Tinderbox)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Torch)), 10),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Waterskin)), 1),
            ],
            Pack::MonsterHunter => vec![
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Chest)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Crowbar)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Hammer)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::HolyWater)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Manacles)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::MirrorSteel)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Oil)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Tinderbox)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Torch)), 3),
                Equipment::new(Item::Other("wooden stake".into()), 3),
            ],
            Pack::Priest => vec![
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Backpack)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Blanket)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Candle)), 10),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Rations)), 2),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Tinderbox)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Waterskin)), 1),
                Equipment::new(Item::Other("an alms box".to_string()), 1),
                Equipment::new(Item::Other("block of incense".to_string()), 2),
                Equipment::new(Item::Other("a censer".to_string()), 1),
                Equipment::new(Item::Other("vestments".to_string()), 1),
            ],
            Pack::Scholar => vec![
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Backpack)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Ink)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Ink)), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Parchment)), 10),
                Equipment::new(Item::Other("a book of lore".to_string()), 1),
                Equipment::new(Item::Other("a little bag of sand".to_string()), 1),
                Equipment::new(Item::Other("a small knife".to_string()), 1),
            ],
        }
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        match self {
            Pack::MonsterHunter => vec![EquipmentOption::HolySymbol],
            Pack::Burglar
            | Pack::Diplomat
            | Pack::Dungeoneer
            | Pack::Entertainer
            | Pack::Explorer
            | Pack::Priest
            | Pack::Scholar => vec![],
        }
    }
}
