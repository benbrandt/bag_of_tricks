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
        equipment::{armor::ArmorType, weapons::WeaponCategory},
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{goblinoid::HOBGOBLIN, Name},
        proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

use super::{origins::MONSTROUS_ORIGIN, Race};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 8),
    base_weight: 110,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Hobgoblin {
    origin: String,
}

impl AlignmentInfluences for Hobgoblin {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Lawful]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Evil]
    }
}

impl Backstory for Hobgoblin {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Origin: {}", self.origin)]
    }
}

impl Characteristics for Hobgoblin {
    const AGE_RANGE: AgeRange = AgeRange(10..=100);
    const SIZE: Size = Size::Medium;

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Hobgoblin {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 119)])
    }
}

impl Features for Hobgoblin {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Vgtm, 119),
            },
            // Hobgoblins are careful not to show weakness in front of their allies, for fear of losing status. If you miss with an attack roll or fail an ability check or a saving throw, you can gain a bonus to the roll equal to the number of allies you can see within 30 feet of you (maximum bonus of +5). Once you use this trait, you can’t use it again until you finish a short or long rest.
            Feature {
                title: "Saving Face",
                citation: Citation(Book::Vgtm, 119),
            },
        ]
    }
}

impl Languages for Hobgoblin {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Goblin]
    }
}

impl Name for Hobgoblin {
    fn gen_name(rng: &mut impl Rng, _: &CharacteristicDetails) -> String {
        (*HOBGOBLIN.choose(rng).unwrap()).to_string()
    }
}

impl Proficiencies for Hobgoblin {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![Proficiency::Armor(ArmorType::Light)]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Weapon(
            Some(WeaponCategory::Martial),
            None,
            2,
        )]
    }
}

#[typetag::serde]
impl Race for Hobgoblin {
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
            AbilityScore(AbilityScoreType::Constitution, 2),
            AbilityScore(AbilityScoreType::Intelligence, 1),
        ]
    }
}

impl Resistances for Hobgoblin {}

impl fmt::Display for Hobgoblin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hobgoblin")
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
        let hobgoblin = Hobgoblin::gen(&mut rng);
        insta::assert_yaml_snapshot!(hobgoblin);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (hobgoblin, _name, _characteristics) = Hobgoblin::gen(&mut rng);
        insta::assert_display_snapshot!(hobgoblin);
    }

    #[test]
    fn test_attitude() {
        let hobgoblin = Hobgoblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(hobgoblin.attitude());
    }

    #[test]
    fn test_morality() {
        let hobgoblin = Hobgoblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(hobgoblin.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (hobgoblin, _name, _characteristics) = Hobgoblin::gen(&mut rng);
        insta::assert_yaml_snapshot!(hobgoblin.backstory());
    }

    #[test]
    fn test_characteristics() {
        let hobgoblin = Hobgoblin {
            origin: String::new(),
        };
        assert_eq!(hobgoblin.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(hobgoblin.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (hobgoblin, _name, _characteristics) = Hobgoblin::gen(&mut rng);
        insta::assert_yaml_snapshot!(hobgoblin.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let hobgoblin = Hobgoblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(hobgoblin.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let hobgoblin = Hobgoblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(hobgoblin.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let hobgoblin = Hobgoblin {
            origin: String::new(),
        };
        let characteristics = hobgoblin.gen_characteristics(&mut rng);
        let name = Hobgoblin::gen_name(&mut rng, &characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let hobgoblin = Hobgoblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(hobgoblin.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let hobgoblin = Hobgoblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(hobgoblin.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_abilities() {
        let hobgoblin = Hobgoblin {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(hobgoblin.abilities());
    }
}
