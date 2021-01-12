use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::Characteristics,
        features::Feature,
    },
    citation::{Book, Citation, Citations},
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Dragonborn;

#[typetag::serde]
impl Race for Dragonborn {
    fn gen(_rng: &mut impl Rng) -> (Box<dyn Race>, String, Characteristics) {
        (Box::new(Self), todo!(), todo!())
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Charisma, 1),
            AbilityScore(AbilityScoreType::Strength, 2),
        ])
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 32,
        }])
    }

    fn features(&self) -> Vec<Feature> {
        vec![Feature {
            title: "Ability Score Increase",
            description:
                "Your Strength score increases by 2, and your Charisma score increases by 1.",
            citation: Citation {
                book: Book::PHB,
                page: 34,
            },
        }]
    }
}

impl fmt::Display for Dragonborn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dragonborn")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_snapshot() {
        let mut rng = Pcg64::seed_from_u64(1);
        let dragonborn = Dragonborn::gen(&mut rng);
        insta::assert_yaml_snapshot!(dragonborn);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (dragonborn, _name, _characteristics) = Dragonborn::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", dragonborn));
    }

    #[test]
    fn test_snapshot_abilities() {
        let dragonborn = Dragonborn;
        insta::assert_yaml_snapshot!(dragonborn.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let dragonborn = Dragonborn;
        insta::assert_yaml_snapshot!(dragonborn.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let dragonborn = Dragonborn;
        insta::assert_yaml_snapshot!(dragonborn.features());
    }
}
