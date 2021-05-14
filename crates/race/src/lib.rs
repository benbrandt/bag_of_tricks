mod aasimar;
mod bugbear;
mod dragonborn;
mod dwarf;
mod elf;
mod firbolg;
mod gith;
mod gnome;
mod goblin;
mod goliath;
mod half_elf;
mod half_orc;
mod halfling;
mod hobgoblin;
mod human;
mod kenku;
mod kobold;
mod lizardfolk;
mod orc;
mod tabaxi;
mod tiefling;
mod triton;
mod yuan_ti;

use std::fmt;

use alignment::AlignmentInfluences;
use attack::Resistances;
use backstory::Backstory;
use characteristics::{names::Name, Appearance, Characteristics};
use citation::Citations;
use deities::Pantheons;
use features::Features;
use languages::Languages;
use personality::PersonalityOptions;
use rand::prelude::IteratorRandom;
use rand::Rng;
use stats::{ability::AbilityScore, proficiencies::Proficiencies};
use strum::{EnumIter, IntoEnumIterator};
use trinkets::Trinkets;

use self::{
    aasimar::Aasimar, bugbear::Bugbear, dragonborn::Dragonborn, dwarf::Dwarf, elf::Elf,
    firbolg::Firbolg, gith::Gith, gnome::Gnome, goblin::Goblin, goliath::Goliath,
    half_elf::HalfElf, half_orc::HalfOrc, halfling::Halfling, hobgoblin::Hobgoblin, human::Human,
    kenku::Kenku, kobold::Kobold, lizardfolk::Lizardfolk, orc::Orc, tabaxi::Tabaxi,
    tiefling::Tiefling, triton::Triton, yuan_ti::YuanTiPureblood,
};

/// Shared racial traits each race should provide.
#[typetag::serde(tag = "type")]
pub trait Race:
    AlignmentInfluences
    + Appearance
    + Backstory
    + Characteristics
    + Citations
    + Features
    + Languages
    + Name
    + Pantheons
    + PersonalityOptions
    + Proficiencies
    + Resistances
    + Trinkets
    + fmt::Display
{
    /// Method to generate a new instance of the struct
    fn gen(rng: &mut impl Rng) -> Self
    where
        Self: Sized;

    /// Returns ability score increases for the race
    fn abilities(&self) -> Vec<AbilityScore>;
}

/// All currently supported Race Options for character creation.
#[derive(EnumIter)]
pub enum RaceOptions {
    Aasimar,
    Bugbear,
    Dragonborn,
    Dwarf,
    Elf,
    Firbolg,
    Gith,
    Gnome,
    Goblin,
    Goliath,
    HalfElf,
    HalfOrc,
    Halfling,
    Hobgoblin,
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
    pub fn gen(rng: &mut impl Rng) -> Box<dyn Race> {
        match Self::iter().choose(rng).unwrap() {
            Self::Aasimar => Box::new(Aasimar::gen(rng)),
            Self::Bugbear => Box::new(Bugbear::gen(rng)),
            Self::Dragonborn => Box::new(Dragonborn::gen(rng)),
            Self::Dwarf => Box::new(Dwarf::gen(rng)),
            Self::Elf => Box::new(Elf::gen(rng)),
            Self::Firbolg => Box::new(Firbolg::gen(rng)),
            Self::Gith => Box::new(Gith::gen(rng)),
            Self::Gnome => Box::new(Gnome::gen(rng)),
            Self::Goblin => Box::new(Goblin::gen(rng)),
            Self::Goliath => Box::new(Goliath::gen(rng)),
            Self::HalfElf => Box::new(HalfElf::gen(rng)),
            Self::HalfOrc => Box::new(HalfOrc::gen(rng)),
            Self::Halfling => Box::new(Halfling::gen(rng)),
            Self::Hobgoblin => Box::new(Hobgoblin::gen(rng)),
            Self::Human => Box::new(Human::gen(rng)),
            Self::Kenku => Box::new(Kenku::gen(rng)),
            Self::Kobold => Box::new(Kobold::gen(rng)),
            Self::Lizardfolk => Box::new(Lizardfolk::gen(rng)),
            Self::Orc => Box::new(Orc::gen(rng)),
            Self::Tabaxi => Box::new(Tabaxi::gen(rng)),
            Self::Tiefling => Box::new(Tiefling::gen(rng)),
            Self::Triton => Box::new(Triton::gen(rng)),
            Self::YuanTiPureblood => Box::new(YuanTiPureblood::gen(rng)),
        }
    }
}
