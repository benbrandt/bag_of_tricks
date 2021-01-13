use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::{AgeRange, Characteristics, Gender},
        features::Feature,
        names::{human::Names, Name},
    },
    citation::{Book, Citation, Citations},
};

const AGE_RANGE: AgeRange = AgeRange(1..=100);

#[derive(Deserialize, Serialize)]
pub(crate) struct Human;

impl Human {
    pub(crate) fn gen_first_name<'a>(
        rng: &mut impl Rng,
        names: &'a Names,
        Characteristics { gender, .. }: &Characteristics,
    ) -> &'a str {
        let first_names = match gender {
            Gender::Female => names.female,
            Gender::Male => names.male,
        };
        first_names.iter().choose(rng).unwrap()
    }

    pub(crate) fn gen_surname<'a>(rng: &mut impl Rng, names: &'a Names) -> &'a str {
        names.surname.iter().choose(rng).unwrap()
    }
}

impl Name for Human {
    fn gen_name(rng: &mut impl Rng, characteristics: &Characteristics) -> String {
        let names = Names::gen_names(rng);
        format!(
            "{} {}",
            Human::gen_first_name(rng, &names, characteristics),
            Human::gen_surname(rng, &names),
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
