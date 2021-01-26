mod acolyte;

use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::citation::Citations;

use self::acolyte::Acolyte;

use super::{
    alignment::{AlignmentInfluences, Attitude, Morality},
    features::Features,
    languages::Languages,
    proficiencies::Proficiencies,
};

#[derive(Clone, Copy, Deserialize, Display, Serialize)]
pub(crate) enum Influence {
    Any,
    Chaotic,
    Evil,
    Good,
    Lawful,
    Neutral,
}

impl AlignmentInfluences for Influence {
    fn attitude(&self) -> Vec<Attitude> {
        match self {
            Self::Chaotic => vec![Attitude::Chaotic],
            Self::Lawful => vec![Attitude::Lawful],
            Self::Neutral => vec![Attitude::Neutral],
            _ => vec![],
        }
    }

    fn morality(&self) -> Vec<Morality> {
        match self {
            Self::Good => vec![Morality::Good],
            Self::Evil => vec![Morality::Evil],
            Self::Neutral => vec![Morality::Neutral],
            _ => vec![],
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Personality {
    bond: String,
    flaw: String,
    ideal: (String, Influence),
    traits: Vec<String>,
}

impl AlignmentInfluences for Personality {
    fn attitude(&self) -> Vec<Attitude> {
        self.ideal.1.attitude()
    }

    fn morality(&self) -> Vec<Morality> {
        self.ideal.1.morality()
    }
}

impl fmt::Display for Personality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PERSONALITY TRAITS:")?;
        for t in &self.traits {
            writeln!(f, "{}", t)?;
        }
        writeln!(f, "IDEAL: {} ({})", self.ideal.0, self.ideal.1)?;
        writeln!(f, "BOND: {}", self.bond)?;
        writeln!(f, "FLAW: {}", self.flaw)
    }
}

pub(crate) trait PersonalityOptions {
    const BONDS: [&'static str; 6];
    const FLAWS: [&'static str; 6];
    const IDEALS: [(&'static str, Influence); 6];
    const TRAITS: [&'static str; 8];

    fn gen_personality(rng: &mut impl Rng) -> Personality {
        let ideal = *Self::IDEALS.iter().choose(rng).unwrap();
        Personality {
            bond: String::from(*Self::BONDS.iter().choose(rng).unwrap()),
            flaw: String::from(*Self::FLAWS.iter().choose(rng).unwrap()),
            ideal: (String::from(ideal.0), ideal.1),
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
