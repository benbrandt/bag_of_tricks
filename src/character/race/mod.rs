pub(crate) mod dragonborn;
pub(crate) mod dwarf;

use std::fmt;

use rand::Rng;
use rand_derive::Rand;

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

#[derive(Rand)]
enum RaceOptions {
    Dragonborn,
    Dwarf,
}

pub(crate) fn gen_race_option(rng: &mut impl Rng) -> Box<dyn Race> {
    match rng.gen::<RaceOptions>() {
        RaceOptions::Dragonborn => Box::new(dragonborn::Dragonborn::new(rng)),
        RaceOptions::Dwarf => Box::new(dwarf::Dwarf::new(rng)),
    }
}
