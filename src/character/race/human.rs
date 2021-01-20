use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, WeightMod,
        },
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{human::Names, Name},
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 8),
    base_weight: 110,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Human {
    extra_language: Language,
}

impl Human {
    fn gen_extra_language(rng: &mut impl Rng) -> Language {
        Language::iter()
            .filter(|l| l != &Language::Common)
            .choose(rng)
            .unwrap()
    }
}

impl Human {
    pub(crate) fn gen_first_name<'a>(
        rng: &mut impl Rng,
        names: &'a Names,
        CharacteristicDetails { gender, .. }: &CharacteristicDetails,
    ) -> &'a str {
        let first_names = match gender {
            Gender::Female => names.female,
            Gender::Male => names.male,
        };
        first_names.iter().choose(rng).unwrap()
    }

    pub(crate) fn gen_surname<'a>(rng: &mut impl Rng, names: &'a Names) -> &'a str {
        names.surname.iter().choose(rng).unwrap()
    }
}

impl Characteristics for Human {
    const AGE_RANGE: AgeRange = AgeRange(1..=100);
    const SIZE: Size = Size::Medium;

    fn get_base_speed(&self) -> u8 {
        30
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Human {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation {
            book: Book::PHB,
            page: 29,
        }])
    }
}

impl Features for Human {
    fn features(&self) -> Vec<Feature> {
        vec![Feature {
            title: "Alignment",
            description: " Humans tend toward no particular alignment. The best and the worst are found among them.",
            citation: Citation {
                book: Book::PHB,
                page: 31,
            },
        }]
    }
}

impl Languages for Human {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, self.extra_language]
    }
}

impl Name for Human {
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        let names = Names::gen_names(rng);
        format!(
            "{} {}",
            Self::gen_first_name(rng, &names, characteristics),
            Self::gen_surname(rng, &names),
        )
    }
}

#[typetag::serde]
impl Race for Human {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            extra_language: Self::gen_extra_language(rng),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(
            AbilityScoreType::iter()
                .map(|t| AbilityScore(t, 1))
                .collect(),
        )
    }
}

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Human")
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
        let human = Human::gen(&mut rng);
        insta::assert_yaml_snapshot!(human);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (human, _name, _characteristics) = Human::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", human));
    }

    #[test]
    fn test_snapshot_abilities() {
        let human = Human {
            extra_language: Language::Abyssal,
        };
        insta::assert_yaml_snapshot!(human.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let human = Human {
            extra_language: Language::Celestial,
        };
        insta::assert_yaml_snapshot!(human.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let human = Human {
            extra_language: Language::DeepSpeech,
        };
        insta::assert_yaml_snapshot!(human.features());
    }
}
