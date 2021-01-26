use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType},
        alignment::{Attitude, Morality},
        attack::{DamageType, Resistances},
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, WeightMod,
        },
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{
            dragonborn::{CHILD, CLAN, FEMALE, MALE},
            Name,
        },
        proficiencies::Proficiencies,
    },
    citation::{Book, Citation, CitationList, Citations},
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

    // fn breath_weapon(&self, character: &Character) -> Attack {
    //     let (save_type, range) = match self.ancestry {
    //         DraconicAncestry::Black
    //         | DraconicAncestry::Blue
    //         | DraconicAncestry::Brass
    //         | DraconicAncestry::Bronze
    //         | DraconicAncestry::Copper => (AbilityScoreType::Dexterity, "5 by 30 ft. line"),
    //         DraconicAncestry::Gold | DraconicAncestry::Red => {
    //             (AbilityScoreType::Dexterity, "15 ft. cone")
    //         }
    //         DraconicAncestry::Green | DraconicAncestry::Silver | DraconicAncestry::White => {
    //             (AbilityScoreType::Constitution, "15 ft. cone")
    //         }
    //     };
    //     Attack {
    //         citation: Citation(Book::PHB, 34),
    //         damage: Damage {
    //             damage_type: self.damage_type(),
    //             modifier: 0,
    //             roll: RollCmd(
    //                 match character.level {
    //                     0..=5 => 2,
    //                     6..=10 => 3,
    //                     11..=15 => 4,
    //                     16..=u8::MAX => 5,
    //                 },
    //                 Die::D6,
    //             ),
    //         },
    //         hit: Hit::DC(
    //             save_type,
    //             8 + character.abilities.modifier(AbilityScoreType::Constitution)
    //                 + character.proficiency_bonus(),
    //         ),
    //         name: "Breath Weapon",
    //         range,
    //     }
    // }
}

impl Characteristics for Dragonborn {
    const AGE_RANGE: AgeRange = AgeRange(1..=80);
    const SIZE: Size = Size::Medium;

    fn get_alignment_tendencies(&self) -> (Option<Attitude>, Option<Morality>) {
        (
            None,
            Some(match self.ancestry {
                DraconicAncestry::Black
                | DraconicAncestry::Blue
                | DraconicAncestry::Green
                | DraconicAncestry::Red
                | DraconicAncestry::White => Morality::Evil,
                DraconicAncestry::Brass
                | DraconicAncestry::Bronze
                | DraconicAncestry::Copper
                | DraconicAncestry::Gold
                | DraconicAncestry::Silver => Morality::Good,
            }),
        )
    }

    fn get_base_speed(&self) -> u8 {
        30
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Dragonborn {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::PHB, 32)])
    }
}

impl Features for Dragonborn {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can use your action to exhale destructive energy. Your draconic ancestry determines the size, shape, and damage type of the exhalation. When you use your breath weapon, each creature in the area of the exhalation must make a saving throw, the type of which is determined by your draconic ancestry. The DC for this saving throw equals 8 + your Constitution modifier + your proficiency bonus. A creature takes 2d6 damage on a failed save, and half as much damage on a successful one. The damage increases to 3d6 at 6th level, 4d6 at 11th level, and 5d6 at 16th level. After you use your breath weapon, you can't use it again until you complete a short or long rest.
            Feature {
                title: "Breath Weapon",
                citation: Citation(Book::PHB, 34),
            },
        ]
    }
}

impl Languages for Dragonborn {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Draconic]
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

impl Proficiencies for Dragonborn {}

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

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Charisma, 1),
            AbilityScore(AbilityScoreType::Strength, 2),
        ]
    }
}

impl Resistances for Dragonborn {
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
