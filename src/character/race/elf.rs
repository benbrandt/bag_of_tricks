use rand::prelude::IteratorRandom;
use rand::Rng;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::character::ability::AbilityScoreIncreases;

#[derive(Display, EnumIter)]
enum ElfSubrace {
    Drow,
    #[strum(serialize = "High Elf")]
    High,
    #[strum(serialize = "Wood Elf")]
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

    fn increases(&self) -> AbilityScoreIncreases {
        let increases = AbilityScoreIncreases {
            dexterity: 2,
            ..AbilityScoreIncreases::default()
        };
        match self.subrace {
            ElfSubrace::Drow => AbilityScoreIncreases {
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
        write!(f, "{}", self.subrace)
    }
}
