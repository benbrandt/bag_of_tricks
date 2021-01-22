use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType},
        attack::{DamageType, Resistances},
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, WeightMod,
        },
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{
            halfling::{FAMILY, FEMALE, MALE},
            Name,
        },
        proficiencies::Proficiencies,
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(2, 7),
    base_weight: 35,
    height_mod: RollCmd(2, Die::D4),
    weight_mod: WeightMod::Fixed(1),
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

impl Characteristics for Halfling {
    const AGE_RANGE: AgeRange = AgeRange(1..=150);
    const SIZE: Size = Size::Small;

    fn get_base_speed(&self) -> u8 {
        25
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Halfling {
    fn citations(&self) -> CitationList {
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
        CitationList(vec![race, subrace])
    }
}

impl Features for Halfling {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
            Feature {
                title: "Alignment",
                description: "Most halflings are lawful good. As a rule, they are good-hearted and kind, hate to see others in pain, and have no tolerance for oppression. They are also very orderly and traditional, leaning heavily on the support of their community and the comfort of their old ways.",
                citation: Citation {
                    book: Book::PHB,
                    page: 28,
                },
            },
            Feature {
                title: "Brave",
                description: "You have advantage on saving throws against being frightened.",
                citation: Citation {
                    book: Book::PHB,
                    page: 28,
                },
            },
            Feature {
                title: "Halfling Nimbleness",
                description: "You can move through the space of any creature that is of a size larger than yours.",
                citation: Citation {
                    book: Book::PHB,
                    page: 28,
                },
            },
        ];
        features.extend(match self.subrace {
            HalflingSubrace::Lightfoot => vec![Feature {
                title: "Naturally Stealthy",
                description: "You can attempt to hide even when you are obscured only by a creature that is at least one size larger than you.",
                citation: Citation {
                    book: Book::PHB,
                    page: 28,
                },
            }],
            HalflingSubrace::Stout => vec![Feature {
                title: "Stout Resilience",
                description: "You have advantage on saving throws against poison, and you have resistance against poison damage.",
                citation: Citation {
                    book: Book::PHB,
                    page: 28,
                },
            }],
        });
        features
    }
}

impl Languages for Halfling {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Halfling]
    }
}

impl Name for Halfling {
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
            FAMILY.iter().choose(rng).unwrap(),
        )
    }
}

impl Proficiencies for Halfling {}

#[typetag::serde]
impl Race for Halfling {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            subrace: HalflingSubrace::iter().choose(rng).unwrap(),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            match self.subrace {
                HalflingSubrace::Lightfoot => AbilityScore(AbilityScoreType::Charisma, 1),
                HalflingSubrace::Stout => AbilityScore(AbilityScoreType::Constitution, 1),
            },
        ]
    }
}

impl Resistances for Halfling {
    fn resistances(&self) -> Vec<DamageType> {
        vec![DamageType::Poison]
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
