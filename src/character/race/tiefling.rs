use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{human::Human, Race};
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        attack::DamageType,
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, WeightMod,
        },
        features::Feature,
        languages::Language,
        names::{
            human::Names,
            tiefling::{FEMALE_ABYSSAL, MALE_ABYSSAL, VIRTUE_NAMES},
            Name,
        },
    },
    citation::{Book, Citation, Citations},
    dice_roller::{Die, RollCmd},
};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 9),
    base_weight: 110,
    height_mod: RollCmd(2, Die::D8),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Tiefling;

impl Tiefling {
    fn gen_first_name<'a>(
        rng: &mut impl Rng,
        names: &'a Names,
        characteristics: &CharacteristicDetails,
    ) -> &'a str {
        let abyssal_name = *(match characteristics.gender {
            Gender::Female => FEMALE_ABYSSAL,
            Gender::Male => MALE_ABYSSAL,
        })
        .iter()
        .choose(rng)
        .unwrap();
        let human_name = Human::gen_first_name(rng, names, characteristics);
        let virtue_name = *VIRTUE_NAMES.iter().choose(rng).unwrap();
        *[abyssal_name, human_name, virtue_name]
            .iter()
            .choose(rng)
            .unwrap()
    }
}

impl Characteristics for Tiefling {
    const AGE_RANGE: AgeRange = AgeRange(1..=100);
    const SIZE: Size = Size::Medium;

    fn get_base_speed(&self) -> u8 {
        30
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Name for Tiefling {
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        let names = Names::gen_names(rng);
        format!(
            "{} {}",
            Self::gen_first_name(rng, &names, characteristics),
            Human::gen_surname(rng, &names)
        )
    }
}

#[typetag::serde]
impl Race for Tiefling {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self);
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Charisma, 2),
            AbilityScore(AbilityScoreType::Intelligence, 1),
        ])
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 42,
        }])
    }

    fn features(&self) -> Vec<Feature> {
        vec![
            Feature {
                title: "Alignment",
                description: "Tieflings might not have an innate tendency toward evil, but many of them end up there. Evil or not, an independent nature inclines many tieflings toward a chaotic alignment.",
                citation: Citation {
                    book: Book::PHB,
                    page: 43,
                },
            },
            Feature {
                title: "Darkvision",
                description: "Thanks to your infernal heritage, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
                citation: Citation {
                    book: Book::PHB,
                    page: 43,
                },
            },
            Feature {
                title: "Infernal Legacy",
                description: "You know the thaumaturgy cantrip. When you reach 3rd level, you can cast the hellish rebuke spell as a 2nd-level spell once with this trait and regain the ability to do so when you finish a long rest. When you reach 5th level, you can cast the darkness spell once with this trait and regain the ability to do so when you finish a long rest. Charisma is your spellcasting ability for these spells.",
                citation: Citation {
                    book: Book::PHB,
                    page: 43,
                },
            },
        ]
    }

    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Infernal]
    }

    fn resistances(&self) -> Vec<DamageType> {
        vec![DamageType::Fire]
    }
}

impl fmt::Display for Tiefling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tiefling")
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
        let tiefling = Tiefling::gen(&mut rng);
        insta::assert_yaml_snapshot!(tiefling);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (tiefling, _name, _characteristics) = Tiefling::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", tiefling));
    }

    #[test]
    fn test_snapshot_abilities() {
        let tiefling = Tiefling;
        insta::assert_yaml_snapshot!(tiefling.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let tiefling = Tiefling;
        insta::assert_yaml_snapshot!(tiefling.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let tiefling = Tiefling;
        insta::assert_yaml_snapshot!(tiefling.features());
    }
}
