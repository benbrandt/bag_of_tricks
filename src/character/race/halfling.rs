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
        characteristics::Characteristics,
        features::Feature,
    },
    citation::{Book, Citation, Citations},
};

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
        (
            Box::new(Self {
                subrace: HalflingSubrace::iter().choose(rng).unwrap(),
            }),
            todo!(),
            todo!(),
        )
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
