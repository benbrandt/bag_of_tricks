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

use super::{ability::AbilityScores, features::Feature};

/// Shared race traits
#[typetag::serde(tag = "type")]
pub(crate) trait Race: fmt::Display {
    /// Method to generate a new instance of the struct
    fn gen(rng: &mut impl Rng) -> Self
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

pub(crate) fn gen_race_option(rng: &mut impl Rng) -> Box<dyn Race> {
    match RaceOptions::iter()
        .choose(rng)
        .unwrap_or(RaceOptions::Dragonborn)
    {
        RaceOptions::Dragonborn => Box::new(dragonborn::Dragonborn::gen(rng)),
        RaceOptions::Dwarf => Box::new(dwarf::Dwarf::gen(rng)),
        RaceOptions::Elf => Box::new(elf::Elf::gen(rng)),
        RaceOptions::Gnome => Box::new(gnome::Gnome::gen(rng)),
        RaceOptions::HalfElf => Box::new(half_elf::HalfElf::gen(rng)),
        RaceOptions::HalfOrc => Box::new(half_orc::HalfOrc::gen(rng)),
        RaceOptions::Halfling => Box::new(halfling::Halfling::gen(rng)),
        RaceOptions::Human => Box::new(human::Human::gen(rng)),
        RaceOptions::Tiefling => Box::new(tiefling::Tiefling::gen(rng)),
    }
}
