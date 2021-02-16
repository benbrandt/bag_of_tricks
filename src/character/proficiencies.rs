use rand::{
    prelude::{Distribution, IteratorRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::Display;

use super::{
    ability::{weighted_modifiers_dist, Skill},
    equipment::{
        armor::ArmorType,
        tools::{ArtisansTools, GamingSet, MusicalInstrument, Tool},
        vehicles::VehicleProficiency,
        weapons::{WeaponCategory, WeaponType},
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
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ProficiencyOption {
    /// Choose from a given list of proficiency options.
    From(Vec<Proficiency>),
    /// Choose a random artisan's tools to be proficient in.
    ArtisansTools,
    /// Choose a random gaming set to be proficient in.
    GamingSet,
    /// Choose a random musical instrument to be proficient in.
    MusicalInstrument,
    /// Choose a random skill to be proficient in (weighted towards you highest modifiers)
    Skill,
}

impl ProficiencyOption {
    /// Randomly choose a given proficiency option, avoiding already existing proficiencies.
    pub(crate) fn gen(&self, rng: &mut impl Rng, character: &Character) -> Proficiency {
        match self {
            Self::From(list) => list
                .clone()
                .into_iter()
                .filter(|p| !character.proficiencies().contains(p))
                .choose(rng)
                .unwrap(),
            Self::ArtisansTools => Self::From(
                ArtisansTools::iter()
                    .map(|g| Proficiency::Tool(Tool::ArtisansTools(g)))
                    .collect(),
            )
            .gen(rng, character),
            Self::GamingSet => Self::From(
                GamingSet::iter()
                    .map(|g| Proficiency::Tool(Tool::GamingSet(g)))
                    .collect(),
            )
            .gen(rng, character),
            Self::MusicalInstrument => Self::From(
                MusicalInstrument::iter()
                    .map(|m| Proficiency::Tool(Tool::MusicalInstrument(m)))
                    .collect(),
            )
            .gen(rng, character),
            Self::Skill => {
                let available_skills = Skill::iter()
                    .filter(|s| !character.proficiencies().contains(&Proficiency::Skill(*s)));
                // Weight the proficiencies based on their underlying ability score.
                let modifiers = available_skills
                    .clone()
                    .map(|s| character.abilities.modifier(s.ability_score_type()));
                let dist = weighted_modifiers_dist(modifiers);
                Proficiency::Skill(available_skills.collect::<Vec<Skill>>()[dist.sample(rng)])
            }
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
