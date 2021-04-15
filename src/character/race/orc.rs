use std::fmt;

use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    alignment::{AlignmentInfluences, Attitude, Morality},
    character::{
        ability::{AbilityScore, AbilityScoreType, Skill},
        attack::Resistances,
        backstory::Backstory,
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, Speed, WeightMod,
        },
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{
            orc::{EPITHET, FEMALE, MALE},
            Name,
        },
        proficiencies::{Proficiencies, Proficiency},
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

use super::{origins::MONSTROUS_ORIGIN, Race};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(5, 4),
    base_weight: 175,
    height_mod: RollCmd(2, Die::D8),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Orc {
    origin: String,
}

impl Orc {
    pub(crate) fn gen_first_name<'a>(
        rng: &mut impl Rng,
        CharacteristicDetails { gender, .. }: &CharacteristicDetails,
    ) -> &'a str {
        let first_names = match gender {
            Gender::Female => FEMALE,
            Gender::Male => MALE,
        };
        first_names.choose(rng).unwrap()
    }

    pub(crate) fn gen_epithet<'a>(rng: &mut impl Rng) -> &'a str {
        EPITHET.choose(rng).unwrap()
    }
}

impl AlignmentInfluences for Orc {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Evil]
    }
}

impl Backstory for Orc {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Origin: {}", self.origin)]
    }
}

impl Characteristics for Orc {
    const AGE_RANGE: AgeRange = AgeRange(6..=50);
    const SIZE: Size = Size::Medium;

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Orc {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 120)])
    }
}

impl Features for Orc {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Vgtm, 120),
            },
            // As a bonus action, you can move up to your speed toward an enemy of your choice that you can see or hear. You must end this move closer to the enemy than you started.
            Feature {
                title: "Aggresive",
                citation: Citation(Book::Vgtm, 120),
            },
            // You count as one size larger when determining your carrying capacity and the weight you can push, drag, or lift.
            Feature {
                title: "Powerful Build",
                citation: Citation(Book::Vgtm, 120),
            },
        ]
    }
}

impl Languages for Orc {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Orc]
    }
}

impl Name for Orc {
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        format!(
            "{} {}",
            Self::gen_first_name(rng, characteristics),
            Self::gen_epithet(rng)
        )
    }
}

impl Proficiencies for Orc {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![Proficiency::Skill(Skill::Intimidation)]
    }
}

#[typetag::serde]
impl Race for Orc {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            origin: (*MONSTROUS_ORIGIN.choose(rng).unwrap()).to_string(),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Strength, 2),
            AbilityScore(AbilityScoreType::Constitution, 1),
            AbilityScore(AbilityScoreType::Intelligence, -2),
        ]
    }
}

impl Resistances for Orc {}

impl fmt::Display for Orc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Orc")
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
        let orc = Orc::gen(&mut rng);
        insta::assert_yaml_snapshot!(orc);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (orc, _name, _characteristics) = Orc::gen(&mut rng);
        insta::assert_display_snapshot!(orc);
    }

    #[test]
    fn test_attitude() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.attitude());
    }

    #[test]
    fn test_morality() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (orc, _name, _characteristics) = Orc::gen(&mut rng);
        insta::assert_yaml_snapshot!(orc.backstory());
    }

    #[test]
    fn test_characteristics() {
        let orc = Orc {
            origin: String::new(),
        };
        assert_eq!(orc.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(orc.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (orc, _name, _characteristics) = Orc::gen(&mut rng);
        insta::assert_yaml_snapshot!(orc.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let orc = Orc {
            origin: String::new(),
        };
        let characteristics = orc.gen_characteristics(&mut rng);
        let name = Orc::gen_name(&mut rng, &characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.proficiencies());
    }

    #[test]
    fn test_snapshot_abilities() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.abilities());
    }
}
