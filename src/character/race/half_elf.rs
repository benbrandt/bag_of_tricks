use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;

use super::{elf::Elf, human::Human, Race};
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::{
            in_inches, AgeRange, Characteristics, HeightAndWeightTable, Size, WeightMod,
        },
        features::Feature,
        names::{human::Names, Name},
    },
    citation::{Book, Citation, Citations},
    dice_roller::{Die, RollCmd},
};

const AGE_RANGE: AgeRange = AgeRange(1..=180);
const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 9),
    base_weight: 110,
    height_mod: RollCmd(2, Die::D8),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct HalfElf {
    addl_increases: Vec<AbilityScore>,
}

impl Name for HalfElf {
    fn gen_name(rng: &mut impl Rng, characteristics: &Characteristics) -> String {
        let names = Names::gen_names(rng);
        let first_name = *[
            Elf::gen_first_name(rng, characteristics),
            Human::gen_first_name(rng, &names, characteristics),
        ]
        .iter()
        .choose(rng)
        .unwrap();
        let surname = *[Elf::gen_family_name(rng), Human::gen_surname(rng, &names)]
            .iter()
            .choose(rng)
            .unwrap();
        format!("{} {}", first_name, surname)
    }
}

#[typetag::serde]
impl Race for HalfElf {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, Characteristics) {
        let race = Self {
            addl_increases: AbilityScoreType::iter()
                .filter(|s| s != &AbilityScoreType::Charisma)
                .choose_multiple(rng, 2)
                .into_iter()
                .map(|t| AbilityScore(t, 1))
                .collect(),
        };
        let characteristics =
            Characteristics::gen(rng, &AGE_RANGE, Size::Medium, &HEIGHT_AND_WEIGHT);
        let name = Self::gen_name(rng, &characteristics);
        (Box::new(race), name, characteristics)
    }

    fn abilities(&self) -> AbilityScores {
        let mut abilities = vec![AbilityScore(AbilityScoreType::Charisma, 2)];
        abilities.extend(self.addl_increases.clone());
        AbilityScores(abilities)
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 38,
        }])
    }

    fn features(&self) -> Vec<Feature> {
        vec![Feature {
            title: "Ability Score Increase",
            description:
                "Your Charisma score increases by 2, and two other ability scores of your choice increase by 1.",
            citation: Citation {
                book: Book::PHB,
                page: 39,
            },
        }]
    }
}

impl fmt::Display for HalfElf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Half-Elf")
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
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (half_elf, _name, _characteristics) = HalfElf::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", half_elf));
    }

    #[test]
    fn test_snapshot_abilities() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (half_elf, _name, _characteristics) = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (half_elf, _name, _characteristics) = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (half_elf, _name, _characteristics) = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.features());
    }
}
