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
pub(crate) struct HalfOrc;

#[typetag::serde]
impl Race for HalfOrc {
    fn gen(_rng: &mut impl Rng) -> (Box<dyn Race>, String, Characteristics) {
        (Box::new(Self), todo!(), todo!())
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

    fn features(&self) -> Vec<Feature> {
        vec![Feature {
            title: "Ability Score Increase",
            description:
                "Your Strength score increases by 2, and your Constitution score increases by 1.",
            citation: Citation {
                book: Book::PHB,
                page: 41,
            },
        }]
    }
}

impl fmt::Display for HalfOrc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Half-Orc")
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
        let half_orc = HalfOrc::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_orc);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (half_orc, _name, _characteristics) = HalfOrc::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", half_orc));
    }

    #[test]
    fn test_snapshot_abilities() {
        let half_orc = HalfOrc;
        insta::assert_yaml_snapshot!(half_orc.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let half_orc = HalfOrc;
        insta::assert_yaml_snapshot!(half_orc.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let half_orc = HalfOrc;
        insta::assert_yaml_snapshot!(half_orc.features());
    }
}
