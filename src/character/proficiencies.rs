use std::convert::TryFrom;

use rand::{
    distributions::WeightedIndex,
    prelude::{Distribution, IteratorRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::Display;

use super::{
    ability::Skill,
    equipment::{
        armor::ArmorType,
        tools::Tool,
        weapons::{WeaponCategory, WeaponType},
    },
    Character,
};

#[derive(Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum WeaponProficiency {
    Category(WeaponCategory),
    Specific(WeaponType),
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ProficiencyOption {
    From(Vec<Proficiency>),
    Skill,
}

impl ProficiencyOption {
    pub(crate) fn gen(&self, rng: &mut impl Rng, character: &Character) -> Proficiency {
        match self {
            Self::From(list) => list
                .clone()
                .into_iter()
                .filter(|p| !character.proficiencies().contains(p))
                .choose(rng)
                .unwrap(),
            Self::Skill => {
                let available_skills = Skill::iter()
                    .filter(|s| !character.proficiencies().contains(&Proficiency::Skill(*s)));
                // Weight the proficiencies based on their underlying ability score.
                let modifiers = available_skills
                    .clone()
                    .map(|s| character.abilities.modifier(s.ability_score_type()));
                let min = modifiers.clone().min().unwrap_or(0);
                // Make sure they are positive, and increase the weight of the higher ones
                let weights = modifiers.map(|m| {
                    let pos_mod =
                        usize::try_from(if min < 0 { m + min.abs() } else { m }).unwrap_or(0);
                    pos_mod * pos_mod
                });
                let dist = WeightedIndex::new(weights).unwrap();
                Proficiency::Skill(available_skills.collect::<Vec<Skill>>()[dist.sample(rng)])
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum Proficiency {
    Armor(ArmorType),
    Skill(Skill),
    Tool(Tool),
    Weapon(WeaponProficiency),
}

pub(crate) trait Proficiencies {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![]
    }
}
