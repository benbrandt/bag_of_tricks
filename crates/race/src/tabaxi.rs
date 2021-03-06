use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::Resistances;
use backstory::Backstory;
use characteristics::{
    in_inches,
    names::{
        tabaxi::{CLANS, NAMES},
        Name,
    },
    AgeRange, Appearance, CharacteristicDetails, Characteristics, HeightAndWeightTable, Size,
    Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use dice_roller::{Die, RollCmd};
use features::{Feature, Features};
use languages::{Language, LanguageType, Languages};
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
    base_height: in_inches(4, 10),
    base_weight: 90,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

const OBSESSIONS: &[&str] = &[
    "a god or planar entity",
    "a monster",
    "a lost civilization",
    "a wizard's secrets",
    "a mundane item",
    "a magic item",
    "a location",
    "a legend or tale",
];

const QUIRKS: &[&str] = &[
    "You miss your tropical home and complain endlessly about the freezing weather, even in summer.",
    "You never wear the same outfit twice, unless you absolutely must.",
    "You have a minor phobia of water and hate getting wet.",
    "Your tail always betrays your inner thoughts.",
    "You purr loudly when you are happy.",
    "You keep a small ball of yarn in your hand, which you constantly fidget with.",
    "You are always in debt, since you spend your gold on lavish parties and gifts for friends.",
    "When talking about something you're obsessed with, you speak quickly and never pause and other's can't understand you.",
    "You are a font of random trivia from the lore and stories you have discovered.",
    "You can't help but pocket interesting objects you come across.",
];

#[derive(Default, Deserialize, Serialize)]
pub struct Tabaxi {
    obsession: String,
    quirk: String,
}

impl AlignmentInfluences for Tabaxi {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Good, Morality::Neutral]
    }
}

impl Appearance for Tabaxi {}

impl Backstory for Tabaxi {
    fn backstory(&self) -> Vec<String> {
        vec![
            format!("My curiousity is currently fixed on {}", self.obsession),
            format!("Quirk: {}", self.quirk),
        ]
    }
}

impl Characteristics for Tabaxi {
    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=100)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30), Speed::Climbing(20)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }

    fn get_size(&self) -> Size {
        Size::Medium
    }
}

impl Citations for Tabaxi {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 113)])
    }
}

impl Features for Tabaxi {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You have a cat's keen senses, especially in the dark. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Vgtm, 115),
            },
            // Your reflexes and agility allow you to move with a burst of speed. When you move on your tum in combat, you can double your speed until the end of the tum. Once you use this trait, you can't use it again until you move 0 feet on one of your turns.
            Feature {
                title: "Feline Agility",
                citation: Citation(Book::Vgtm, 115),
            },
            // Because of your claws, you have a climbing speed of 20 feet. In addition, your claws are natural weapons, which you can use to make unarmed strikes. If you hit with them, you deal slashing damage equal to 1d4 + your Strength modifier, instead of the bludgeoning damage normal for an unarmed strike.
            Feature {
                title: "Cat's Claws",
                citation: Citation(Book::Vgtm, 115),
            },
        ]
    }
}

impl Languages for Tabaxi {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common]
    }

    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (1, None)
    }
}

impl Name for Tabaxi {
    fn gen_name(&self, rng: &mut impl Rng, _: &CharacteristicDetails) -> String {
        format!(
            "{} {}",
            NAMES.choose(rng).unwrap(),
            CLANS.choose(rng).unwrap(),
        )
    }
}

impl Pantheons for Tabaxi {}

impl PersonalityOptions for Tabaxi {}

impl Proficiencies for Tabaxi {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Skill(Skill::Perception),
            Proficiency::Skill(Skill::Stealth),
        ]
    }
}

impl Race for Tabaxi {
    fn gen(rng: &mut impl Rng) -> Self {
        Self {
            obsession: (*OBSESSIONS.choose(rng).unwrap()).to_string(),
            quirk: (*QUIRKS.choose(rng).unwrap()).to_string(),
        }
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            AbilityScore(AbilityScoreType::Charisma, 1),
        ]
    }
}

impl Resistances for Tabaxi {}

impl Trinkets for Tabaxi {}

impl fmt::Display for Tabaxi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tabaxi")
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
        let tabaxi = Tabaxi::gen(&mut rng);
        insta::assert_yaml_snapshot!(tabaxi);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let tabaxi = Tabaxi::gen(&mut rng);
        insta::assert_display_snapshot!(tabaxi);
    }

    #[test]
    fn test_attitude() {
        let tabaxi = Tabaxi {
            obsession: String::new(),
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(tabaxi.attitude());
    }

    #[test]
    fn test_morality() {
        let tabaxi = Tabaxi {
            obsession: String::new(),
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(tabaxi.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let tabaxi = Tabaxi::gen(&mut rng);
        insta::assert_yaml_snapshot!(tabaxi.backstory());
    }

    #[test]
    fn test_characteristics() {
        let tabaxi = Tabaxi {
            obsession: String::new(),
            quirk: String::new(),
        };
        assert_eq!(
            tabaxi.get_base_speeds(),
            vec![Speed::Walking(30), Speed::Climbing(20)]
        );
        assert_eq!(tabaxi.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let tabaxi = Tabaxi::gen(&mut rng);
        insta::assert_yaml_snapshot!(tabaxi.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let tabaxi = Tabaxi {
            obsession: String::new(),
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(tabaxi.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let tabaxi = Tabaxi {
            obsession: String::new(),
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(tabaxi.languages());
        assert_eq!(tabaxi.addl_languages(), (1, None));
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let tabaxi = Tabaxi {
            obsession: String::new(),
            quirk: String::new(),
        };
        let characteristics = tabaxi.gen_characteristics(&mut rng);
        let name = tabaxi.gen_name(&mut rng, &characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let tabaxi = Tabaxi {
            obsession: String::new(),
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(tabaxi.proficiencies());
    }

    #[test]
    fn test_snapshot_abilities() {
        let tabaxi = Tabaxi {
            obsession: String::new(),
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(tabaxi.abilities());
    }
}
