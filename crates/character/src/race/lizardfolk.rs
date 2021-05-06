use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::Resistances;
use characteristics::{
    in_inches,
    names::{lizardfolk::NAMES, Name},
    AgeRange, Appearance, CharacteristicDetails, Characteristics, HeightAndWeightTable, Size,
    Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use dice_roller::{Die, RollCmd};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use trinkets::Trinkets;

use crate::{
    ability::{AbilityScore, AbilityScoreType, Skill},
    backstory::Backstory,
    features::{Feature, Features},
    languages::{Language, Languages},
    proficiencies::{Proficiencies, ProficiencyOption},
};

use super::Race;

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 9),
    base_weight: 120,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
};

const QUIRKS: &[&str] = &[
    "You hate waste and see no reason not to scavenge fallen enemies. Fingers are tasty and portable!",
    "You sleep best while mostly submerged in water.",
    "Money is meaningless to you.",
    "You think there are only two species of humanoid: lizardfolk and meat.",
    "You have learned to laugh. You use this talent in response to all emotional situations, to better fit in with your comrades.",
    "You still don't understand how metaphors work. That doesn't stop you from using them at every opportunity.",
    "You appreciate the soft humanoids who realize they need chain mail and swords to match the gifts you were born with.",
    "You enjoy eating your food while it's still wriggling.",
];

#[derive(Deserialize, Serialize)]
pub(crate) struct Lizardfolk {
    quirk: String,
}

impl AlignmentInfluences for Lizardfolk {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Neutral]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Neutral]
    }
}

impl Appearance for Lizardfolk {}

impl Backstory for Lizardfolk {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Quirk: {}", self.quirk)]
    }
}

impl Characteristics for Lizardfolk {
    const SIZE: Size = Size::Medium;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(7..=60)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30), Speed::Swimming(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Lizardfolk {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 111)])
    }
}

impl Features for Lizardfolk {
    fn features(&self) -> Vec<Feature> {
        vec![
            // Your fanged maw is a natural weapon, which you can use to make unarmed strikes. If you hit with it, you deal piercing damage equal to 1d6 + your Strength modifier, instead of the bludgeoning damage normal for an unarmed strike.
            Feature {
                title: "Bite",
                citation: Citation(Book::Vgtm, 113),
            },
            // As part of a short rest, you can harvest bone and hide from a slain beast, construct, dragon, monstrosity, or plant creature of size Small or larger to create one of the following items: a shield, a club, a javelin, or 1d4 darts or blowgun needles. To use this trait, you need a blade, such as a dagger, or appropriate artisan's tools, such as leatherworker's tools.
            Feature {
                title: "Cunning Artisan",
                citation: Citation(Book::Vgtm, 113),
            },
            // You can hold your breath for up to 15 minutes at a time.
            Feature {
                title: "Hold Breath",
                citation: Citation(Book::Vgtm, 113),
            },
            // You have tough, scaly skin. When you aren't wearing armor, your AC is 13 + your Dexterity modifier. You can use your natural armor to determine your AC if the armor you wear would leave you with a lower AC. A shield's benefits apply as normal while you use your natural armor.
            Feature {
                title: "Natural Armor",
                citation: Citation(Book::Vgtm, 113),
            },
            // In battle, you can throw yourself into a vicious feeding frenzy. As a bonus action, you can make a special attack with your bite. If the attack hits, it deals its normal damage, and you gain temporary hit points (minimum of 1) equal to your Constitution modifier, and you can't use this trait again until you finish a short or long rest.
            Feature {
                title: "Hungry Jaws",
                citation: Citation(Book::Vgtm, 113),
            },
        ]
    }
}

impl Languages for Lizardfolk {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Draconic]
    }
}

impl Name for Lizardfolk {
    fn gen_name(rng: &mut impl Rng, _: &CharacteristicDetails) -> String {
        (*NAMES.choose(rng).unwrap()).to_string()
    }
}

impl Proficiencies for Lizardfolk {
    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(
            Some(vec![
                Skill::AnimalHandling,
                Skill::Nature,
                Skill::Stealth,
                Skill::Survival,
            ]),
            2,
        )]
    }
}

#[typetag::serde]
impl Race for Lizardfolk {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            quirk: (*QUIRKS.choose(rng).unwrap()).to_string(),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Constitution, 2),
            AbilityScore(AbilityScoreType::Wisdom, 1),
        ]
    }
}

impl Resistances for Lizardfolk {}

impl Trinkets for Lizardfolk {}

impl fmt::Display for Lizardfolk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lizardfolk")
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
        let lizardfolk = Lizardfolk::gen(&mut rng);
        insta::assert_yaml_snapshot!(lizardfolk);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (lizardfolk, _name, _characteristics) = Lizardfolk::gen(&mut rng);
        insta::assert_display_snapshot!(lizardfolk);
    }

    #[test]
    fn test_attitude() {
        let lizardfolk = Lizardfolk {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(lizardfolk.attitude());
    }

    #[test]
    fn test_morality() {
        let lizardfolk = Lizardfolk {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(lizardfolk.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (lizardfolk, _name, _characteristics) = Lizardfolk::gen(&mut rng);
        insta::assert_yaml_snapshot!(lizardfolk.backstory());
    }

    #[test]
    fn test_characteristics() {
        let lizardfolk = Lizardfolk {
            quirk: String::new(),
        };
        assert_eq!(
            lizardfolk.get_base_speeds(),
            vec![Speed::Walking(30), Speed::Swimming(30)]
        );
        assert_eq!(lizardfolk.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (lizardfolk, _name, _characteristics) = Lizardfolk::gen(&mut rng);
        insta::assert_yaml_snapshot!(lizardfolk.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let lizardfolk = Lizardfolk {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(lizardfolk.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let lizardfolk = Lizardfolk {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(lizardfolk.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let lizardfolk = Lizardfolk {
            quirk: String::new(),
        };
        let characteristics = lizardfolk.gen_characteristics(&mut rng);
        let name = Lizardfolk::gen_name(&mut rng, &characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let lizardfolk = Lizardfolk {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(lizardfolk.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_abilities() {
        let lizardfolk = Lizardfolk {
            quirk: String::new(),
        };
        insta::assert_yaml_snapshot!(lizardfolk.abilities());
    }
}
