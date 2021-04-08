use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType},
        alignment::{AlignmentInfluences, Attitude, Morality},
        attack::{DamageType, Resistances},
        backstory::Backstory,
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, WeightMod,
        },
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{
            halfling::{FAMILY, FEMALE, MALE},
            Name,
        },
        proficiencies::Proficiencies,
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(2, 7),
    base_weight: 35,
    height_mod: RollCmd(2, Die::D4),
    weight_mod: WeightMod::Fixed(1),
};

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum HalflingSubrace {
    Lightfoot,
    Stout,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Halfling {
    /// Randomly chosen subrace
    subrace: HalflingSubrace,
}

impl AlignmentInfluences for Halfling {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Lawful]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Good]
    }
}

impl Backstory for Halfling {}

impl Characteristics for Halfling {
    const AGE_RANGE: AgeRange = AgeRange(10..=150);
    const SIZE: Size = Size::Small;

    fn get_base_speed(&self) -> u8 {
        25
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Halfling {
    fn citations(&self) -> CitationList {
        let race = Citation(Book::Phb, 26);
        let subrace = match self.subrace {
            HalflingSubrace::Lightfoot | HalflingSubrace::Stout => Citation(Book::Phb, 28),
        };
        CitationList(vec![race, subrace])
    }
}

impl Features for Halfling {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
            // You have advantage on saving throws against being frightened.
            Feature {
                title: "Brave",
                citation: Citation(Book::Phb, 28),
            },
            // You can move through the space of any creature that is of a size larger than yours.
            Feature {
                title: "Halfling Nimbleness",
                citation: Citation(Book::Phb, 28),
            },
        ];
        features.extend(match self.subrace {
            // You can attempt to hide even when you are obscured only by a creature that is at least one size larger than you.
            HalflingSubrace::Lightfoot => vec![Feature {
                title: "Naturally Stealthy",
                citation: Citation(Book::Phb, 28),
            }],
            // You have advantage on saving throws against poison, and you have resistance against poison damage.
            HalflingSubrace::Stout => vec![Feature {
                title: "Stout Resilience",
                citation: Citation(Book::Phb, 28),
            }],
        });
        features
    }
}

impl Languages for Halfling {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Halfling]
    }
}

impl Name for Halfling {
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
            FAMILY.choose(rng).unwrap(),
        )
    }
}

impl Proficiencies for Halfling {}

#[typetag::serde]
impl Race for Halfling {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            subrace: HalflingSubrace::iter().choose(rng).unwrap(),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            match self.subrace {
                HalflingSubrace::Lightfoot => AbilityScore(AbilityScoreType::Charisma, 1),
                HalflingSubrace::Stout => AbilityScore(AbilityScoreType::Constitution, 1),
            },
        ]
    }
}

impl Resistances for Halfling {
    fn resistances(&self) -> Vec<DamageType> {
        match self.subrace {
            HalflingSubrace::Lightfoot => vec![],
            HalflingSubrace::Stout => vec![DamageType::Poison],
        }
    }
}

impl fmt::Display for Halfling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Halfling", self.subrace)
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
        let halfling = Halfling::gen(&mut rng);
        insta::assert_yaml_snapshot!(halfling);
    }

    #[test]
    fn test_snapshot_display() {
        insta::assert_snapshot!(HalflingSubrace::iter()
            .map(|subrace| format!("{}", Halfling { subrace }))
            .collect::<Vec<String>>()
            .join("\n\n"));
    }

    #[test]
    fn test_snapshot_abilities() {
        insta::assert_yaml_snapshot!(HalflingSubrace::iter()
            .map(|subrace| (Halfling { subrace }).abilities())
            .collect::<Vec<Vec<AbilityScore>>>());
    }

    #[test]
    fn test_snapshot_citations() {
        insta::assert_yaml_snapshot!(HalflingSubrace::iter()
            .map(|subrace| (Halfling { subrace }).citations())
            .collect::<Vec<CitationList>>());
    }

    #[test]
    fn test_snapshot_features() {
        insta::assert_yaml_snapshot!(HalflingSubrace::iter()
            .map(|subrace| (Halfling { subrace }).features())
            .collect::<Vec<Vec<Feature>>>());
    }
}
