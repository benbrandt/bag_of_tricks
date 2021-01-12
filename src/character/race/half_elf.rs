use rand::{prelude::IteratorRandom, Rng};
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
pub(crate) struct HalfElf {
    addl_increases: Vec<AbilityScore>,
}

#[typetag::serde]
impl Race for HalfElf {
    fn gen(rng: &mut impl Rng) -> Self {
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

    fn features(&self) -> Vec<Feature> {
        vec![Feature {
            title: "Ability Score Increase",
            description:
                "Your Charisma score increases by 2, and two other ability scores of your choice increase by 1.",
            citation: Citation {
                book: Book::PHB,
                page: 39,
            },
        }]
    }
}

impl fmt::Display for HalfElf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Half-Elf")
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
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", half_elf));
    }

    #[test]
    fn test_snapshot_abilities() {
        let mut rng = Pcg64::seed_from_u64(1);
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.features());
    }
}
