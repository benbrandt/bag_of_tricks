mod aasimar;
mod dragonborn;
mod dwarf;
mod elf;
mod firbolg;
mod gnome;
mod goliath;
mod half_elf;
mod half_orc;
mod halfling;
mod human;
mod kenku;
mod kobold;
mod lizardfolk;
mod orc;
mod origins;
mod tabaxi;
mod tiefling;
mod triton;
mod yuan_ti;

use rand::prelude::IteratorRandom;
use rand::Rng;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::citation::Citations;

use self::{
    aasimar::Aasimar, dragonborn::Dragonborn, dwarf::Dwarf, elf::Elf, firbolg::Firbolg,
    gnome::Gnome, goliath::Goliath, half_elf::HalfElf, half_orc::HalfOrc, halfling::Halfling,
    human::Human, kenku::Kenku, kobold::Kobold, lizardfolk::Lizardfolk, orc::Orc, tabaxi::Tabaxi,
    tiefling::Tiefling, triton::Triton, yuan_ti::YuanTiPureblood,
};

use super::{
    ability::AbilityScore, alignment::AlignmentInfluences, attack::Resistances,
    backstory::Backstory, characteristics::CharacteristicDetails, features::Features,
    languages::Languages, proficiencies::Proficiencies,
};

/// Shared racial traits each race should provide.
#[typetag::serde(tag = "type")]
pub(crate) trait Race:
    AlignmentInfluences
    + Backstory
    + Citations
    + Features
    + Languages
    + Proficiencies
    + Resistances
    + fmt::Display
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
    Aasimar,
    Dragonborn,
    Dwarf,
    Elf,
    Firbolg,
    Gnome,
    Goliath,
    HalfElf,
    HalfOrc,
    Halfling,
    Human,
    Kenku,
    Kobold,
    Lizardfolk,
    Orc,
    Tabaxi,
    Tiefling,
    Triton,
    YuanTiPureblood,
}

impl RaceOptions {
    /// Randomly choose a race option and return the result of the corresponding racial struct's `gen` method
    pub(crate) fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        match Self::iter().choose(rng).unwrap() {
            Self::Aasimar => Aasimar::gen(rng),
            Self::Dragonborn => Dragonborn::gen(rng),
            Self::Dwarf => Dwarf::gen(rng),
            Self::Elf => Elf::gen(rng),
            Self::Firbolg => Firbolg::gen(rng),
            Self::Gnome => Gnome::gen(rng),
            Self::Goliath => Goliath::gen(rng),
            Self::HalfElf => HalfElf::gen(rng),
            Self::HalfOrc => HalfOrc::gen(rng),
            Self::Halfling => Halfling::gen(rng),
            Self::Human => Human::gen(rng),
            Self::Kenku => Kenku::gen(rng),
            Self::Kobold => Kobold::gen(rng),
            Self::Lizardfolk => Lizardfolk::gen(rng),
            Self::Orc => Orc::gen(rng),
            Self::Tabaxi => Tabaxi::gen(rng),
            Self::Tiefling => Tiefling::gen(rng),
            Self::Triton => Triton::gen(rng),
            Self::YuanTiPureblood => YuanTiPureblood::gen(rng),
        }
    }
}
