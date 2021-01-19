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

use super::{
    ability::AbilityScores,
    attack::{Attack, DamageType},
    characteristics::CharacteristicDetails,
    features::Feature,
    Character,
};

/// Shared race traits
#[typetag::serde(tag = "type")]
pub(crate) trait Race: fmt::Display {
    /// Method to generate a new instance of the struct
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails)
    where
        Self: Sized;

    /// Returns ability score increases for the race
    fn abilities(&self) -> AbilityScores;

    /// Returns list of attacks for the race
    fn attacks(&self, character: &Character) -> Vec<Attack> {
        vec![]
    }

    /// Return list of citations for this race/subrace
    fn citations(&self) -> Citations;

    /// Return list of features & traits for this race
    fn features(&self) -> Vec<Feature>;

    /// Return list of resistances for this race
    fn resistances(&self) -> Vec<DamageType> {
        vec![]
    }
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
