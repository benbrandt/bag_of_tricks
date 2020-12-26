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
enum GnomeSubrace {
    Forest,
    Rock,
}

pub(crate) struct Gnome {
    subrace: GnomeSubrace,
}

impl Race for Gnome {
    fn new(rng: &mut impl Rng) -> Self {
        Self {
            subrace: GnomeSubrace::iter()
                .choose(rng)
                .unwrap_or(GnomeSubrace::Forest),
        }
    }

    fn citations(&self) -> Vec<Citation> {
        let gnome = Citation {
            book: Book::PlayersHandbook,
            page: 35,
        };
        let subrace = match self.subrace {
            GnomeSubrace::Forest => Citation {
                book: Book::PlayersHandbook,
                page: 37,
            },
            GnomeSubrace::Rock => Citation {
                book: Book::PlayersHandbook,
                page: 37,
            },
        };
        vec![gnome, subrace]
    }

    fn increases(&self) -> AbilityScoreIncreases {
        let increases = AbilityScoreIncreases {
            intelligence: 2,
            ..AbilityScoreIncreases::default()
        };
        match self.subrace {
            GnomeSubrace::Forest => AbilityScoreIncreases {
                dexterity: 1,
                ..increases
            },
            GnomeSubrace::Rock => AbilityScoreIncreases {
                constitution: 1,
                ..increases
            },
        }
    }
}

impl fmt::Display for Gnome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Gnome", self.subrace)
    }
}
