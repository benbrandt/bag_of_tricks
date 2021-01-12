use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::Race;
use crate::{
    character::ability::{AbilityScore, AbilityScoreType, AbilityScores},
    citation::{Book, Citation, Citations},
};

#[derive(Deserialize, Serialize)]
pub(crate) struct HalfOrc;

#[typetag::serde]
impl Race for HalfOrc {
    fn new(_: &mut impl Rng) -> Self {
        Self
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Constitution, 1),
            AbilityScore(AbilityScoreType::Strength, 2),
        ])
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 40,
        }])
    }
}

impl fmt::Display for HalfOrc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Half-Orc")
    }
}
