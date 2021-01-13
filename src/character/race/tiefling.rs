use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{human::Human, Race};
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::{AgeRange, Characteristics, Gender},
        features::Feature,
        names::{
            human::Names,
            tiefling::{FEMALE_ABYSSAL, MALE_ABYSSAL, VIRTUE_NAMES},
            Name,
        },
    },
    citation::{Book, Citation, Citations},
};

const AGE_RANGE: AgeRange = AgeRange(1..=100);

#[derive(Deserialize, Serialize)]
pub(crate) struct Tiefling;

impl Tiefling {
    fn gen_first_name<'a>(
        rng: &mut impl Rng,
        names: &'a Names,
        characteristics: &Characteristics,
    ) -> &'a str {
        let abyssal_name = *(match characteristics.gender {
            Gender::Female => FEMALE_ABYSSAL,
            Gender::Male => MALE_ABYSSAL,
        })
        .iter()
        .choose(rng)
        .unwrap();
        let human_name = Human::gen_first_name(rng, names, characteristics);
        let virtue_name = *VIRTUE_NAMES.iter().choose(rng).unwrap();
        *[abyssal_name, human_name, virtue_name]
            .iter()
            .choose(rng)
            .unwrap()
    }
}

impl Name for Tiefling {
    fn gen_name(rng: &mut impl Rng, characteristics: &Characteristics) -> String {
        let names = Names::gen_names(rng);
        format!(
            "{} {}",
            Self::gen_first_name(rng, &names, characteristics),
            Human::gen_surname(rng, &names)
        )
    }
}

#[typetag::serde]
impl Race for Tiefling {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, Characteristics) {
        let race = Box::new(Self);
        let characteristics = Characteristics::gen(rng, &AGE_RANGE);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
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
    fn test_snapshot() {
        let mut rng = Pcg64::seed_from_u64(1);
        let tiefling = Tiefling::gen(&mut rng);
        insta::assert_yaml_snapshot!(tiefling);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (tiefling, _name, _characteristics) = Tiefling::gen(&mut rng);
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
