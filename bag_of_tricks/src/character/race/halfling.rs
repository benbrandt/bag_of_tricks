#![allow(clippy::default_trait_access)]
use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use characteristics::{
    in_inches,
    names::{
        halfling::{FAMILY, FEMALE, MALE},
        Name,
    },
    AgeRange, Appearance, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable,
    Size, Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use dice_roller::{Die, RollCmd};
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::character::{
    ability::{AbilityScore, AbilityScoreType},
    attack::{DamageType, Resistances},
    backstory::Backstory,
    features::{Feature, Features},
    languages::{Language, Languages},
    proficiencies::Proficiencies,
};

use super::Race;

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(2, 7),
    base_weight: 35,
    height_mod: RollCmd(2, Die::D4),
    weight_mod: WeightMod::Fixed(1),
};

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum StoutVariant {
    Stout,
    Strongheart,
}

impl Default for StoutVariant {
    fn default() -> Self {
        Self::Stout
    }
}

#[derive(Debug, Deserialize, EnumIter, PartialEq, Serialize)]
enum HalflingSubrace {
    Ghostwise,
    Lightfoot,
    Stout(StoutVariant),
}

impl HalflingSubrace {
    fn gen(rng: &mut impl Rng) -> Self {
        let subrace = Self::iter().choose(rng).unwrap();
        match subrace {
            Self::Stout(_) => Self::Stout(StoutVariant::iter().choose(rng).unwrap()),
            Self::Ghostwise | Self::Lightfoot => subrace,
        }
    }
}

impl fmt::Display for HalflingSubrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ghostwise => write!(f, "Ghostwise"),
            Self::Lightfoot => write!(f, "Lightfoot"),
            Self::Stout(v) => write!(f, "{}", v),
        }
    }
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

impl Appearance for Halfling {}

impl Backstory for Halfling {}

impl Characteristics for Halfling {
    const SIZE: Size = Size::Small;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=150)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(25)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Halfling {
    fn citations(&self) -> CitationList {
        let race = Citation(Book::Phb, 26);
        let subrace = match self.subrace {
            HalflingSubrace::Ghostwise => Citation(Book::Scag, 110),
            HalflingSubrace::Lightfoot | HalflingSubrace::Stout(StoutVariant::Stout) => {
                Citation(Book::Phb, 28)
            }
            HalflingSubrace::Stout(StoutVariant::Strongheart) => Citation(Book::Scag, 109),
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
        features.push(match self.subrace {
            // You can speak telepathically to any creature within 30 feet of you. The creature understands you only if the two of you share a language. You can speak telepathically in this way to one creature at a time.
            HalflingSubrace::Ghostwise => Feature {
                title: "Silent Speech",
                citation: Citation(Book::Scag, 110),
            },
            // You can attempt to hide even when you are obscured only by a creature that is at least one size larger than you.
            HalflingSubrace::Lightfoot => Feature {
                title: "Naturally Stealthy",
                citation: Citation(Book::Phb, 28),
            },
            // You have advantage on saving throws against poison, and you have resistance against poison damage.
            HalflingSubrace::Stout(_) => Feature {
                title: "Stout Resilience",
                citation: Citation(Book::Phb, 28),
            },
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
            subrace: HalflingSubrace::gen(rng),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            match self.subrace {
                HalflingSubrace::Ghostwise => AbilityScore(AbilityScoreType::Wisdom, 1),
                HalflingSubrace::Lightfoot => AbilityScore(AbilityScoreType::Charisma, 1),
                HalflingSubrace::Stout(_) => AbilityScore(AbilityScoreType::Constitution, 1),
            },
        ]
    }
}

impl Resistances for Halfling {
    fn resistances(&self) -> Vec<DamageType> {
        match self.subrace {
            HalflingSubrace::Ghostwise | HalflingSubrace::Lightfoot => vec![],
            HalflingSubrace::Stout(_) => vec![DamageType::Poison],
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
