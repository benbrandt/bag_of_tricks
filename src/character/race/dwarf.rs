use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::{AgeRange, Characteristics, Gender},
        features::Feature,
        names::{
            dwarf::{CLAN, FEMALE, MALE},
            Name,
        },
    },
    citation::{Book, Citation, Citations},
};

const AGE_RANGE: AgeRange = AgeRange(1..=350);

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum DwarfSubrace {
    Hill,
    Mountain,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Dwarf {
    subrace: DwarfSubrace,
}

impl Name for Dwarf {
    fn gen_name(rng: &mut impl Rng, Characteristics { gender, .. }: &Characteristics) -> String {
        let first_names = match gender {
            Gender::Female => FEMALE,
            Gender::Male => MALE,
        };
        format!(
            "{} {}",
            first_names.iter().choose(rng).unwrap(),
            CLAN.iter().choose(rng).unwrap()
        )
    }
}

#[typetag::serde]
impl Race for Dwarf {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, Characteristics) {
        let race = Box::new(Self {
            subrace: DwarfSubrace::iter().choose(rng).unwrap(),
        });
        let characteristics = Characteristics::gen(rng, &AGE_RANGE);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Constitution, 2),
            match self.subrace {
                DwarfSubrace::Hill => AbilityScore(AbilityScoreType::Wisdom, 1),
                DwarfSubrace::Mountain => AbilityScore(AbilityScoreType::Strength, 2),
            },
        ])
    }

    fn citations(&self) -> Citations {
        let race = Citation {
            book: Book::PHB,
            page: 18,
        };
        let subrace = match self.subrace {
            DwarfSubrace::Hill | DwarfSubrace::Mountain => Citation {
                book: Book::PHB,
                page: 20,
            },
        };
        Citations(vec![race, subrace])
    }

    fn features(&self) -> Vec<Feature> {
        let mut features = vec![Feature {
            title: "Ability Score Increase",
            description: "Your Constitution score increases by 2.",
            citation: Citation {
                book: Book::PHB,
                page: 20,
            },
        }];
        features.extend(match self.subrace {
            DwarfSubrace::Hill => vec![Feature {
                title: "Ability Score Increase",
                description: "Your Wisdom score increases by 1.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            }],
            DwarfSubrace::Mountain => vec![Feature {
                title: "Ability Score Increase",
                description: "Your Strength score increases by 2.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            }],
        });
        features
    }
}

impl fmt::Display for Dwarf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Dwarf", self.subrace)
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
        let dwarf = Dwarf::gen(&mut rng);
        insta::assert_yaml_snapshot!(dwarf);
    }

    #[test]
    fn test_snapshot_display() {
        for subrace in DwarfSubrace::iter() {
            let dwarf = Dwarf { subrace };
            insta::assert_snapshot!(format!("{}", dwarf));
        }
    }

    #[test]
    fn test_snapshot_abilities() {
        for subrace in DwarfSubrace::iter() {
            let dwarf = Dwarf { subrace };
            insta::assert_yaml_snapshot!(dwarf.abilities());
        }
    }

    #[test]
    fn test_snapshot_citations() {
        for subrace in DwarfSubrace::iter() {
            let dwarf = Dwarf { subrace };
            insta::assert_yaml_snapshot!(dwarf.citations());
        }
    }

    #[test]
    fn test_snapshot_features() {
        for subrace in DwarfSubrace::iter() {
            let dwarf = Dwarf { subrace };
            insta::assert_yaml_snapshot!(dwarf.features());
        }
    }
}
