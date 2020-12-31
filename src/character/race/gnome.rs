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

    fn citations(&self) -> Citations {
        let race = Citation {
            book: Book::PHB,
            page: 35,
        };
        let subrace = match self.subrace {
            GnomeSubrace::Forest | GnomeSubrace::Rock => Citation {
                book: Book::PHB,
                page: 37,
            },
        };
        Citations(vec![race, subrace])
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
