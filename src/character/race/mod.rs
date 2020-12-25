mod dragonborn;
mod dwarf;
mod elf;

use rand::prelude::IteratorRandom;
use rand::Rng;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::ability::AbilityScoreIncreases;

/// Shared race traits
pub(crate) trait Race: fmt::Display {
    /// Method to generate a new instance of the struct
    fn new(rng: &mut impl Rng) -> Self
    where
        Self: Sized;

    /// Returns ability score increases for the race
    fn increases(&self) -> AbilityScoreIncreases;
}

#[derive(EnumIter)]
enum RaceOptions {
    Dragonborn,
    Dwarf,
    Elf,
}

pub(crate) fn gen_race_option(rng: &mut impl Rng) -> Box<dyn Race> {
    match RaceOptions::iter()
        .choose(rng)
        .unwrap_or(RaceOptions::Dragonborn)
    {
        RaceOptions::Dragonborn => Box::new(dragonborn::Dragonborn::new(rng)),
        RaceOptions::Dwarf => Box::new(dwarf::Dwarf::new(rng)),
        RaceOptions::Elf => Box::new(elf::Elf::new(rng)),
    }
}
