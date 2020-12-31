use rand::Rng;
use std::fmt;

use super::Race;
use crate::{
    character::ability::AbilityScoreIncreases,
    citation::{Book, Citation, Citations},
};

pub(crate) struct Dragonborn;

impl Race for Dragonborn {
    fn new(_: &mut impl Rng) -> Self {
        Self
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 32,
        }])
    }

    fn increases(&self) -> AbilityScoreIncreases {
        AbilityScoreIncreases {
            charisma: 1,
            constitution: 0,
            dexterity: 0,
            intelligence: 0,
            strength: 2,
            wisdom: 0,
        }
    }
}

impl fmt::Display for Dragonborn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dragonborn")
    }
}
