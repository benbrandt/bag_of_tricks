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
            elf::{CHILD, FAMILY, FEMALE, MALE},
            Name,
        },
    },
    citation::{Book, Citation, Citations},
};

const AGE_RANGE: AgeRange = AgeRange(1..=750);

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum ElfSubrace {
    Dark,
    High,
    Wood,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Elf {
    subrace: ElfSubrace,
}

impl Elf {
    pub(crate) fn gen_first_name<'a>(
        rng: &mut impl Rng,
        Characteristics { age, gender }: &Characteristics,
    ) -> &'a str {
        let first_names = match age {
            1..=100 => CHILD,
            _ => match gender {
                Gender::Female => FEMALE,
                Gender::Male => MALE,
            },
        };
        first_names.iter().choose(rng).unwrap()
    }

    pub(crate) fn gen_family_name<'a>(rng: &mut impl Rng) -> &'a str {
        FAMILY.iter().choose(rng).unwrap()
    }
}

impl Name for Elf {
    fn gen_name(rng: &mut impl Rng, characteristics: &Characteristics) -> String {
        format!(
            "{} {}",
            Elf::gen_first_name(rng, characteristics),
            Elf::gen_family_name(rng),
        )
    }
}

#[typetag::serde]
impl Race for Elf {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, Characteristics) {
        let race = Box::new(Self {
            subrace: ElfSubrace::iter().choose(rng).unwrap(),
        });
        let characteristics = Characteristics::gen(rng, &AGE_RANGE);
        let name = Elf::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            match self.subrace {
                ElfSubrace::Dark => AbilityScore(AbilityScoreType::Charisma, 1),
                ElfSubrace::High => AbilityScore(AbilityScoreType::Intelligence, 1),
                ElfSubrace::Wood => AbilityScore(AbilityScoreType::Wisdom, 1),
            },
        ])
    }

    fn citations(&self) -> Citations {
        let race = Citation {
            book: Book::PHB,
            page: 21,
        };
        let subrace = match self.subrace {
            ElfSubrace::Dark | ElfSubrace::Wood => Citation {
                book: Book::PHB,
                page: 24,
            },
            ElfSubrace::High => Citation {
                book: Book::PHB,
                page: 23,
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
                page: 23,
            },
        }];
        features.extend(match self.subrace {
            ElfSubrace::Dark => vec![Feature {
                title: "Ability Score Increase",
                description: "Your Charisma score increases by 1.",
                citation: Citation {
                    book: Book::PHB,
                    page: 24,
                },
            }],
            ElfSubrace::High => vec![Feature {
                title: "Ability Score Increase",
                description: "Your Intelligence score increases by 1.",
                citation: Citation {
                    book: Book::PHB,
                    page: 23,
                },
            }],
            ElfSubrace::Wood => vec![Feature {
                title: "Ability Score Increase",
                description: "Your Wisdom score increases by 1.",
                citation: Citation {
                    book: Book::PHB,
                    page: 24,
                },
            }],
        });
        features
    }
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Elf", self.subrace)
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
        let elf = Elf::gen(&mut rng);
        insta::assert_yaml_snapshot!(elf);
    }

    #[test]
    fn test_snapshot_display() {
        for subrace in ElfSubrace::iter() {
            let elf = Elf { subrace };
            insta::assert_snapshot!(format!("{}", elf));
        }
    }

    #[test]
    fn test_snapshot_abilities() {
        for subrace in ElfSubrace::iter() {
            let elf = Elf { subrace };
            insta::assert_yaml_snapshot!(elf.abilities());
        }
    }

    #[test]
    fn test_snapshot_citations() {
        for subrace in ElfSubrace::iter() {
            let elf = Elf { subrace };
            insta::assert_yaml_snapshot!(elf.citations());
        }
    }

    #[test]
    fn test_snapshot_features() {
        for subrace in ElfSubrace::iter() {
            let elf = Elf { subrace };
            insta::assert_yaml_snapshot!(elf.features());
        }
    }
}
