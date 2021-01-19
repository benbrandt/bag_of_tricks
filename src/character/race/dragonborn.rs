use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        attack::{Attack, Damage, DamageType, Hit},
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, WeightMod,
        },
        features::Feature,
        names::{
            dragonborn::{CHILD, CLAN, FEMALE, MALE},
            Name,
        },
        Character,
    },
    citation::{Book, Citation, Citations},
    dice_roller::{Die, RollCmd},
};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(5, 6),
    base_weight: 175,
    height_mod: RollCmd(2, Die::D8),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
};

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum DraconicAncestry {
    Black,
    Blue,
    Brass,
    Bronze,
    Copper,
    Gold,
    Green,
    Red,
    Silver,
    White,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Dragonborn {
    ancestry: DraconicAncestry,
}

impl Dragonborn {
    fn damage_type(&self) -> DamageType {
        match self.ancestry {
            DraconicAncestry::Black | DraconicAncestry::Copper => DamageType::Acid,
            DraconicAncestry::Blue | DraconicAncestry::Bronze => DamageType::Lightning,
            DraconicAncestry::Brass | DraconicAncestry::Gold | DraconicAncestry::Red => {
                DamageType::Fire
            }
            DraconicAncestry::Green => DamageType::Poison,
            DraconicAncestry::Silver | DraconicAncestry::White => DamageType::Cold,
        }
    }

    fn breath_weapon(&self, character: &Character) -> Attack {
        let (save_type, range) = match self.ancestry {
            DraconicAncestry::Black
            | DraconicAncestry::Blue
            | DraconicAncestry::Brass
            | DraconicAncestry::Bronze
            | DraconicAncestry::Copper => (AbilityScoreType::Dexterity, "5 by 30 ft. line"),
            DraconicAncestry::Gold | DraconicAncestry::Red => {
                (AbilityScoreType::Dexterity, "15 ft. cone")
            }
            DraconicAncestry::Green | DraconicAncestry::Silver | DraconicAncestry::White => {
                (AbilityScoreType::Constitution, "15 ft. cone")
            }
        };
        Attack {
            citation: Citation {
                book: Book::PHB,
                page: 34,
            },
            damage: Damage {
                damage_type: self.damage_type(),
                modifier: 0,
                roll: RollCmd(
                    match character.level {
                        0..=5 => 2,
                        6..=10 => 3,
                        11..=15 => 4,
                        16..=u8::MAX => 5,
                    },
                    Die::D6,
                ),
            },
            hit: Hit::DC(
                save_type,
                8 + character.abilities.modifier(AbilityScoreType::Constitution)
                    + character.proficiency_bonus(),
            ),
            name: "Breath Weapon",
            range,
        }
    }
}

impl Characteristics for Dragonborn {
    const AGE_RANGE: AgeRange = AgeRange(1..=80);
    const SIZE: Size = Size::Medium;

    fn get_base_speed(&self) -> u8 {
        30
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Name for Dragonborn {
    fn gen_name(
        rng: &mut impl Rng,
        CharacteristicDetails { gender, .. }: &CharacteristicDetails,
    ) -> String {
        let first_names = match gender {
            Gender::Female => FEMALE,
            Gender::Male => MALE,
        };
        format!(
            "{} {} \"{}\"",
            CLAN.iter().choose(rng).unwrap(),
            first_names.iter().choose(rng).unwrap(),
            CHILD.iter().choose(rng).unwrap(),
        )
    }
}

#[typetag::serde]
impl Race for Dragonborn {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            ancestry: DraconicAncestry::iter().choose(rng).unwrap(),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Charisma, 1),
            AbilityScore(AbilityScoreType::Strength, 2),
        ])
    }

    fn attacks(&self, character: &Character) -> Vec<Attack> {
        vec![self.breath_weapon(character)]
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 32,
        }])
    }

    fn features(&self) -> Vec<Feature> {
        let ability = Feature {
            title: "Ability Score Increase",
            description:
                "Your Strength score increases by 2, and your Charisma score increases by 1.",
            citation: Citation {
                book: Book::PHB,
                page: 34,
            },
        };
        let age = Feature {
            title: "Age",
            description: "Young dragonborn grow quickly. They walk hours after hatching, attain the size and development of a 10-year-old human child by the age of 3, and reach adulthood by 15. They live to be around 80.",
            citation: Citation {
                book: Book::PHB,
                page: 34,
            },
        };
        let alignment = Feature {
            title: "Alignment",
            description: "Dragonborn tend to extremes, making a conscious choice for one side or the other in the cosmic war between good and evil (represented by Bahamut and Tiamat, respectively). Most dragonborn are good, but those who side with Tiamat can be terrible villains.",
            citation: Citation {
                book: Book::PHB,
                page: 34,
            },
        };
        let size = Feature {
            title: "Size",
            description: "Dragonborn are taller and heavier than humans, standing well over 6 feet tall and averaging almost 250 pounds. Your size is Medium.",
            citation: Citation {
                book: Book::PHB,
                page: 34,
            },
        };
        let speed = Feature {
            title: "Speed",
            description: "Your base walking speed is 30 feet.",
            citation: Citation {
                book: Book::PHB,
                page: 34,
            },
        };
        let breath_weapon = Feature {
            title: "Breath Weapon",
            description: "You can use your action to exhale destructive energy. Your draconic ancestry determines the size, shape, and damage type of the exhalation. When you use your breath weapon, each creature in the area of the exhalation must make a saving throw, the type of which is determined by your draconic ancestry. The DC for this saving throw equals 8 + your Constitution modifier + your proficiency bonus. A creature takes 2d6 damage on a failed save, and half as much damage on a successful one. The damage increases to 3d6 at 6th level, 4d6 at 11th level, and 5d6 at 16th level. After you use your breath weapon, you can't use it again until you complete a short or long rest.",
            citation: Citation {
                book: Book::PHB,
                page: 34,
            },
        };
        let damage_resistance = Feature {
            title: "Damage Resistance",
            description:
                "You have resistance to the damage type associated with your draconic ancestry.",
            citation: Citation {
                book: Book::PHB,
                page: 34,
            },
        };
        let languages = Feature {
            title: "Languages",
            description: "You can speak, read, and write Common and Draconic. Draconic is thought to be one of the oldest languages and is often used in the study of magic. The language sounds harsh to most other creatures and includes numerous hard consonants and sibilants.",
            citation: Citation {
                book: Book::PHB,
                page: 34,
            },
        };
        vec![
            ability,
            age,
            alignment,
            size,
            speed,
            breath_weapon,
            damage_resistance,
            languages,
        ]
    }

    fn resistances(&self) -> Vec<DamageType> {
        vec![self.damage_type()]
    }
}

impl fmt::Display for Dragonborn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Dragonborn", self.ancestry)
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
        let dragonborn = Dragonborn::gen(&mut rng);
        insta::assert_yaml_snapshot!(dragonborn);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (dragonborn, _name, _characteristics) = Dragonborn::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", dragonborn));
    }

    #[test]
    fn test_snapshot_abilities() {
        let dragonborn = Dragonborn {
            ancestry: DraconicAncestry::Black,
        };
        insta::assert_yaml_snapshot!(dragonborn.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let dragonborn = Dragonborn {
            ancestry: DraconicAncestry::Black,
        };
        insta::assert_yaml_snapshot!(dragonborn.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let dragonborn = Dragonborn {
            ancestry: DraconicAncestry::Black,
        };
        insta::assert_yaml_snapshot!(dragonborn.features());
    }
}
