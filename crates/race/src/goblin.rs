use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::Resistances;
use backstory::{Backstory, MONSTROUS_ORIGIN};
use characteristics::{
    in_inches,
    names::{goblinoid::GOBLIN, Name},
    AgeRange, Appearance, CharacteristicDetails, Characteristics, HeightAndWeightTable, Size,
    Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use deities::{Pantheon, PantheonWeight, Pantheons};
use dice_roller::{Die, RollCmd};
use features::{Feature, Features};
use languages::{Language, Languages};
use personality::PersonalityOptions;
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScore, AbilityScoreType},
    proficiencies::Proficiencies,
};
use trinkets::{TrinketOption, Trinkets};

use super::Race;

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(3, 5),
    base_weight: 35,
    height_mod: RollCmd(2, Die::D4),
    weight_mod: WeightMod::Fixed(1),
};

#[derive(Default, Deserialize, Serialize)]
pub struct Goblin {
    origin: String,
}

impl AlignmentInfluences for Goblin {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Neutral]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Evil]
    }
}

impl Appearance for Goblin {}

impl Backstory for Goblin {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Origin: {}", self.origin)]
    }
}

impl Characteristics for Goblin {
    fn get_age_range(&self) -> AgeRange {
        AgeRange(4..=60)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }

    fn get_size(&self) -> Size {
        Size::Small
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
    fn gen_name(&self, rng: &mut impl Rng, _: &CharacteristicDetails) -> String {
        (*GOBLIN.choose(rng).unwrap()).to_string()
    }
}

impl Pantheons for Goblin {
    fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {
        vec![(Pantheon::Goblin, PantheonWeight::Likely)]
    }
}

impl PersonalityOptions for Goblin {}

impl Proficiencies for Goblin {}

impl Race for Goblin {
    fn gen(rng: &mut impl Rng) -> Self {
        Self {
            origin: (*MONSTROUS_ORIGIN.choose(rng).unwrap()).to_string(),
        }
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            AbilityScore(AbilityScoreType::Constitution, 1),
        ]
    }
}

impl Resistances for Goblin {}

impl Trinkets for Goblin {
    fn trinket_options(&self) -> Vec<TrinketOption> {
        vec![TrinketOption::Goblin]
    }
}

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
        let goblin = Goblin::gen(&mut rng);
        insta::assert_display_snapshot!(goblin);
    }

    #[test]
    fn test_attitude() {
        let goblin = Goblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.attitude());
    }

    #[test]
    fn test_morality() {
        let goblin = Goblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let goblin = Goblin::gen(&mut rng);
        insta::assert_yaml_snapshot!(goblin.backstory());
    }

    #[test]
    fn test_characteristics() {
        let goblin = Goblin {
            origin: String::new(),
        };
        assert_eq!(goblin.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(goblin.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let goblin = Goblin::gen(&mut rng);
        insta::assert_yaml_snapshot!(goblin.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let goblin = Goblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let goblin = Goblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let goblin = Goblin {
            origin: String::new(),
        };
        let characteristics = goblin.gen_characteristics(&mut rng);
        let name = goblin.gen_name(&mut rng, &characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_addl_pantheons() {
        let goblin = Goblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.addl_pantheons());
    }

    #[test]
    fn test_snapshot_abilities() {
        let goblin = Goblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.abilities());
    }

    #[test]
    fn test_snapshot_trinket_options() {
        let goblin = Goblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(goblin.trinket_options());
    }
}
