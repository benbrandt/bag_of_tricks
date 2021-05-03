#![allow(clippy::default_trait_access)]
use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use characteristics::{
    names::{
        elf::{CHILD, FAMILY, FEMALE, MALE},
        Name,
    },
    AgeRange, Appearance, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable,
    Size, Speed,
};
use citation::{Book, Citation, CitationList, Citations};
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::character::{
    ability::{AbilityScore, AbilityScoreType, Skill},
    attack::Resistances,
    backstory::Backstory,
    equipment::weapons::WeaponType,
    features::{Feature, Features},
    languages::{Language, Languages},
    proficiencies::{Proficiencies, Proficiency, WeaponProficiency},
};

use super::Race;

mod height_and_weight {
    use characteristics::{in_inches, HeightAndWeightTable, WeightMod};
    use dice_roller::{Die, RollCmd};

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

#[derive(Deserialize, Display, EnumIter, Serialize)]
pub(crate) enum HighVariant {
    High,
    Moon,
    Sun,
}

impl Default for HighVariant {
    fn default() -> Self {
        Self::High
    }
}

#[derive(Deserialize, EnumIter, Serialize)]
pub(crate) enum ElfSubrace {
    Dark,
    High(HighVariant),
    Wood,
}

impl ElfSubrace {
    pub(crate) fn gen(rng: &mut impl Rng) -> Self {
        let subrace = Self::iter().choose(rng).unwrap();
        match subrace {
            Self::High(_) => Self::High(HighVariant::iter().choose(rng).unwrap()),
            Self::Dark | Self::Wood => subrace,
        }
    }
}

impl fmt::Display for ElfSubrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dark => write!(f, "Dark"),
            Self::High(v) => write!(f, "{}", v),
            Self::Wood => write!(f, "Wood"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Elf {
    /// Rnadomly chosen subrace
    subrace: ElfSubrace,
}

impl Elf {
    /// Before the age of 100, elves go by their child name
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
        first_names.choose(rng).unwrap()
    }

    pub(crate) fn gen_family_name<'a>(rng: &mut impl Rng) -> &'a str {
        FAMILY.choose(rng).unwrap()
    }

    pub(crate) fn weapon_training(subrace: &ElfSubrace) -> Vec<Proficiency> {
        match subrace {
            ElfSubrace::Dark => vec![
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::CrossbowHand)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Rapier)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortsword)),
            ],
            ElfSubrace::High(_) => vec![
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
        }
    }
}

impl AlignmentInfluences for Elf {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![match self.subrace {
            ElfSubrace::Dark => Morality::Evil,
            ElfSubrace::High(_) | ElfSubrace::Wood => Morality::Good,
        }]
    }
}

impl Appearance for Elf {}

impl Backstory for Elf {}

impl Characteristics for Elf {
    const SIZE: Size = Size::Medium;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=750)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![match self.subrace {
            ElfSubrace::Dark | ElfSubrace::High(_) => Speed::Walking(30),
            ElfSubrace::Wood => Speed::Walking(35),
        }]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        match self.subrace {
            ElfSubrace::Dark => &height_and_weight::DARK,
            ElfSubrace::High(_) => &height_and_weight::HIGH,
            ElfSubrace::Wood => &height_and_weight::WOOD,
        }
    }
}

impl Citations for Elf {
    fn citations(&self) -> CitationList {
        let race = Citation(Book::Phb, 21);
        let subrace = match self.subrace {
            ElfSubrace::Dark | ElfSubrace::Wood => Citation(Book::Phb, 24),
            ElfSubrace::High(HighVariant::High) => Citation(Book::Phb, 23),
            ElfSubrace::High(HighVariant::Moon) => Citation(Book::Scag, 105),
            ElfSubrace::High(HighVariant::Sun) => Citation(Book::Scag, 106),
        };
        CitationList(vec![race, subrace])
    }
}

