use rand::Rng;
use std::fmt;

use super::Race;
use crate::{
    character::ability::AbilityScoreIncreases,
    citation::{Book, Citation, Citations},
};

pub(crate) struct Tiefling;

impl Race for Tiefling {
    fn new(_: &mut impl Rng) -> Self {
        Self
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 42,
        }])
    }

    fn increases(&self) -> AbilityScoreIncreases {
        AbilityScoreIncreases {
            charisma: 2,
            constitution: 0,
            dexterity: 0,
            intelligence: 1,
            strength: 0,
            wisdom: 0,
        }
    }
}

impl fmt::Display for Tiefling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tiefling")
    }
}
