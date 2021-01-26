mod acolyte;

use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::citation::Citations;

use self::acolyte::Acolyte;

use super::{features::Features, languages::Languages, proficiencies::Proficiencies};

#[derive(Deserialize, Serialize)]
pub(crate) struct Personality {
    bond: String,
    flaw: String,
    ideal: String,
    traits: Vec<String>,
}

impl fmt::Display for Personality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PERSONALITY TRAITS:")?;
        for t in &self.traits {
            writeln!(f, "{}", t)?;
        }
        writeln!(f, "IDEAL: {}", self.ideal)?;
        writeln!(f, "BOND: {}", self.bond)?;
        writeln!(f, "FLAW: {}", self.flaw)
    }
}

pub(crate) trait PersonalityOptions {
    const BONDS: [&'static str; 6];
    const FLAWS: [&'static str; 6];
    const IDEALS: [&'static str; 6];
    const TRAITS: [&'static str; 8];

    fn gen_personality(rng: &mut impl Rng) -> Personality {
        Personality {
            bond: String::from(*Self::BONDS.iter().choose(rng).unwrap()),
            flaw: String::from(*Self::FLAWS.iter().choose(rng).unwrap()),
            ideal: String::from(*Self::IDEALS.iter().choose(rng).unwrap()),
            traits: Self::TRAITS
                .iter()
                .map(|s| String::from(*s))
                .choose_multiple(rng, 2),
        }
    }
}

#[typetag::serde(tag = "type")]
pub(crate) trait Background:
    Citations + Features + Languages + Proficiencies + fmt::Display
{
    /// Generate a new instance of the background
    fn gen(rng: &mut impl Rng) -> (Box<dyn Background>, Personality)
    where
        Self: Sized;

    fn equipment(&self) -> String;
}

#[derive(EnumIter)]
pub(crate) enum BackgroundOptions {
    Acolyte,
}

impl BackgroundOptions {
    pub(crate) fn gen(rng: &mut impl Rng) -> (Box<dyn Background>, Personality) {
        match Self::iter().choose(rng).unwrap() {
            Self::Acolyte => Acolyte::gen(rng),
        }
    }
}
