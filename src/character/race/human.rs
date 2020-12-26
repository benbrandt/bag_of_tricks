use rand::Rng;
use std::fmt;

use super::Race;
use crate::{
    character::ability::AbilityScoreIncreases,
    citation::{Book, Citation},
};

pub(crate) struct Human;

impl Race for Human {
    fn new(_: &mut impl Rng) -> Self {
        Self
    }

    fn citations(&self) -> Vec<Citation> {
        vec![Citation {
            book: Book::PlayersHandbook,
            page: 29,
        }]
    }

    fn increases(&self) -> AbilityScoreIncreases {
        AbilityScoreIncreases {
            charisma: 1,
            constitution: 1,
            dexterity: 1,
            intelligence: 1,
            strength: 1,
            wisdom: 1,
        }
    }
}

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Human")
    }
}
