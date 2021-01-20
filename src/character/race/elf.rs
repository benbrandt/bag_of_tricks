use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores, Skill},
        characteristics::{
            AgeRange, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable, Size,
        },
        equipment::weapons::WeaponType,
        features::{Feature, Features},
        languages::Language,
        names::{
            elf::{CHILD, FAMILY, FEMALE, MALE},
            Name,
        },
        proficiencies::{Proficiency, WeaponProficiency},
    }, citation::{Book, Citation, CitationList, Citations}};

mod height_and_weight {
    use crate::{
        character::characteristics::{in_inches, HeightAndWeightTable, WeightMod},
        dice_roller::{Die, RollCmd},
    };

    pub(crate) const DARK: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 5),
        base_weight: 75,
        height_mod: RollCmd(2, Die::D6),
        weight_mod: WeightMod::Roll(RollCmd(1, Die::D6)),
    };
    pub(crate) const HIGH: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 6),
        base_weight: 90,
        height_mod: RollCmd(2, Die::D10),
        weight_mod: WeightMod::Roll(RollCmd(1, Die::D4)),
    };
    pub(crate) const WOOD: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 6),
        base_weight: 100,
        height_mod: RollCmd(2, Die::D10),
        weight_mod: WeightMod::Roll(RollCmd(1, Die::D4)),
    };
}
const BASE_LANGUAGES: &[Language] = &[Language::Common, Language::Elvish];

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum ElfSubrace {
    Dark,
    High,
    Wood,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Elf {
    extra_language: Option<Language>,
    subrace: ElfSubrace,
}

impl Elf {
    fn gen_extra_language(subrace: &ElfSubrace, rng: &mut impl Rng) -> Option<Language> {
        let languages: HashSet<_> = BASE_LANGUAGES.iter().cloned().collect();
        match subrace {
            ElfSubrace::High => Some(
                *Language::iter()
                    .collect::<HashSet<Language>>()
                    .difference(&languages)
                    .choose(rng)
                    .unwrap(),
            ),
            ElfSubrace::Dark | ElfSubrace::Wood => None,
        }
    }

    pub(crate) fn gen_first_name<'a>(
        rng: &mut impl Rng,
        CharacteristicDetails { age, gender, .. }: &CharacteristicDetails,
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

impl Characteristics for Elf {
    const AGE_RANGE: AgeRange = AgeRange(1..=750);
    const SIZE: Size = Size::Medium;

    fn get_base_speed(&self) -> u8 {
        match self.subrace {
            ElfSubrace::Wood => 35,
            _ => 30,
        }
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        match self.subrace {
            ElfSubrace::Dark => &height_and_weight::DARK,
            ElfSubrace::High => &height_and_weight::HIGH,
            ElfSubrace::Wood => &height_and_weight::WOOD,
        }
    }
}

impl Citations for Elf {
    fn citations(&self) -> CitationList {
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
        CitationList(vec![race, subrace])
    }
}

impl Features for Elf {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
            Feature {
                title: "Alignment",
                description: "Elves love freedom, variety, and self-expression, so they lean strongly toward the gentler aspects of chaos. They value and protect others' freedom as well as their own, and they are more often good than not. The drow are an exception; their exile into the Underdark has made them vicious and dangerous. Drow are more often evil than not.",
                citation: Citation {
                    book: Book::PHB,
                    page: 23,
                },
            },
            Feature {
                title: "Darkvision",
                description: "Accustomed to twilit forests and the night sky, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
                citation: Citation {
                    book: Book::PHB,
                    page: 23,
                },
            },
            Feature {
                title: "Fey Ancestry",
                description: "You have advantage on saving throws against being charmed, and magic can't put you to sleep.",
                citation: Citation {
                    book: Book::PHB,
                    page: 23,
                },
            },
            Feature {
                title: "Trance",
                description: "Elves don't need to sleep. Instead, they meditate deeply, remaining semiconscious, for 4 hours a day. (The Common word for such meditation is \"trance.\") While meditating, you can dream after a fashion; such dreams are actually mental exercises that have become reflexive through years of practice. After resting in this way, you gain the same benefit that a human does from 8 hours of sleep.",
                citation: Citation {
                    book: Book::PHB,
                    page: 23,
                },
            },
        ];
        features.extend(match self.subrace {
            ElfSubrace::Dark => vec![
                Feature {
                    title: "Superior Darkvision",
                    description: "Your darkvision has a radius of 120 feet.",
                    citation: Citation {
                        book: Book::PHB,
                        page: 24,
                    },
                },
                Feature {
                    title: "Sunlight Sensitivity",
                    description: "You have disadvantage on attack rolls and on Wisdom (Perception) checks that rely on sight when you, the target of your attack, or whatever you are trying to perceive is in direct sunlight.",
                    citation: Citation {
                        book: Book::PHB,
                        page: 24,
                    },
                },
                Feature {
                    title: "Drow Magic",
                    description: "You know the dancing lights cantrip. When you reach 3rd level, you can cast the faerie fire spell once with this trait and regain the ability to do so when you finish a long rest. When you reach 5th level, you can cast the darkness spell once with this trait and regain the ability to do so when you finish a long rest. Charisma is your spellcasting ability for these spells.",
                    citation: Citation {
                        book: Book::PHB,
                        page: 24,
                    },
                },
            ],
            ElfSubrace::High => vec![Feature {
                title: "Cantrip",
                description: "You know one cantrip of your choice from the wizard spell list. Intelligence is your spellcasting ability for it.",
                citation: Citation {
                    book: Book::PHB,
                    page: 24,
                },
            }],
            ElfSubrace::Wood => vec![Feature {
                title: "Mask of the Wild",
                description: "You can attempt to hide even when you are only lightly obscured by foliage, heavy rain, falling snow, mist, and other natural phenomena.",
                citation: Citation {
                    book: Book::PHB,
                    page: 24,
                },
            }],
        });
        features
    }
}

