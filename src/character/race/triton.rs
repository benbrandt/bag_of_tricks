use std::fmt;

use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType},
        alignment::{AlignmentInfluences, Attitude, Morality},
        attack::{DamageType, Resistances},
        backstory::Backstory,
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, Speed, WeightMod,
        },
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{
            triton::{FEMALE, MALE, SURNAMES},
            Name,
        },
        proficiencies::Proficiencies,
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

use super::Race;

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 6),
    base_weight: 90,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

const QUIRKS: &[&str] = &[
    "You phrase requests as orders that you expect to be obeyed.",
    "You are quick to boast of the greatness of your civilization.",
    "You learned an antiquated version of Common and drop \"thee\" and \"thou\" into your speech.",
    "You assume that people are telling you the truth about local customs and expectations.",
    "The surface world is a wondrous place, and you catalog all its details in a journal.",
    "You mistakenly assume that surface folk know about and are impressed by your people's history.",
];

#[derive(Deserialize, Serialize)]
pub(crate) struct Triton {
    quirk: String,
}

impl AlignmentInfluences for Triton {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Lawful]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Good]
    }
}

impl Backstory for Triton {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Quirk: {}", self.quirk)]
    }
}

impl Characteristics for Triton {
    const AGE_RANGE: AgeRange = AgeRange(8..=200);
    const SIZE: Size = Size::Medium;

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30), Speed::Swimming(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Triton {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 115)])
    }
}

impl Features for Triton {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can breathe air and water.
            Feature {
                title: "Amphibious",
                citation: Citation(Book::Vgtm, 118),
            },
            // A child of the sea, you can call on the magic of elemental air and water. You can cast fog cloud with this trait. Starting at 3rd level, you can cast gust of wind with it, and starting at 5th level, you can also cast wall of water with it. Once you cast a spell with this trait, you can’t cast that spell with it again until you finish a long rest. Charisma is your spellcasting ability for these spells.
            Feature {
                title: "Control Air and Water",
                citation: Citation(Book::Vgtm, 118),
            },
            // Aquatic beasts have an extraordinary affinity with your people. You can communicate simple ideas with beasts that can breathe water. They can understand the meaning of your words, though you have no special ability to understand them in return.
            Feature {
                title: "Emissary of the Sea",
                citation: Citation(Book::Vgtm, 118),
            },
            // Adapted to even the most extreme ocean depths, you have resistance to cold damage, and you ignore any of the drawbacks caused by a deep, underwater environment.
            Feature {
                title: "Guardians of the Depths",
                citation: Citation(Book::Vgtm, 118),
            },
        ]
    }
}

impl Languages for Triton {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Primordial]
    }
}

impl Name for Triton {
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
            first_names.choose(rng).unwrap(),
            SURNAMES.choose(rng).unwrap(),
        )
    }
}

impl Proficiencies for Triton {}

#[typetag::serde]
impl Race for Triton {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            quirk: (*QUIRKS.choose(rng).unwrap()).to_string(),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Charisma, 1),
            AbilityScore(AbilityScoreType::Constitution, 1),
            AbilityScore(AbilityScoreType::Strength, 1),
        ]
    }
}

impl Resistances for Triton {
    fn resistances(&self) -> Vec<DamageType> {
        vec![DamageType::Cold]
    }
}

impl fmt::Display for Triton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Triton")
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
        let triton = Triton::gen(&mut rng);
        insta::assert_yaml_snapshot!(triton);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (triton, _name, _characteristics) = Triton::gen(&mut rng);
        insta::assert_display_snapshot!(triton);
    }

    #[test]
    fn test_attitude() {
        let triton = Triton {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(triton.attitude());
    }

    #[test]
    fn test_morality() {
        let triton = Triton {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(triton.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (triton, _name, _characteristics) = Triton::gen(&mut rng);
        insta::assert_yaml_snapshot!(triton.backstory());
    }

    #[test]
    fn test_characteristics() {
        let triton = Triton {
            quirk: String::new(),
        };
        assert_eq!(
            triton.get_base_speeds(),
            vec![Speed::Walking(30), Speed::Swimming(30)]
        );
        assert_eq!(triton.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (triton, _name, _characteristics) = Triton::gen(&mut rng);
        insta::assert_yaml_snapshot!(triton.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let triton = Triton {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(triton.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let triton = Triton {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(triton.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let triton = Triton {
            quirk: String::new(),
        };
        let characteristics_1 = triton.gen_characteristics(&mut rng);
        let characteristics_2 = triton.gen_characteristics(&mut rng);
        let female_name = Triton::gen_name(
            &mut rng,
            &CharacteristicDetails {
                gender: Gender::Female,
                ..characteristics_1
            },
        );
        let male_name = Triton::gen_name(
            &mut rng,
            &CharacteristicDetails {
                gender: Gender::Male,
                ..characteristics_2
            },
        );
        insta::assert_yaml_snapshot!([female_name, male_name]);
    }

    #[test]
    fn test_snapshot_abilities() {
        let triton = Triton {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(triton.abilities());
    }

    #[test]
    fn test_snapshot_resistances() {
        let triton = Triton {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(triton.resistances());
    }
}
