mod dragonborn;
mod dwarf;
mod elf;
mod gnome;
mod half_elf;
mod half_orc;
mod halfling;
mod human;
mod tiefling;

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

use super::{ability::AbilityScores, features::Feature, Gender};

/// Shared race traits
#[typetag::serde(tag = "type")]
pub(crate) trait Race: fmt::Display {
    /// Method to generate a new instance of the struct
    fn gen(rng: &mut impl Rng, gender: &Gender) -> (Box<dyn Race>, String)
    where
        Self: Sized;

    /// Returns ability score increases for the race
    fn abilities(&self) -> AbilityScores;

    /// Return list of citations for this race/subrace
    fn citations(&self) -> Citations;

    /// Return list of features & traits for this race
    fn features(&self) -> Vec<Feature>;
}

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
    pub(crate) fn gen(rng: &mut impl Rng, gender: &Gender) -> (Box<dyn Race>, String) {
        match Self::iter().choose(rng).unwrap() {
            Self::Dragonborn => Dragonborn::gen(rng, gender),
            Self::Dwarf => Dwarf::gen(rng, gender),
            Self::Elf => Elf::gen(rng, gender),
            Self::Gnome => Gnome::gen(rng, gender),
            Self::HalfElf => HalfElf::gen(rng, gender),
            Self::HalfOrc => HalfOrc::gen(rng, gender),
            Self::Halfling => Halfling::gen(rng, gender),
            Self::Human => Human::gen(rng, gender),
            Self::Tiefling => Tiefling::gen(rng, gender),
        }
    }
}
