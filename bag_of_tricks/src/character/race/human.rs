use std::fmt;

use alignment::AlignmentInfluences;
use dice_roller::{Die, RollCmd};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType},
        attack::Resistances,
        backstory::Backstory,
        characteristics::{
            in_inches, AgeRange, Appearance, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, Speed, WeightMod,
        },
        features::Features,
        languages::{Language, Languages},
        names::Name,
        proficiencies::Proficiencies,
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::Race;

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 8),
    base_weight: 110,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Human;

impl Human {
    /// Separate function to make it easier to share with other races
    pub(crate) fn gen_first_name(
        rng: &mut impl Rng,
        CharacteristicDetails {
            ethnicity, gender, ..
        }: &CharacteristicDetails,
    ) -> &'static str {
        // Should be set if using this
        let names = ethnicity.unwrap().names();
        let first_names = match gender {
            Gender::Female => names.female,
            Gender::Male => names.male,
        };
        first_names.choose(rng).unwrap()
    }

    /// Separate function to make it easier to share with other races
    pub(crate) fn gen_surname(
        rng: &mut impl Rng,
        CharacteristicDetails { ethnicity, .. }: &CharacteristicDetails,
    ) -> &'static str {
        // Should be set if using this
        let names = ethnicity.unwrap().names();
        names.surname.choose(rng).unwrap_or(&"")
    }
}

impl AlignmentInfluences for Human {}

impl Appearance for Human {}

impl Backstory for Human {}

impl Characteristics for Human {
    const HUMAN_ANCESTRY: bool = true;
    const SIZE: Size = Size::Medium;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=100)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Human {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 29)])
    }
}

impl Features for Human {}

impl Languages for Human {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common]
    }

    fn addl_languages(&self) -> usize {
        1
    }
}

impl Name for Human {
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        format!(
            "{} {}",
            Self::gen_first_name(rng, characteristics),
            Self::gen_surname(rng, characteristics),
        )
    }
}

impl Proficiencies for Human {}

#[typetag::serde]
impl Race for Human {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self);
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        AbilityScoreType::iter()
            .map(|t| AbilityScore(t, 1))
            .collect()
    }
}

impl Resistances for Human {}

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
        let human = Human;
        insta::assert_yaml_snapshot!(human.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let human = Human;
        insta::assert_yaml_snapshot!(human.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let human = Human;
        insta::assert_yaml_snapshot!(human.features());
    }
}
