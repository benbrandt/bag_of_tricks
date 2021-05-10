use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::{Display, IntoEnumIterator};

use super::{
    ability::Skill,
    equipment::{
        armor::ArmorType,
        tools::{ArtisansTools, GamingSet, MusicalInstrument, Tool},
        vehicles::VehicleProficiency,
        weapons::{WeaponCategory, WeaponClassification, WeaponType},
    },
    Character,
};

/// Types of weapons a character is proficient in.
#[derive(Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum WeaponProficiency {
    /// Proficiency in an entire category of weapons
    Category(WeaponCategory),
    /// Proficiency in a specific weapon type
    Specific(WeaponType),
}

/// A way to encapsulate a proficiency that needs to be chosen for a character.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum ProficiencyOption {
    /// Choose from a given list of proficiency options.
    From(Vec<Proficiency>, usize),
    /// Choose from multiple options (usually Musical Instrument _OR_ Gaming Set)
    FromOptions(Vec<ProficiencyOption>, usize),
    /// Choose a random artisan's tools to be proficient in.
    ArtisansTools,
    /// Choose a random gaming set to be proficient in.
    GamingSet,
    /// Choose a random musical instrument to be proficient in.
    MusicalInstrument,
    /// Choose a random skill to be proficient in (weighted towards you highest modifiers)
    Skill(Option<Vec<Skill>>, usize),
    /// Choose a random tool to be proficient in
    Tool(usize),
    /// Choose a random weapon.
    Weapon(Option<WeaponCategory>, Option<WeaponClassification>, usize),
}

impl ProficiencyOption {
    /// Randomly choose a given proficiency option, avoiding already existing proficiencies.
    pub(crate) fn gen(&self, rng: &mut impl Rng, character: &Character) -> Vec<Proficiency> {
        match self {
            Self::From(list, amount) => list
                .clone()
                .into_iter()
                .filter(|p| !character.proficiencies.contains(p))
                .choose_multiple(rng, *amount),
            Self::FromOptions(choices, amount) => {
                let mut options: Vec<Proficiency> = choices
                    .choose_multiple(rng, *amount)
                    .flat_map(|c| c.gen(rng, character))
                    .collect();
                // Add some more if we didn't get enough
                if options.len() < *amount {
                    options.extend(
                        Self::FromOptions(choices.clone(), *amount - options.len())
                            .gen(rng, character),
                    );
                }
                options
            }
            Self::ArtisansTools => Self::From(
                ArtisansTools::iter()
                    .map(|g| Proficiency::Tool(Tool::ArtisansTools(g)))
                    .collect(),
                1,
            )
            .gen(rng, character),
            Self::GamingSet => Self::From(
                GamingSet::iter()
                    .map(|g| Proficiency::Tool(Tool::GamingSet(g)))
                    .collect(),
                1,
            )
            .gen(rng, character),
            Self::MusicalInstrument => Self::From(
                MusicalInstrument::iter()
                    .map(|m| Proficiency::Tool(Tool::MusicalInstrument(m)))
                    .collect(),
                1,
            )
            .gen(rng, character),
            Self::Skill(skills, amount) => {
                let available_skills = skills
                    .clone()
                    .unwrap_or_else(|| Skill::iter().collect())
                    .into_iter()
                    .filter(|&s| !character.proficiencies.contains(&Proficiency::Skill(s)));
                let mut skills = available_skills
                    .collect::<Vec<_>>()
                    .choose_multiple_weighted(rng, *amount, |s| s.weight(character))
                    .unwrap()
                    .map(|&s| Proficiency::Skill(s))
                    .collect::<Vec<_>>();
                // Add some more if we didn't get enough
                if skills.len() < *amount {
                    skills.extend(Self::Skill(None, *amount - skills.len()).gen(rng, character));
                }
                skills
            }
            Self::Tool(amount) => {
                let mut tools = Self::FromOptions(
                    Tool::iter()
                        .filter_map(|t| match t {
                            Tool::ArtisansTools(_) => Some(ProficiencyOption::ArtisansTools),
                            Tool::GamingSet(_) => Some(ProficiencyOption::GamingSet),
                            Tool::MusicalInstrument(_) => {
                                Some(ProficiencyOption::MusicalInstrument)
                            }
                            _ => (!character.proficiencies.contains(&Proficiency::Tool(t)))
                                .then(|| ProficiencyOption::From(vec![Proficiency::Tool(t)], 1)),
                        })
                        .collect(),
                    *amount,
                )
                .gen(rng, character);
                // Add some more if we didn't get enough
                if tools.len() < *amount {
                    tools.extend(Self::Tool(*amount - tools.len()).gen(rng, character));
                }
                tools
            }
            Self::Weapon(category, classification, amount) => Self::From(
                WeaponType::iter()
                    .filter(|w| {
                        if let Some(c) = category {
                            if c != &w.category() {
                                return false;
                            }
                        } else if let Some(c) = classification {
                            if c != &w.classification() {
                                return false;
                            }
                        }
                        true
                    })
                    .map(|w| Proficiency::Weapon(WeaponProficiency::Specific(w)))
                    .collect(),
                *amount,
            )
            .gen(rng, character),
        }
    }
}

/// Types of proficiencies
#[derive(Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum Proficiency {
    Armor(ArmorType),
    Skill(Skill),
    Tool(Tool),
    Vehicle(VehicleProficiency),
    Weapon(WeaponProficiency),
}

impl Proficiency {
    // Sometimes you end up with dupes. Consume and replace with a new option
    pub(crate) fn gen_replacement(self, rng: &mut impl Rng, character: &Character) -> Vec<Self> {
        match self {
            Self::Skill(_) => ProficiencyOption::Skill(None, 1).gen(rng, character),
            Self::Tool(_) => ProficiencyOption::Tool(1).gen(rng, character),
            Self::Weapon(_) => ProficiencyOption::Weapon(None, None, 1).gen(rng, character),
            Self::Armor(_) | Self::Vehicle(_) => todo!(),
        }
    }
}

/// Trait to describe proficiencies given by an entity and any additional choices that can be made.
pub(crate) trait Proficiencies {
    /// Proficiencies given by an entity/object
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![]
    }

    /// Proficiency options given by an entity/object that need to be chosen.
    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![]
    }
}