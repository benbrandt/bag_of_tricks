mod dragonborn;
mod dwarf;
mod elf;
mod gnome;
mod half_elf;
mod half_orc;
mod halfling;
mod human;
mod kobold;
mod tiefling;
mod yuan_ti;

use rand::prelude::IteratorRandom;
use rand::Rng;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::citation::Citations;

use self::{
    dragonborn::Dragonborn, dwarf::Dwarf, elf::Elf, gnome::Gnome, half_elf::HalfElf,
    half_orc::HalfOrc, halfling::Halfling, human::Human, tiefling::Tiefling,
};

use super::{
    ability::AbilityScore, alignment::AlignmentInfluences, attack::Resistances,
    characteristics::CharacteristicDetails, features::Features, languages::Languages,
    proficiencies::Proficiencies,
};

/// Shared racial traits each race should provide.
#[typetag::serde(tag = "type")]
pub(crate) trait Race:
    AlignmentInfluences + Citations + Features + Languages + Proficiencies + Resistances + fmt::Display
{
    /// Method to generate a new instance of the struct
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails)
    where
        Self: Sized;

    /// Returns ability score increases for the race
    fn abilities(&self) -> Vec<AbilityScore>;
}

/// All currently supported Race Options for character creation.
#[derive(EnumIter)]
pub(crate) enum RaceOptions {
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    HalfOrc,
    Halfling,
    Human,
    Tiefling,
}

impl RaceOptions {
    /// Randomly choose a race option and return the result of the corresponding racial struct's `gen` method
    pub(crate) fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        match Self::iter().choose(rng).unwrap() {
            Self::Dragonborn => Dragonborn::gen(rng),
            Self::Dwarf => Dwarf::gen(rng),
            Self::Elf => Elf::gen(rng),
            Self::Gnome => Gnome::gen(rng),
            Self::HalfElf => HalfElf::gen(rng),
            Self::HalfOrc => HalfOrc::gen(rng),
            Self::Halfling => Halfling::gen(rng),
            Self::Human => Human::gen(rng),
            Self::Tiefling => Tiefling::gen(rng),
        }
    }
}
