use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        attack::DamageType,
        characteristics::{
            AgeRange, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable, Size,
        },
        equipment::{
            armor::ArmorType,
            tools::{ArtisansTools, Tool},
            weapons::WeaponType,
        },
        features::Feature,
        languages::Language,
        names::{
            dwarf::{CLAN, FEMALE, MALE},
            Name,
        },
        proficiencies::{Proficiency, WeaponProficiency},
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
const TOOL_OPTIONS: &[ArtisansTools] = &[
    ArtisansTools::BrewersSupplies,
    ArtisansTools::MasonsTools,
    ArtisansTools::SmithsTools,
];

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum DwarfSubrace {
    Hill,
    Mountain,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Dwarf {
    subrace: DwarfSubrace,
    tools: ArtisansTools,
}

impl Dwarf {
    fn base_features() -> Vec<Feature<'static>> {
        vec![
            Feature {
                title: "Ability Score Increase",
                description: "Your Constitution score increases by 2.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            },
            Feature {
                title: "Age",
                description: "Dwarves mature at the same rate as humans, but they're considered young until they reach the age of 50. On average, they live about 350 years.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            },
            Feature {
                title: "Alignment",
                description: "Most dwarves are lawful, believing firmly in the benefits of a well-ordered society. They tend toward good as well, with a strong sense of fair play and a belief that everyone deserves to share in the benefits of a just order.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            },
            Feature {
                title: "Size",
                description: "Dwarves stand between 4 and 5 feet tall and average about 150 pounds. Your size is Medium.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            },
            Feature {
                title: "Speed",
                description: "Your base walking speed is 25 feet. Your speed is not reduced by wearing heavy armor.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            },
            Feature {
                title: "Darkvision",
                description: "Accustomed to life underground, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            },
            Feature {
                title: "Dwarven Resilience",
                description: "You have advantage on saving throws against poison, and you have resistance against poison damage (explained in the \"Combat\" section).",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            },
            Feature {
                title: "Dwarven Combat Training",
                description: "You have proficiency with the battleaxe, handaxe, light hammer, and warhammer.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            },
            Feature {
                title: "Tool Proficiency",
                description: "You gain proficiency with the artisan's tools of your choice: smith's tools, brewer's supplies, or mason's tools.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            },
            Feature {
                title: "Stonecunning",
                description: "Whenever you make an Intelligence (History) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check, instead of your normal proficiency bonus.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            },
            Feature {
                title: "Languages",
                description: "You can speak, read, and write Common and Dwarvish. Dwarvish is full of hard consonants and guttural sounds, and those characteristics spill over into whatever other language a dwarf might speak.",
                citation: Citation {
                    book: Book::PHB,
                    page: 20,
                },
            },
        ]
    }

    fn subrace_features(&self) -> Vec<Feature> {
        match self.subrace {
            DwarfSubrace::Hill => vec![
                Feature {
                    title: "Ability Score Increase",
                    description: "Your Wisdom score increases by 1.",
                    citation: Citation {
                        book: Book::PHB,
                        page: 20,
                    },
                },
                Feature {
                    title: "Dwarven Toughness",
                    description: "Your hit point maximum increases by 1, and it increases by 1 every time you gain a level.",
                    citation: Citation {
                        book: Book::PHB,
                        page: 20,
                    },
                },
            ],
            DwarfSubrace::Mountain => vec![
                Feature {
                    title: "Ability Score Increase",
                    description: "Your Strength score increases by 2.",
                    citation: Citation {
                        book: Book::PHB,
                        page: 20,
                    },
                },
                Feature {
                    title: "Dwarven Armor Training",
                    description: "You have proficiency with light and medium armor.",
                    citation: Citation {
                        book: Book::PHB,
                        page: 20,
                    },
                },
            ],
        }
    }
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
            tools: *TOOL_OPTIONS.iter().choose(rng).unwrap(),
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
        let mut features = Self::base_features();
        features.extend(self.subrace_features());
        features
    }

    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Dwarvish]
    }

    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![
            Proficiency::Tool(Tool::ArtisansTools(self.tools)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Battleaxe)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Handaxe)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::LightHammer)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Warhammer)),
        ];
        if let DwarfSubrace::Mountain = self.subrace {
            proficiencies.extend(vec![
                Proficiency::Armor(ArmorType::Light),
                Proficiency::Armor(ArmorType::Medium),
            ]);
        };
        proficiencies
    }

    fn resistances(&self) -> Vec<DamageType> {
        vec![DamageType::Poison]
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
            let dwarf = Dwarf {
                subrace,
                tools: ArtisansTools::MasonsTools,
            };
            insta::assert_snapshot!(format!("{}", dwarf));
        }
    }

    #[test]
    fn test_snapshot_abilities() {
        for subrace in DwarfSubrace::iter() {
            let dwarf = Dwarf {
                subrace,
                tools: ArtisansTools::BrewersSupplies,
            };
            insta::assert_yaml_snapshot!(dwarf.abilities());
        }
    }

    #[test]
    fn test_snapshot_citations() {
        for subrace in DwarfSubrace::iter() {
            let dwarf = Dwarf {
                subrace,
                tools: ArtisansTools::MasonsTools,
            };
            insta::assert_yaml_snapshot!(dwarf.citations());
        }
    }

    #[test]
    fn test_snapshot_features() {
        for subrace in DwarfSubrace::iter() {
            let dwarf = Dwarf {
                subrace,
                tools: ArtisansTools::SmithsTools,
            };
            insta::assert_yaml_snapshot!(dwarf.features());
        }
    }
}
