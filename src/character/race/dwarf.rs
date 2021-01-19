use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::{
            AgeRange, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable, Size,
        },
        features::Feature,
        names::{
            dwarf::{CLAN, FEMALE, MALE},
            Name,
        },
    },
    citation::{Book, Citation, Citations},
};

mod height_and_weight {
    use crate::{
        character::characteristics::{in_inches, HeightAndWeightTable, WeightMod},
        dice_roller::{Die, RollCmd},
    };

    pub(crate) const HILL: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(3, 8),
        base_weight: 115,
        height_mod: RollCmd(2, Die::D4),
        weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
    };
    pub(crate) const MOUNTAIN: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 1),
        base_weight: 130,
        height_mod: RollCmd(2, Die::D4),
        weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
    };
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

impl Characteristics for Dwarf {
    const AGE_RANGE: AgeRange = AgeRange(1..=350);
    const SIZE: Size = Size::Medium;

    fn get_base_speed(&self) -> u8 {
        25
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        match self.subrace {
            DwarfSubrace::Hill => &height_and_weight::HILL,
            DwarfSubrace::Mountain => &height_and_weight::MOUNTAIN,
        }
    }
}

impl Name for Dwarf {
    fn gen_name(
        rng: &mut impl Rng,
        CharacteristicDetails { gender, .. }: &CharacteristicDetails,
    ) -> String {
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
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            subrace: DwarfSubrace::iter().choose(rng).unwrap(),
        });
        let characteristics = race.gen_characteristics(rng);
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
