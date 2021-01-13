use rand::prelude::IteratorRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::{AgeRange, Characteristics},
        features::Feature,
    },
    citation::{Book, Citation, Citations},
};

const AGE_RANGE: AgeRange = AgeRange(1..=150);
mod names {
    use rand::{prelude::IteratorRandom, Rng};

    use crate::character::characteristics::{Characteristics, Gender};

    const FAMILY: &[&str] = &[
        "Brushgather",
        "Goodbarrel",
        "Greenbottle",
        "High-hill",
        "Hilltopple",
        "Leagallow",
        "Tealeaf",
        "Thorngage",
        "Tosscobble",
        "Underbough",
    ];

    const FEMALE: &[&str] = &[
        "Andry",
        "Bree",
        "Callie",
        "Cora",
        "Euphemia",
        "Jillian",
        "Kithri",
        "Lavinia",
        "Lidda",
        "Merla",
        "Nedda",
        "Paela",
        "Portia",
        "Seraphina",
        "Shaena",
        "Trym",
        "Vani",
        "Verna",
    ];

    const MALE: &[&str] = &[
        "Alton", "Ander", "Cade", "Corrin", "Eldon", "Errich", "Finnan", "Garret", "Lindal",
        "Lyle", "Merric", "Milo", "Osborn", "Perrin", "Reed", "Roscoe", "Wellby",
    ];

    pub(crate) fn gen_name(
        rng: &mut impl Rng,
        Characteristics { gender, .. }: &Characteristics,
    ) -> String {
        let first_names = match gender {
            Gender::Female => FEMALE,
            Gender::Male => MALE,
        };
        format!(
            "{} {}",
            first_names.iter().choose(rng).unwrap(),
            FAMILY.iter().choose(rng).unwrap(),
        )
    }
}

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum HalflingSubrace {
    Lightfoot,
    Stout,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Halfling {
    subrace: HalflingSubrace,
}

#[typetag::serde]
impl Race for Halfling {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, Characteristics) {
        let race = Box::new(Self {
            subrace: HalflingSubrace::iter().choose(rng).unwrap(),
        });
        let characteristics = Characteristics::gen(rng, &AGE_RANGE);
        let name = names::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            match self.subrace {
                HalflingSubrace::Lightfoot => AbilityScore(AbilityScoreType::Charisma, 1),
                HalflingSubrace::Stout => AbilityScore(AbilityScoreType::Constitution, 1),
            },
        ])
    }

    fn citations(&self) -> Citations {
        let race = Citation {
            book: Book::PHB,
            page: 26,
        };
        let subrace = match self.subrace {
            HalflingSubrace::Lightfoot | HalflingSubrace::Stout => Citation {
                book: Book::PHB,
                page: 28,
            },
        };
        Citations(vec![race, subrace])
    }

    fn features(&self) -> Vec<Feature> {
        let mut features = vec![Feature {
            title: "Ability Score Increase",
            description: "Your Dexterity score increases by 2.",
            citation: Citation {
                book: Book::PHB,
                page: 28,
            },
        }];
        features.extend(match self.subrace {
            HalflingSubrace::Lightfoot => vec![Feature {
                title: "Ability Score Increase",
                description: "Your Charisma score increases by 1.",
                citation: Citation {
                    book: Book::PHB,
                    page: 28,
                },
            }],
            HalflingSubrace::Stout => vec![Feature {
                title: "Ability Score Increase",
                description: "Your Constitution score increases by 2.",
                citation: Citation {
                    book: Book::PHB,
                    page: 28,
                },
            }],
        });
        features
    }
}

impl fmt::Display for Halfling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Halfling", self.subrace)
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
        let halfling = Halfling::gen(&mut rng);
        insta::assert_yaml_snapshot!(halfling);
    }

    #[test]
    fn test_snapshot_display() {
        for subrace in HalflingSubrace::iter() {
            let halfling = Halfling { subrace };
            insta::assert_snapshot!(format!("{}", halfling));
        }
    }

    #[test]
    fn test_snapshot_abilities() {
        for subrace in HalflingSubrace::iter() {
            let halfling = Halfling { subrace };
            insta::assert_yaml_snapshot!(halfling.abilities());
        }
    }

    #[test]
    fn test_snapshot_citations() {
        for subrace in HalflingSubrace::iter() {
            let halfling = Halfling { subrace };
            insta::assert_yaml_snapshot!(halfling.citations());
        }
    }

    #[test]
    fn test_snapshot_features() {
        for subrace in HalflingSubrace::iter() {
            let halfling = Halfling { subrace };
            insta::assert_yaml_snapshot!(halfling.features());
        }
    }
}
