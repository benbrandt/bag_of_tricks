#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::{convert::TryFrom, f64::consts::E, fmt};

use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Deserialize, Display, EnumIter, PartialEq, Serialize)]
pub enum Attitude {
    Chaotic,
    Lawful,
    Neutral,
}

#[derive(Clone, Copy, Deserialize, Display, EnumIter, PartialEq, Serialize)]
pub enum Morality {
    Evil,
    Good,
    Neutral,
}

/// Choose from a given list of options weighted by number of influences for a given type
fn weighted_choice<T, I>(rng: &mut impl Rng, options: I, influences: &[T]) -> T
where
    T: Clone + Copy + PartialEq,
    I: Clone + Iterator<Item = T>,
{
    *options
        .collect::<Vec<T>>()
        .choose_weighted(rng, |t| {
            E.powi(i32::try_from(influences.iter().filter(|i| t == *i).count()).unwrap_or(0))
        })
        .unwrap()
}

/// Character alignment, both attitude and morality
#[derive(Deserialize, Serialize)]
pub struct Alignment(pub Attitude, pub Morality);

impl Alignment {
    /// Generate alignment, weighted by influences from other choices on the character sheet
    pub fn gen(
        rng: &mut impl Rng,
        attitude_influences: &[Attitude],
        morality_influences: &[Morality],
    ) -> Self {
        let attitude = weighted_choice(rng, Attitude::iter(), attitude_influences);
        let morality = weighted_choice(rng, Morality::iter(), morality_influences);
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

/// Trait to describe how this entity affects a character's alignment
pub trait AlignmentInfluences {
    /// List of attitude influences
    fn attitude(&self) -> Vec<Attitude> {
        vec![]
    }

    /// List of morality influences
    fn morality(&self) -> Vec<Morality> {
        vec![]
    }
}
