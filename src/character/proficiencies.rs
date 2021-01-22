use rand::{prelude::IteratorRandom, Rng};
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
};

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize)]
pub(crate) enum WeaponProficiency {
    Category(WeaponCategory),
    Specific(WeaponType),
}

pub(crate) enum ProficiencyOption {
    From(Vec<Proficiency>),
    Skill,
}

impl ProficiencyOption {
    pub(crate) fn gen(&self, rng: &mut impl Rng, existing: &[Proficiency]) -> Vec<Proficiency> {
        match self {
            Self::From(list) => list
                .clone()
                .into_iter()
                .filter(|p| !existing.contains(p))
                .choose_multiple(rng, 1),
            Self::Skill => Skill::iter()
                .map(Proficiency::Skill)
                .filter(|s| !existing.contains(s))
                .choose_multiple(rng, 1),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize)]
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
