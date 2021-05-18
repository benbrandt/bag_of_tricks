mod barbarian;
mod bard;
mod cleric;
mod druid;
mod fighter;
mod monk;
mod paladin;
mod ranger;
mod rogue;
mod sorcerer;
mod warlock;
mod wizard;

use std::fmt;

use backstory::Backstory;
use citation::{CitationList, Citations};
use deities::{Pantheon, PantheonWeight, Pantheons};
use features::{Feature, Features};
use gear::currency::Coin;
use languages::{Language, LanguageType, Languages};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{exp_weight, AbilityScoreType, AbilityScores},
    equipment::{Equipment, EquipmentOption, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
};
use strum::{EnumIter, IntoEnumIterator};

use self::{
    barbarian::Barbarian, bard::Bard, cleric::Cleric, druid::Druid, fighter::Fighter, monk::Monk,
    paladin::Paladin, ranger::Ranger, rogue::Rogue, sorcerer::Sorcerer, warlock::Warlock,
    wizard::Wizard,
};

fn max_score_mod(types: &[AbilityScoreType], ability_scores: &AbilityScores) -> i16 {
    types
        .iter()
        .map(|&t| ability_scores.modifier(t))
        .max()
        .unwrap_or_default()
}

pub(crate) fn class_weight(
    primary: &[AbilityScoreType],
    secondary: &[AbilityScoreType],
    ability_scores: &AbilityScores,
) -> f64 {
    let primary_weight = exp_weight(
        max_score_mod(primary, ability_scores),
        ability_scores.shift_weight_by(),
    );
    // Not exponential, just minor bump for secondary
    let secondary_weight =
        f64::from(max_score_mod(secondary, ability_scores) + ability_scores.shift_weight_by());
    primary_weight + secondary_weight
}

pub trait Class:
    Backstory
    + Citations
    + Features
    + Languages
    + Pantheons
    + Proficiencies
    + StartingEquipment
    + fmt::Display
{
    /// Generate new instance of class
    fn gen(rng: &mut impl Rng) -> Self;

    /// Return primary and secondary ability ranking
    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>);

    /// Weight based on quick build suggestions in Player's Handbook
    fn weight(ability_scores: &AbilityScores) -> f64 {
        let (primary, secondary) = Self::ability_rank();
        class_weight(&primary, &secondary, ability_scores)
    }
}

#[impl_enum::with_methods {
    pub fn addl_equipment(&self) -> Vec<EquipmentOption> {}
    pub fn addl_languages(&self) -> (usize, Option<LanguageType>) {}
    pub fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {}
    pub fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {}
    pub fn backstory(&self) -> Vec<String> {}
    pub fn citations(&self) -> CitationList {}
    pub fn coins(&self) -> (Coin, u8) {}
    pub fn deity_required(&self) -> bool {}
    pub fn equipment(&self) -> Vec<Equipment> {}
    pub fn features(&self) -> Vec<Feature> {}
    pub fn languages(&self) -> Vec<Language> {}
    pub fn proficiencies(&self) -> Vec<Proficiency> {}
    pub fn weight(ability_scores: &AbilityScores) -> f64 {}
}]
#[derive(Deserialize, EnumIter, Serialize)]
pub enum ClassOption {
    Barbarian(Barbarian),
    Bard(Bard),
    Cleric(Cleric),
    Druid(Druid),
    Fighter(Fighter),
    Monk(Monk),
    Paladin(Paladin),
    Ranger(Ranger),
    Rogue(Rogue),
    Sorcerer(Sorcerer),
    Warlock(Warlock),
    Wizard(Wizard),
}

impl ClassOption {
    /// Choose a random background option, weighted by ability scores
    pub fn gen(rng: &mut impl Rng, ability_scores: &AbilityScores) -> Self {
        let options: Vec<ClassOption> = Self::iter().collect();
        let option = options
            .choose_weighted(rng, |o| o.weight(ability_scores))
            .unwrap();
        match option {
            Self::Barbarian(_) => Self::Barbarian(Barbarian::gen(rng)),
            Self::Bard(_) => Self::Bard(Bard::gen(rng)),
            Self::Cleric(_) => Self::Cleric(Cleric::gen(rng)),
            Self::Druid(_) => Self::Druid(Druid::gen(rng)),
            Self::Fighter(_) => Self::Fighter(Fighter::gen(rng)),
            Self::Monk(_) => Self::Monk(Monk::gen(rng)),
            Self::Paladin(_) => Self::Paladin(Paladin::gen(rng)),
            Self::Ranger(_) => Self::Ranger(Ranger::gen(rng)),
            Self::Rogue(_) => Self::Rogue(Rogue::gen(rng)),
            Self::Sorcerer(_) => Self::Sorcerer(Sorcerer::gen(rng)),
            Self::Warlock(_) => Self::Warlock(Warlock::gen(rng)),
            Self::Wizard(_) => Self::Wizard(Wizard::gen(rng)),
        }
    }
}

impl fmt::Display for ClassOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Barbarian(c) => write!(f, "{}", c),
            Self::Bard(c) => write!(f, "{}", c),
            Self::Cleric(c) => write!(f, "{}", c),
            Self::Druid(c) => write!(f, "{}", c),
            Self::Fighter(c) => write!(f, "{}", c),
            Self::Monk(c) => write!(f, "{}", c),
            Self::Paladin(c) => write!(f, "{}", c),
            Self::Ranger(c) => write!(f, "{}", c),
            Self::Rogue(c) => write!(f, "{}", c),
            Self::Sorcerer(c) => write!(f, "{}", c),
            Self::Warlock(c) => write!(f, "{}", c),
            Self::Wizard(c) => write!(f, "{}", c),
        }
    }
}
