use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        features::Feature,
    },
    citation::{Book, Citation, Citations},
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Tiefling;

#[typetag::serde]
impl Race for Tiefling {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Charisma, 2),
            AbilityScore(AbilityScoreType::Intelligence, 1),
        ])
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 42,
        }])
    }

    fn features(&self) -> Vec<Feature> {
        vec![Feature {
            title: "Ability Score Increase",
            description:
                "Your Intelligence score increases by 1, and your Charisma score increases by 12",
            citation: Citation {
                book: Book::PHB,
                page: 43,
            },
        }]
    }
}

impl fmt::Display for Tiefling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tiefling")
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
        let tiefling = Tiefling::gen(&mut rng);
        // Struct Snapshot
        // insta::assert_yaml_snapshot!(tiefling);
        // fmt::Display Snapshot
        insta::assert_snapshot!(format!("{}", tiefling));
    }

    #[test]
    fn test_snapshot_abilities() {
        let tiefling = Tiefling;
        insta::assert_yaml_snapshot!(tiefling.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let tiefling = Tiefling;
        insta::assert_yaml_snapshot!(tiefling.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let tiefling = Tiefling;
        insta::assert_yaml_snapshot!(tiefling.features());
    }
}
