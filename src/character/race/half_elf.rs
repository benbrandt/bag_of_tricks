use rand::{prelude::IteratorRandom, Rng};
use std::fmt;
use strum::IntoEnumIterator;

use super::Race;
use crate::{
    character::ability::{AbilityScoreIncreases, AbilityScoreType},
    citation::{Book, Citation},
};

pub(crate) struct HalfElf {
    addl_increases: Vec<AbilityScoreType>,
}

impl HalfElf {
    fn score_increase(&self, score: AbilityScoreType) -> i8 {
        if self.addl_increases.contains(&score) {
            1
        } else {
            0
        }
    }
}

impl Race for HalfElf {
    fn new(rng: &mut impl Rng) -> Self {
        Self {
            addl_increases: AbilityScoreType::iter()
                .filter(|s| s != &AbilityScoreType::Charisma)
                .choose_multiple(rng, 2),
        }
    }

    fn citations(&self) -> Vec<Citation> {
        vec![Citation {
            book: Book::PlayersHandbook,
            page: 38,
        }]
    }

    fn increases(&self) -> AbilityScoreIncreases {
        AbilityScoreIncreases {
            charisma: 2,
            constitution: self.score_increase(AbilityScoreType::Constitution),
            dexterity: self.score_increase(AbilityScoreType::Dexterity),
            intelligence: self.score_increase(AbilityScoreType::Intelligence),
            strength: self.score_increase(AbilityScoreType::Strength),
            wisdom: self.score_increase(AbilityScoreType::Wisdom),
        }
    }
}

impl fmt::Display for HalfElf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Half-Elf")
    }
}
