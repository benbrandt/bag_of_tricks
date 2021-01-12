use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        features::Feature,
    },
    citation::{Book, Citation, Citations},
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Human;

#[typetag::serde]
impl Race for Human {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(
            AbilityScoreType::iter()
                .map(|t| AbilityScore(t, 1))
                .collect(),
        )
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 29,
        }])
    }

    fn features(&self) -> Vec<Feature> {
        vec![Feature {
            title: "Ability Score Increase",
            description: "Your ability scores each increase by 1.",
            citation: Citation {
                book: Book::PHB,
                page: 31,
            },
        }]
    }
}

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Human")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let human = Human::gen(&mut rng);
        // Struct Snapshot
        // insta::assert_yaml_snapshot!(human);
        // fmt::Display Snapshot
        insta::assert_snapshot!(format!("{}", human));
    }

    #[test]
    fn test_snapshot_abilities() {
        let human = Human;
        insta::assert_yaml_snapshot!(human.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let human = Human;
        insta::assert_yaml_snapshot!(human.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let human = Human;
        insta::assert_yaml_snapshot!(human.features());
    }
}
