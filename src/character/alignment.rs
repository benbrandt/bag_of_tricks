use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Attitude {
    Chaotic,
    Lawful,
    Neutral,
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Morality {
    Evil,
    Good,
    Neutral,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Alignment(Attitude, Morality);

impl Alignment {
    pub(crate) fn gen(rng: &mut impl Rng) -> Self {
        let attitude = Attitude::iter().choose(rng).unwrap();
        let morality = Morality::iter().choose(rng).unwrap();
        Self(attitude, morality)
    }
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self(Attitude::Neutral, Morality::Neutral) => write!(f, "Neutral"),
            Self(a, m) => write!(f, "{} {}", a, m),
        }
    }
}
