use std::fmt;

use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Deserialize, Display, EnumIter, PartialEq, Serialize)]
pub(crate) enum Attitude {
    Chaotic,
    Lawful,
    Neutral,
}

#[derive(Clone, Copy, Deserialize, Display, EnumIter, PartialEq, Serialize)]
pub(crate) enum Morality {
    Evil,
    Good,
    Neutral,
}

fn weighted_choice<T, I>(rng: &mut impl Rng, options: I, tendency: Option<T>) -> T
where
    T: Clone + Copy + PartialEq,
    I: Clone + Iterator<Item = T>,
{
    let weights = options
        .clone()
        .map(|a| if Some(a) == tendency { 2 } else { 1 });
    options.collect::<Vec<T>>()[WeightedIndex::new(weights).unwrap().sample(rng)]
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Alignment(Attitude, Morality);

impl Alignment {
    pub(crate) fn gen(
        rng: &mut impl Rng,
        attitude_tendency: Option<Attitude>,
        morality_tendency: Option<Morality>,
    ) -> Self {
        let attitude = weighted_choice(rng, Attitude::iter(), attitude_tendency);
        let morality = weighted_choice(rng, Morality::iter(), morality_tendency);
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
