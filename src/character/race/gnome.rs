use rand::prelude::IteratorRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::ability::{AbilityScore, AbilityScoreType, AbilityScores},
    citation::{Book, Citation, Citations},
};

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum GnomeSubrace {
    Forest,
    Rock,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Gnome {
    subrace: GnomeSubrace,
}

#[typetag::serde]
impl Race for Gnome {
    fn new(rng: &mut impl Rng) -> Self {
        Self {
            subrace: GnomeSubrace::iter()
                .choose(rng)
                .unwrap_or(GnomeSubrace::Forest),
        }
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Intelligence, 2),
            match self.subrace {
                GnomeSubrace::Forest => AbilityScore(AbilityScoreType::Dexterity, 1),
                GnomeSubrace::Rock => AbilityScore(AbilityScoreType::Constitution, 1),
            },
        ])
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
}

impl fmt::Display for Gnome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Gnome", self.subrace)
    }
}
