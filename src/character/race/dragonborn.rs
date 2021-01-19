use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, WeightMod,
        },
        features::Feature,
        names::{
            dragonborn::{CHILD, CLAN, FEMALE, MALE},
            Name,
        },
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

#[derive(Deserialize, Serialize)]
pub(crate) struct Dragonborn;

impl Characteristics for Dragonborn {
    const AGE_RANGE: AgeRange = AgeRange(1..=80);

    const SIZE: Size = Size::Medium;

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
        let race = Box::new(Self);
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

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 32,
        }])
    }

    fn features(&self) -> Vec<Feature> {
        vec![Feature {
            title: "Ability Score Increase",
            description:
                "Your Strength score increases by 2, and your Charisma score increases by 1.",
            citation: Citation {
                book: Book::PHB,
                page: 34,
            },
        }]
    }
}

impl fmt::Display for Dragonborn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dragonborn")
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
        let dragonborn = Dragonborn;
        insta::assert_yaml_snapshot!(dragonborn.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let dragonborn = Dragonborn;
        insta::assert_yaml_snapshot!(dragonborn.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let dragonborn = Dragonborn;
        insta::assert_yaml_snapshot!(dragonborn.features());
    }
}
