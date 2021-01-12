use rand::{prelude::IteratorRandom, Rng};
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

const AGE_RANGE: AgeRange = AgeRange(1..=350);

mod names {
    use rand::{prelude::IteratorRandom, Rng};

    use crate::character::characteristics::Gender;

    const CLAN_NAMES: &[&str] = &[
        "Balderk",
        "Battlehammer",
        "Brawnanvil",
        "Dankil",
        "Fireforge",
        "Frostbeard",
        "Gorunn",
        "Holderhek",
        "Ironfist",
        "Loderr",
        "Lutgehr",
        "Rumnaheim",
        "Strakeln",
        "Torunn",
        "Ungart",
    ];
    const FEMALE_NAMES: &[&str] = &[
        "Amber", "Artin", "Audhild", "Bardryn", "Dagnal", "Diesa", "Eldeth", "Falkrunn",
        "Finellen", "Gunnloda", "Gurdis", "Helja", "Hlin", "Kathra", "Kristryd", "Ilde",
        "Liftrasa", "Mardred", "Riswynn", "Sannl", "Torbera", "Torgga", "Vistra",
    ];
    const MALE_NAMES: &[&str] = &[
        "Adrik", "Alberich", "Baern", "Barendd", "Brottor", "Bruenor", "Dain", "Darrak", "Delg",
        "Eberk", "Einkil", "Fargrim", "Flint", "Gardain", "Harbek", "Kildrak", "Morgran", "Orsik",
        "Oskar", "Rangrim", "Rurik", "Taklinn", "Thoradin", "Thorin", "Tordek", "Traubon",
        "Travok", "Ulfgar", "Veit", "Vondal",
    ];

    pub(crate) fn gen_name(rng: &mut impl Rng, gender: &Gender) -> String {
        let first_names = match gender {
            Gender::Female => FEMALE_NAMES,
            Gender::Male => MALE_NAMES,
        };
        format!(
            "{} {}",
            first_names.iter().choose(rng).unwrap(),
            CLAN_NAMES.iter().choose(rng).unwrap()
        )
    }
}

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum DwarfSubrace {
    Hill,
    Mountain,
}
#[derive(Deserialize, Serialize)]
pub(crate) struct Dwarf {
    subrace: DwarfSubrace,
}

#[typetag::serde]
impl Race for Dwarf {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, Characteristics) {
        let characteristics = Characteristics::gen(rng, &AGE_RANGE);
        (
            Box::new(Self {
                subrace: DwarfSubrace::iter().choose(rng).unwrap(),
            }),
            names::gen_name(rng, &characteristics.gender),
            characteristics,
        )
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
