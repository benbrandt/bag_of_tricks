use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::{AgeRange, Characteristics, Gender},
        features::Feature,
        names::{
            human::{
                CALISHITE, CHONDATHAN, DAMARAN, ILLUSKAN, MULAN, RASHEMI, SHOU, TETHYRIAN, TURAMI,
            },
            Name,
        },
    },
    citation::{Book, Citation, Citations},
};

const AGE_RANGE: AgeRange = AgeRange(1..=100);

#[derive(EnumIter)]
enum Ethnicity {
    Calishite,
    Chondathan,
    Damaran,
    Illuskan,
    Mulan,
    Rashemi,
    Shou,
    Tethyrian,
    Turami,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Human;

impl Name for Human {
    fn gen_name(rng: &mut impl Rng, Characteristics { gender, .. }: &Characteristics) -> String {
        let ethnicity = match Ethnicity::iter().choose(rng).unwrap() {
            Ethnicity::Calishite => CALISHITE,
            Ethnicity::Chondathan => CHONDATHAN,
            Ethnicity::Damaran => DAMARAN,
            Ethnicity::Illuskan => ILLUSKAN,
            Ethnicity::Mulan => MULAN,
            Ethnicity::Rashemi => RASHEMI,
            Ethnicity::Shou => SHOU,
            Ethnicity::Tethyrian => TETHYRIAN,
            Ethnicity::Turami => TURAMI,
        };
        let first_names = match gender {
            Gender::Female => ethnicity.female,
            Gender::Male => ethnicity.male,
        };
        format!(
            "{} {}",
            first_names.iter().choose(rng).unwrap(),
            ethnicity.surname.iter().choose(rng).unwrap(),
        )
    }
}

#[typetag::serde]
impl Race for Human {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, Characteristics) {
        let race = Box::new(Self);
        let characteristics = Characteristics::gen(rng, &AGE_RANGE);
        let name = Human::gen_name(rng, &characteristics);
        (race, name, characteristics)
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
    fn test_snapshot() {
        let mut rng = Pcg64::seed_from_u64(1);
        let human = Human::gen(&mut rng);
        insta::assert_yaml_snapshot!(human);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (human, _name, _characteristics) = Human::gen(&mut rng);
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
