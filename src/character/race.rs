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
    fn gen(rng: &mut impl Rng) -> Self
    where
        Self: Sized;

    /// Generate a name for a character of this race
    fn gen_name(rng: &mut impl Rng, gender: &Gender) -> String
    where
        Self: Sized,
    {
        String::new()
    }

    /// Returns ability score increases for the race
    fn abilities(&self) -> AbilityScores;

    /// Return list of citations for this race/subrace
    fn citations(&self) -> Citations;

    /// Return list of features & traits for this race
    fn features(&self) -> Vec<Feature>;
}

#[derive(EnumIter)]
enum RaceOptions {
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

pub(crate) fn gen_race_option(rng: &mut impl Rng, gender: &Gender) -> (Box<dyn Race>, String) {
    match RaceOptions::iter().choose(rng).unwrap() {
        RaceOptions::Dragonborn => (
            Box::new(Dragonborn::gen(rng)),
            Dragonborn::gen_name(rng, gender),
        ),
        RaceOptions::Dwarf => (Box::new(Dwarf::gen(rng)), Dwarf::gen_name(rng, gender)),
        RaceOptions::Elf => (Box::new(Elf::gen(rng)), Elf::gen_name(rng, gender)),
        RaceOptions::Gnome => (Box::new(Gnome::gen(rng)), Gnome::gen_name(rng, gender)),
        RaceOptions::HalfElf => (Box::new(HalfElf::gen(rng)), HalfElf::gen_name(rng, gender)),
        RaceOptions::HalfOrc => (Box::new(HalfOrc::gen(rng)), HalfOrc::gen_name(rng, gender)),
        RaceOptions::Halfling => (
            Box::new(Halfling::gen(rng)),
            Halfling::gen_name(rng, gender),
        ),
        RaceOptions::Human => (Box::new(Human::gen(rng)), Human::gen_name(rng, gender)),
        RaceOptions::Tiefling => (
            Box::new(Tiefling::gen(rng)),
            Tiefling::gen_name(rng, gender),
        ),
    }
}