impl Features for Elf {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
            // Accustomed to twilit forests and the night sky, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Phb, 23),
            },
            // You have advantage on saving throws against being charmed, and magic can't put you to sleep.
            Feature {
                title: "Fey Ancestry",
                citation: Citation(Book::Phb, 23),
            },
            // Elves don't need to sleep. Instead, they meditate deeply, remaining semiconscious, for 4 hours a day. (The Common word for such meditation is \"trance.\") While meditating, you can dream after a fashion; such dreams are actually mental exercises that have become reflexive through years of practice. After resting in this way, you gain the same benefit that a human does from 8 hours of sleep.
            Feature {
                title: "Trance",
                citation: Citation(Book::Phb, 23),
            },
        ];
        features.extend(match self.subrace {
            ElfSubrace::Dark => vec![
                // Your darkvision has a radius of 120 feet.
                Feature {
                    title: "Superior Darkvision",
                    citation: Citation(Book::Phb, 24),
                },
                // You have disadvantage on attack rolls and on Wisdom (Perception) checks that rely on sight when you, the target of your attack, or whatever you are trying to perceive is in direct sunlight.
                Feature {
                    title: "Sunlight Sensitivity",
                    citation: Citation(Book::Phb, 24),
                },
                // You know the dancing lights cantrip. When you reach 3rd level, you can cast the faerie fire spell once with this trait and regain the ability to do so when you finish a long rest. When you reach 5th level, you can cast the darkness spell once with this trait and regain the ability to do so when you finish a long rest. Charisma is your spellcasting ability for these spells.
                Feature {
                    title: "Drow Magic",
                    citation: Citation(Book::Phb, 24),
                },
            ],
            // You know one cantrip of your choice from the wizard spell list. Intelligence is your spellcasting ability for it.
            ElfSubrace::High(_) => vec![Feature {
                title: "Cantrip",
                citation: Citation(Book::Phb, 24),
            }],
            // You can attempt to hide even when you are only lightly obscured by foliage, heavy rain, falling snow, mist, and other natural phenomena.
            ElfSubrace::Wood => vec![Feature {
                title: "Mask of the Wild",
                citation: Citation(Book::Phb, 24),
            }],
        });
        features
    }
}

impl Languages for Elf {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Elvish]
    }

    fn addl_languages(&self) -> usize {
        match self.subrace {
            ElfSubrace::High(_) => 1,
            ElfSubrace::Dark | ElfSubrace::Wood => 0,
        }
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
impl Proficiencies for Elf {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![Proficiency::Skill(Skill::Perception)];
        proficiencies.extend(Self::weapon_training(&self.subrace));
        proficiencies
    }
}

#[typetag::serde]
impl Race for Elf {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let subrace = ElfSubrace::gen(rng);
        let race = Box::new(Self { subrace });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            match self.subrace {
                ElfSubrace::Dark => AbilityScore(AbilityScoreType::Charisma, 1),
                ElfSubrace::High(_) => AbilityScore(AbilityScoreType::Intelligence, 1),
                ElfSubrace::Wood => AbilityScore(AbilityScoreType::Wisdom, 1),
            },
        ]
    }
}

impl Resistances for Elf {}

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
        insta::assert_snapshot!(ElfSubrace::iter()
            .map(|subrace| format!("{}", Elf { subrace }))
            .collect::<Vec<String>>()
            .join("\n\n"));
    }

    #[test]
    fn test_snapshot_abilities() {
        insta::assert_yaml_snapshot!(ElfSubrace::iter()
            .map(|subrace| (Elf { subrace }).abilities())
            .collect::<Vec<Vec<AbilityScore>>>());
    }

    #[test]
    fn test_snapshot_citations() {
        insta::assert_yaml_snapshot!(ElfSubrace::iter()
            .map(|subrace| (Elf { subrace }).citations())
            .collect::<Vec<CitationList>>());
    }

    #[test]
    fn test_snapshot_features() {
        insta::assert_yaml_snapshot!(ElfSubrace::iter()
            .map(|subrace| (Elf { subrace }).features())
            .collect::<Vec<Vec<Feature>>>());
    }
}
