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

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::{DamageType, Resistances};
use backstory::Backstory;
use characteristics::{names::Name, Appearance, CharacteristicDetails, Characteristics};
use citation::{CitationList, Citations};
use deities::{Pantheon, PantheonWeight, Pantheons};
use features::{Feature, Features};
use languages::{Language, LanguageType, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::AbilityScore,
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
};
use strum::{EnumIter, IntoEnumIterator};
use trinkets::{TrinketOption, Trinkets};

use self::{
    aasimar::Aasimar, bugbear::Bugbear, dragonborn::Dragonborn, dwarf::Dwarf, elf::Elf,
    firbolg::Firbolg, gith::Gith, gnome::Gnome, goblin::Goblin, goliath::Goliath,
    half_elf::HalfElf, half_orc::HalfOrc, halfling::Halfling, hobgoblin::Hobgoblin, human::Human,
    kenku::Kenku, kobold::Kobold, lizardfolk::Lizardfolk, orc::Orc, tabaxi::Tabaxi,
    tiefling::Tiefling, triton::Triton, yuan_ti::YuanTiPureblood,
};

/// Shared racial traits each race should provide.
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
    fn gen(rng: &mut impl Rng) -> Self;

    /// Returns ability score increases for the race
    fn abilities(&self) -> Vec<AbilityScore>;
}

#[impl_enum::with_methods {
    pub fn abilities(&self) -> Vec<AbilityScore> {}
    pub fn addl_languages(&self) -> (usize, Option<LanguageType>) {}
    pub fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {}
    pub fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {}
    pub fn appearance(&self) -> Vec<String> {}
    pub fn attitude(&self) -> Vec<Attitude> {}
    pub fn backstory(&self) -> Vec<String> {}
    pub  fn citations(&self) -> CitationList {}
    pub fn bonds(&self) -> Vec<String> {}
    pub fn deity_required(&self) -> bool {}
    pub fn features(&self) -> Vec<Feature> {}
    pub fn flaws(&self) -> Vec<String> {}
    pub fn gen_characteristics(&self, rng: &mut impl Rng) -> CharacteristicDetails {}
    pub fn gen_name(&self, rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {}
    pub fn ideals(&self) -> Vec<(String, Influence)> {}
    pub fn immunities(&self) -> Vec<DamageType> {}
    pub fn languages(&self) -> Vec<Language> {}
    pub fn morality(&self) -> Vec<Morality> {}
    pub fn proficiencies(&self) -> Vec<Proficiency> {}
    pub fn resistances(&self) -> Vec<DamageType> {}
    pub fn traits(&self) -> Vec<String> {}
    pub fn trinket_options(&self) -> Vec<TrinketOption> {}
}]
#[derive(Deserialize, EnumIter, Serialize)]
pub enum RaceOption {
    Aasimar(Aasimar),
    Bugbear(Bugbear),
    Dragonborn(Dragonborn),
    Dwarf(Dwarf),
    Elf(Elf),
    Firbolg(Firbolg),
    Gith(Gith),
    Gnome(Gnome),
    Goblin(Goblin),
    Goliath(Goliath),
    HalfElf(HalfElf),
    HalfOrc(HalfOrc),
    Halfling(Halfling),
    Hobgoblin(Hobgoblin),
    Human(Human),
    Kenku(Kenku),
    Kobold(Kobold),
    Lizardfolk(Lizardfolk),
    Orc(Orc),
    Tabaxi(Tabaxi),
    Tiefling(Tiefling),
    Triton(Triton),
    YuanTiPureblood(YuanTiPureblood),
}

impl RaceOption {
    /// Randomly choose a race option and return the result of the corresponding racial struct's `gen` method
    pub fn gen(rng: &mut impl Rng) -> Self {
        match Self::iter().choose(rng).unwrap() {
            Self::Aasimar(_) => Self::Aasimar(Aasimar::gen(rng)),
            Self::Bugbear(_) => Self::Bugbear(Bugbear::gen(rng)),
            Self::Dragonborn(_) => Self::Dragonborn(Dragonborn::gen(rng)),
            Self::Dwarf(_) => Self::Dwarf(Dwarf::gen(rng)),
            Self::Elf(_) => Self::Elf(Elf::gen(rng)),
            Self::Firbolg(_) => Self::Firbolg(Firbolg::gen(rng)),
            Self::Gith(_) => Self::Gith(Gith::gen(rng)),
            Self::Gnome(_) => Self::Gnome(Gnome::gen(rng)),
            Self::Goblin(_) => Self::Goblin(Goblin::gen(rng)),
            Self::Goliath(_) => Self::Goliath(Goliath::gen(rng)),
            Self::HalfElf(_) => Self::HalfElf(HalfElf::gen(rng)),
            Self::HalfOrc(_) => Self::HalfOrc(HalfOrc::gen(rng)),
            Self::Halfling(_) => Self::Halfling(Halfling::gen(rng)),
            Self::Hobgoblin(_) => Self::Hobgoblin(Hobgoblin::gen(rng)),
            Self::Human(_) => Self::Human(Human::gen(rng)),
            Self::Kenku(_) => Self::Kenku(Kenku::gen(rng)),
            Self::Kobold(_) => Self::Kobold(Kobold::gen(rng)),
            Self::Lizardfolk(_) => Self::Lizardfolk(Lizardfolk::gen(rng)),
            Self::Orc(_) => Self::Orc(Orc::gen(rng)),
            Self::Tabaxi(_) => Self::Tabaxi(Tabaxi::gen(rng)),
            Self::Tiefling(_) => Self::Tiefling(Tiefling::gen(rng)),
            Self::Triton(_) => Self::Triton(Triton::gen(rng)),
            Self::YuanTiPureblood(_) => Self::YuanTiPureblood(YuanTiPureblood::gen(rng)),
        }
    }
}

impl fmt::Display for RaceOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Aasimar(r) => write!(f, "{}", r),
            Self::Bugbear(r) => write!(f, "{}", r),
            Self::Dragonborn(r) => write!(f, "{}", r),
            Self::Dwarf(r) => write!(f, "{}", r),
            Self::Elf(r) => write!(f, "{}", r),
            Self::Firbolg(r) => write!(f, "{}", r),
            Self::Gith(r) => write!(f, "{}", r),
            Self::Gnome(r) => write!(f, "{}", r),
            Self::Goblin(r) => write!(f, "{}", r),
            Self::Goliath(r) => write!(f, "{}", r),
            Self::HalfElf(r) => write!(f, "{}", r),
            Self::HalfOrc(r) => write!(f, "{}", r),
            Self::Halfling(r) => write!(f, "{}", r),
            Self::Hobgoblin(r) => write!(f, "{}", r),
            Self::Human(r) => write!(f, "{}", r),
            Self::Kenku(r) => write!(f, "{}", r),
            Self::Kobold(r) => write!(f, "{}", r),
            Self::Lizardfolk(r) => write!(f, "{}", r),
            Self::Orc(r) => write!(f, "{}", r),
            Self::Tabaxi(r) => write!(f, "{}", r),
            Self::Tiefling(r) => write!(f, "{}", r),
            Self::Triton(r) => write!(f, "{}", r),
            Self::YuanTiPureblood(r) => write!(f, "{}", r),
        }
    }
}
