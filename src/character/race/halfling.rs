use rand::prelude::IteratorRandom;
use rand::Rng;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::ability::AbilityScoreIncreases,
    citation::{Book, Citation, Citations},
};

#[derive(Debug, Display, EnumIter, PartialEq)]
enum HalflingSubrace {
    Lightfoot,
    Stout,
}

pub(crate) struct Halfling {
    subrace: HalflingSubrace,
}

impl Race for Halfling {
    fn new(rng: &mut impl Rng) -> Self {
        Self {
            subrace: HalflingSubrace::iter()
                .choose(rng)
                .unwrap_or(HalflingSubrace::Lightfoot),
        }
    }

    fn citations(&self) -> Citations {
        let race = Citation {
            book: Book::PHB,
            page: 26,
        };
        let subrace = match self.subrace {
            HalflingSubrace::Lightfoot | HalflingSubrace::Stout => Citation {
                book: Book::PHB,
                page: 28,
            },
        };
        Citations(vec![race, subrace])
    }

    fn increases(&self) -> AbilityScoreIncreases {
        let increases = AbilityScoreIncreases {
            dexterity: 2,
            ..AbilityScoreIncreases::default()
        };
        match self.subrace {
            HalflingSubrace::Lightfoot => AbilityScoreIncreases {
                charisma: 1,
                ..increases
            },
            HalflingSubrace::Stout => AbilityScoreIncreases {
                constitution: 1,
                ..increases
            },
        }
    }
}

impl fmt::Display for Halfling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Halfling", self.subrace)
    }
}
