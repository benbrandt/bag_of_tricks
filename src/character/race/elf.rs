use rand::prelude::IteratorRandom;
use rand::Rng;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::ability::AbilityScoreIncreases,
    citation::{Book, Citation},
};

#[derive(Display, EnumIter)]
enum ElfSubrace {
    Dark,
    High,
    Wood,
}

pub(crate) struct Elf {
    subrace: ElfSubrace,
}

impl Race for Elf {
    fn new(rng: &mut impl Rng) -> Self {
        Self {
            subrace: ElfSubrace::iter().choose(rng).unwrap_or(ElfSubrace::Wood),
        }
    }

    fn citations(&self) -> Vec<Citation> {
        let elf = Citation {
            book: Book::PlayersHandbook,
            page: 21,
        };
        let subrace = match self.subrace {
            ElfSubrace::Dark | ElfSubrace::Wood => Citation {
                book: Book::PlayersHandbook,
                page: 24,
            },
            ElfSubrace::High => Citation {
                book: Book::PlayersHandbook,
                page: 23,
            },
        };
        vec![elf, subrace]
    }

    fn increases(&self) -> AbilityScoreIncreases {
        let increases = AbilityScoreIncreases {
            dexterity: 2,
            ..AbilityScoreIncreases::default()
        };
        match self.subrace {
            ElfSubrace::Dark => AbilityScoreIncreases {
                charisma: 1,
                ..increases
            },
            ElfSubrace::High => AbilityScoreIncreases {
                intelligence: 1,
                ..increases
            },
            ElfSubrace::Wood => AbilityScoreIncreases {
                wisdom: 1,
                ..increases
            },
        }
    }
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Elf", self.subrace)
    }
}
