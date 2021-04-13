use std::fmt;

use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType},
        alignment::{AlignmentInfluences, Attitude, Morality},
        attack::Resistances,
        backstory::Backstory,
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, HeightAndWeightTable,
            Size, Speed, WeightMod,
        },
        equipment::trinkets::GOBLIN_STATUS_SYMBOLS,
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{goblinoid::GOBLIN, Name},
        proficiencies::Proficiencies,
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

use super::{origins::MONSTROUS_ORIGIN, Race};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(3, 5),
    base_weight: 35,
    height_mod: RollCmd(2, Die::D4),
    weight_mod: WeightMod::Fixed(1),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Goblin {
    origin: String,
    status_symbol: String,
}

impl AlignmentInfluences for Goblin {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Neutral]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Evil]
    }
}

impl Backstory for Goblin {
    fn backstory(&self) -> Vec<String> {
        vec![
            format!("Origin: {}", self.origin),
            format!("Status Symbol: {}", self.status_symbol),
        ]
    }
}

impl Characteristics for Goblin {
    const AGE_RANGE: AgeRange = AgeRange(4..=60);
    const SIZE: Size = Size::Small;

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Goblin {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 119)])
    }
}

impl Features for Goblin {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Vgtm, 119),
            },
            // When you damage a creature with an attack or a spell and the creature’s size is larger than yours, you can cause the attack or spell to deal extra damage to the creature. The extra damage equals your level. Once you use this trait, you can’t use it again until you finish a short or long rest.
            Feature {
                title: "Fury of the Small",
                citation: Citation(Book::Vgtm, 119),
            },
            // You can take the Disengage or Hide action as a bonus action on each of your turns.
            Feature {
                title: "Nimble Escape",
                citation: Citation(Book::Vgtm, 119),
            },
        ]
    }
}

impl Languages for Goblin {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Goblin]
    }
}

impl Name for Goblin {
    fn gen_name(rng: &mut impl Rng, _: &CharacteristicDetails) -> String {
        (*GOBLIN.choose(rng).unwrap()).to_string()
    }
}

impl Proficiencies for Goblin {}

#[typetag::serde]
impl Race for Goblin {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            origin: (*MONSTROUS_ORIGIN.choose(rng).unwrap()).to_string(),
            status_symbol: (*GOBLIN_STATUS_SYMBOLS.choose(rng).unwrap()).to_string(),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            AbilityScore(AbilityScoreType::Constitution, 1),
        ]
    }
}

impl Resistances for Goblin {}

impl fmt::Display for Goblin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Goblin")
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
        let goblin = Goblin::gen(&mut rng);
        insta::assert_yaml_snapshot!(goblin);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (goblin, _name, _characteristics) = Goblin::gen(&mut rng);
        insta::assert_display_snapshot!(goblin);
    }

    #[test]
    fn test_attitude() {
        let goblin = Goblin {
            origin: String::new(),
            status_symbol: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.attitude());
    }

    #[test]
    fn test_morality() {
        let goblin = Goblin {
            origin: String::new(),
            status_symbol: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (goblin, _name, _characteristics) = Goblin::gen(&mut rng);
        insta::assert_yaml_snapshot!(goblin.backstory());
    }

    #[test]
    fn test_characteristics() {
        let goblin = Goblin {
            origin: String::new(),
            status_symbol: String::new(),
        };
        assert_eq!(goblin.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(goblin.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (goblin, _name, _characteristics) = Goblin::gen(&mut rng);
        insta::assert_yaml_snapshot!(goblin.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let goblin = Goblin {
            origin: String::new(),
            status_symbol: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let goblin = Goblin {
            origin: String::new(),
            status_symbol: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let goblin = Goblin {
            origin: String::new(),
            status_symbol: String::new(),
        };
        let characteristics = goblin.gen_characteristics(&mut rng);
        let name = Goblin::gen_name(&mut rng, &characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_abilities() {
        let goblin = Goblin {
            origin: String::new(),
            status_symbol: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.abilities());
    }
}