impl Name for Elf {
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        format!(
            "{} {}",
            Self::gen_first_name(rng, characteristics),
            Self::gen_family_name(rng),
        )
    }
}

#[typetag::serde]
impl Race for Elf {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let subrace = ElfSubrace::iter().choose(rng).unwrap();
        let race = Box::new(Self {
            extra_language: Self::gen_extra_language(&subrace, rng),
            subrace,
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
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

    fn languages(&self) -> Vec<Language> {
        let mut languages = BASE_LANGUAGES.to_vec();
        if let Some(language) = self.extra_language {
            languages.push(language);
        }
        languages
    }

    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![Proficiency::Skill(Skill::Perception)];

        proficiencies.extend(match self.subrace {
            ElfSubrace::Dark => vec![
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::CrossbowHand)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Rapier)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortsword)),
            ],
            ElfSubrace::High => vec![
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Longbow)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Longsword)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortsword)),
            ],
            ElfSubrace::Wood => vec![
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Longbow)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Longsword)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortbow)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortsword)),
            ],
        });
        proficiencies
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
            let elf = Elf {
                extra_language: Some(Language::Abyssal),
                subrace,
            };
            insta::assert_snapshot!(format!("{}", elf));
        }
    }

    #[test]
    fn test_snapshot_abilities() {
        for subrace in ElfSubrace::iter() {
            let elf = Elf {
                extra_language: None,
                subrace,
            };
            insta::assert_yaml_snapshot!(elf.abilities());
        }
    }

    #[test]
    fn test_snapshot_citations() {
        for subrace in ElfSubrace::iter() {
            let elf = Elf {
                extra_language: None,
                subrace,
            };
            insta::assert_yaml_snapshot!(elf.citations());
        }
    }

    #[test]
    fn test_snapshot_features() {
        for subrace in ElfSubrace::iter() {
            let elf = Elf {
                extra_language: None,
                subrace,
            };
            insta::assert_yaml_snapshot!(elf.features());
        }
    }
}
