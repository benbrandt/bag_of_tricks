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

#[derive(Debug, Display, EnumIter, PartialEq)]
enum DwarfSubrace {
    #[strum(serialize = "Hill Dwarf")]
    Hill,
    #[strum(serialize = "Mountain Dwarf")]
    Mountain,
}

pub(crate) struct Dwarf {
    subrace: DwarfSubrace,
}

impl Race for Dwarf {
    fn new(rng: &mut impl Rng) -> Self {
        Self {
            subrace: DwarfSubrace::iter()
                .choose(rng)
                .unwrap_or(DwarfSubrace::Hill),
        }
    }

    fn citations(&self) -> Vec<Citation> {
        let dwarf = Citation {
            book: Book::PlayersHandbook,
            page: 18,
        };
        let subrace = match self.subrace {
            DwarfSubrace::Hill => Citation {
                book: Book::PlayersHandbook,
                page: 20,
            },
            DwarfSubrace::Mountain => Citation {
                book: Book::PlayersHandbook,
                page: 20,
            },
        };
        vec![dwarf, subrace]
    }

    fn increases(&self) -> AbilityScoreIncreases {
        let increases = AbilityScoreIncreases {
            constitution: 2,
            ..AbilityScoreIncreases::default()
        };
        match self.subrace {
            DwarfSubrace::Hill => AbilityScoreIncreases {
                wisdom: 1,
                ..increases
            },
            DwarfSubrace::Mountain => AbilityScoreIncreases {
                strength: 2,
                ..increases
            },
        }
    }
}

impl fmt::Display for Dwarf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.subrace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_dwarf_new() {
        let mut rng = Pcg64::from_entropy();
        let race = Dwarf::new(&mut rng);
        assert_eq!(race.subrace, DwarfSubrace::Hill);
    }
}
