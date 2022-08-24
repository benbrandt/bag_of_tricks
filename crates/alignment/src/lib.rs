#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::{convert::TryFrom, f64::consts::E, fmt};

use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

fn exp_weight(val: usize) -> f64 {
    E.powi(i32::try_from(val).unwrap_or_default())
}

#[derive(Clone, Copy, Deserialize, Display, EnumIter, Eq, PartialEq, Serialize)]
pub enum Attitude {
    Chaotic,
    Lawful,
    Neutral,
}

impl Attitude {
    fn weight(self, influences: &[Self]) -> f64 {
        exp_weight(influences.iter().filter(|&i| i == &self).count())
    }
}

#[derive(Clone, Copy, Deserialize, Display, EnumIter, Eq, PartialEq, Serialize)]
pub enum Morality {
    Evil,
    Good,
    Neutral,
}

impl Morality {
    fn weight(self, influences: &[Self]) -> f64 {
        exp_weight(influences.iter().filter(|&i| i == &self).count())
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

/// Character alignment, both attitude and morality
#[derive(Clone, Deserialize, Serialize)]
pub struct Alignment(pub Attitude, pub Morality);

impl Alignment {
    /// Generate alignment, weighted by influences from other choices on the character sheet
    ///
    /// # Panics
    ///
    /// Will panic if weighting logic is wrong or there are no attitudes/moralities at all
    pub fn gen(
        rng: &mut impl Rng,
        attitude_influences: &[Attitude],
        morality_influences: &[Morality],
    ) -> Self {
        let attitude = *Attitude::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |a| a.weight(attitude_influences))
            .unwrap();
        let morality = *Morality::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |a| a.weight(morality_influences))
            .unwrap();
        Self(attitude, morality)
    }

    /// Weight of a particular alignment based on influences.
    /// Useful for comparing things like deities.
    #[must_use]
    pub fn weight(
        &self,
        attitude_influences: &[Attitude],
        morality_influences: &[Morality],
    ) -> f64 {
        self.0.weight(attitude_influences) + self.1.weight(morality_influences)
    }
}

impl AlignmentInfluences for Alignment {
    fn attitude(&self) -> Vec<Attitude> {
        vec![self.0]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![self.1]
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
