use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;

use super::Race;
use crate::{
    character::ability::{AbilityScore, AbilityScoreType, AbilityScores},
    citation::{Book, Citation, Citations},
};

#[derive(Deserialize, Serialize)]
pub(crate) struct HalfElf {
    addl_increases: Vec<AbilityScore>,
}

#[typetag::serde]
impl Race for HalfElf {
    fn new(rng: &mut impl Rng) -> Self {
        Self {
            addl_increases: AbilityScoreType::iter()
                .filter(|s| s != &AbilityScoreType::Charisma)
                .choose_multiple(rng, 2)
                .into_iter()
                .map(|t| AbilityScore(t, 1))
                .collect(),
        }
    }

    fn abilities(&self) -> AbilityScores {
        let mut abilities = vec![AbilityScore(AbilityScoreType::Charisma, 2)];
        abilities.extend(self.addl_increases.clone());
        AbilityScores(abilities)
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 38,
        }])
    }
}

impl fmt::Display for HalfElf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Half-Elf")
    }
}
