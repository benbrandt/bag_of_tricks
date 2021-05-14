use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::Resistances;
use backstory::{Backstory, MONSTROUS_ORIGIN};
use characteristics::{
    in_inches,
    names::{goblinoid::BUGBEAR, Name},
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
    ability::{AbilityScore, AbilityScoreType, Skill},
    proficiencies::{Proficiencies, Proficiency},
};
use trinkets::Trinkets;

use super::Race;

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(6, 0),
    base_weight: 200,
    height_mod: RollCmd(2, Die::D12),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Bugbear {
    origin: String,
}

impl AlignmentInfluences for Bugbear {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Evil]
    }
}

impl Appearance for Bugbear {}

impl Backstory for Bugbear {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Origin: {}", self.origin)]
    }
}

impl Characteristics for Bugbear {
    fn get_age_range(&self) -> AgeRange {
        AgeRange(8..=80)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }

    fn get_size(&self) -> Size {
        Size::Medium
    }
}

impl Citations for Bugbear {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 119)])
    }
}

impl Features for Bugbear {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Vgtm, 119),
            },
            // When you make a melee attack on your turn, your reach for it is 5 feet greater than normal.
            Feature {
                title: "Long-Limbed",
                citation: Citation(Book::Vgtm, 119),
            },
            // You count as one size larger when determining your carrying capacity and the weight you can push, drag, or lift.
            Feature {
                title: "Powerful Build",
                citation: Citation(Book::Vgtm, 119),
            },
            // If you surprise a creature and hit it with an attack on your first turn in combat, the attack deals an extra 2d6 damage to it. You can use this trait only once per combat.
            Feature {
                title: "Surprise Attack",
                citation: Citation(Book::Vgtm, 119),
            },
        ]
    }
}

impl Languages for Bugbear {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Goblin]
    }
}

impl Name for Bugbear {
    fn gen_name(&self, rng: &mut impl Rng, _: &CharacteristicDetails) -> String {
        (*BUGBEAR.choose(rng).unwrap()).to_string()
    }
}

impl Pantheons for Bugbear {
    fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {
        vec![(Pantheon::Bugbear, PantheonWeight::Likely)]
    }
}

impl PersonalityOptions for Bugbear {}

impl Proficiencies for Bugbear {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![Proficiency::Skill(Skill::Stealth)]
    }
}

#[typetag::serde]
impl Race for Bugbear {
    fn gen(rng: &mut impl Rng) -> Self {
        Self {
            origin: (*MONSTROUS_ORIGIN.choose(rng).unwrap()).to_string(),
        }
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Strength, 2),
            AbilityScore(AbilityScoreType::Dexterity, 1),
        ]
    }
}

impl Resistances for Bugbear {}

impl Trinkets for Bugbear {}

impl fmt::Display for Bugbear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bugbear")
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
        let bugbear = Bugbear::gen(&mut rng);
        insta::assert_yaml_snapshot!(bugbear);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let bugbear = Bugbear::gen(&mut rng);
        insta::assert_display_snapshot!(bugbear);
    }

    #[test]
    fn test_attitude() {
        let bugbear = Bugbear {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(bugbear.attitude());
    }

    #[test]
    fn test_morality() {
        let bugbear = Bugbear {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(bugbear.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let bugbear = Bugbear::gen(&mut rng);
        insta::assert_yaml_snapshot!(bugbear.backstory());
    }

    #[test]
    fn test_characteristics() {
        let bugbear = Bugbear {
            origin: String::new(),
        };
        assert_eq!(bugbear.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(bugbear.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let bugbear = Bugbear::gen(&mut rng);
        insta::assert_yaml_snapshot!(bugbear.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let bugbear = Bugbear {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(bugbear.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let bugbear = Bugbear {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(bugbear.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let bugbear = Bugbear {
            origin: String::new(),
        };
        let characteristics = bugbear.gen_characteristics(&mut rng);
        let name = bugbear.gen_name(&mut rng, &characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_addl_pantheons() {
        let bugbear = Bugbear {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(bugbear.addl_pantheons());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let bugbear = Bugbear {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(bugbear.proficiencies());
    }

    #[test]
    fn test_snapshot_abilities() {
        let bugbear = Bugbear {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(bugbear.abilities());
    }
}
