use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::Resistances;
use backstory::Backstory;
use characteristics::{
    in_inches, names::Name, AgeRange, Appearance, CharacteristicDetails, Characteristics,
    HeightAndWeightTable, Size, Speed, WeightMod,
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
use trinkets::Trinkets;

use super::{elf::Elf, Race};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(6, 2),
    base_weight: 175,
    height_mod: RollCmd(2, Die::D12),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
};

const REASON_FOR_ADVENTURING: &[&str] = &[
    "Outcast for murder",
    "Outcast for severely damaging home territory",
    "Clan slain by invading humanoids",
    "Clan slain by a dragon or demon",
    "Separated from the tribe and lost",
    "Homeland destroyed by natural disaster",
    "Personal quest ordained by omens",
    "Dispatched on a quest by tribe leaders",
];

#[derive(Deserialize, Serialize)]
pub(crate) struct Firbolg {
    reason_for_adventuring: String,
}

impl AlignmentInfluences for Firbolg {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Neutral]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Good]
    }
}

impl Appearance for Firbolg {}

impl Backstory for Firbolg {
    fn backstory(&self) -> Vec<String> {
        vec![format!(
            "Reason for adventuring: {}",
            self.reason_for_adventuring
        )]
    }
}

impl Characteristics for Firbolg {
    const SIZE: Size = Size::Medium;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(15..=500)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Firbolg {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 106)])
    }
}

impl Features for Firbolg {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can cast Detect Magic and Disguise Self with this trait, using Wisdom as your spellcasting ability for them. Once you cast either spell, you can't cast it again with this trait until you finish a short or long rest. When you use this version of disguise self, you can seem up to 3 feet shorter than normal, allowing you to more easily blend in with humans and elves.
            Feature {
                title: "Firbolg Magic",
                citation: Citation(Book::Vgtm, 107),
            },
            //  As a bonus action, you can magically turn invisible until the start of your next turn or until you attack, make a damage roll, or force someone to make a saving throw. Once you use this trait, you can't use it again until you finish a short or long rest.
            Feature {
                title: "Hidden Step",
                citation: Citation(Book::Vgtm, 107),
            },
            // You count as one size larger when determining your carrying capacity and the weight you can push, drag, or lift.
            Feature {
                title: "Powerful Build",
                citation: Citation(Book::Vgtm, 107),
            },
            // You have the ability to communicate in a limited manner with beasts and plants. They can understand the meaning of your words, though you have no special ability to understand them in return. You have advantage on all Charisma checks you make to influence them.
            Feature {
                title: "Speech of Beast and Leaf",
                citation: Citation(Book::Vgtm, 107),
            },
        ]
    }
}

impl Languages for Firbolg {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Elvish, Language::Giant]
    }
}

impl Name for Firbolg {
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        Elf::gen_name(rng, characteristics)
    }
}

impl Pantheons for Firbolg {
    fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {
        vec![(Pantheon::Elven, PantheonWeight::Likely)]
    }
}

impl Proficiencies for Firbolg {}

impl PersonalityOptions for Firbolg {}

#[typetag::serde]
impl Race for Firbolg {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            reason_for_adventuring: (*REASON_FOR_ADVENTURING.choose(rng).unwrap()).to_string(),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Wisdom, 2),
            AbilityScore(AbilityScoreType::Strength, 1),
        ]
    }
}

impl Resistances for Firbolg {}

impl Trinkets for Firbolg {}

impl fmt::Display for Firbolg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Firbolg")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use characteristics::Gender;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_snapshot() {
        let mut rng = Pcg64::seed_from_u64(1);
        let firbolg = Firbolg::gen(&mut rng);
        insta::assert_yaml_snapshot!(firbolg);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (firbolg, _name, _characteristics) = Firbolg::gen(&mut rng);
        insta::assert_display_snapshot!(firbolg);
    }

    #[test]
    fn test_attitude() {
        let firbolg = Firbolg {
            reason_for_adventuring: String::new(),
        };
        insta::assert_yaml_snapshot!(firbolg.attitude());
    }

    #[test]
    fn test_morality() {
        let firbolg = Firbolg {
            reason_for_adventuring: String::new(),
        };
        insta::assert_yaml_snapshot!(firbolg.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (firbolg, _name, _characteristics) = Firbolg::gen(&mut rng);
        insta::assert_yaml_snapshot!(firbolg.backstory());
    }

    #[test]
    fn test_characteristics() {
        let firbolg = Firbolg {
            reason_for_adventuring: String::new(),
        };
        assert_eq!(firbolg.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(firbolg.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let firbolg = Firbolg {
            reason_for_adventuring: String::new(),
        };
        insta::assert_yaml_snapshot!(firbolg.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let firbolg = Firbolg {
            reason_for_adventuring: String::new(),
        };
        insta::assert_yaml_snapshot!(firbolg.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let firbolg = Firbolg {
            reason_for_adventuring: String::new(),
        };
        insta::assert_yaml_snapshot!(firbolg.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let firbolg = Firbolg {
            reason_for_adventuring: String::new(),
        };
        let characteristics_1 = firbolg.gen_characteristics(&mut rng);
        let characteristics_2 = firbolg.gen_characteristics(&mut rng);
        let female_name = Firbolg::gen_name(
            &mut rng,
            &CharacteristicDetails {
                gender: Gender::Female,
                ..characteristics_1
            },
        );
        let male_name = Firbolg::gen_name(
            &mut rng,
            &CharacteristicDetails {
                gender: Gender::Male,
                ..characteristics_2
            },
        );
        insta::assert_yaml_snapshot!([female_name, male_name]);
    }

    #[test]
    fn test_snapshot_addl_pantheons() {
        let firbolg = Firbolg {
            reason_for_adventuring: String::new(),
        };
        insta::assert_yaml_snapshot!(firbolg.addl_pantheons());
    }

    #[test]
    fn test_snapshot_abilities() {
        let firbolg = Firbolg {
            reason_for_adventuring: String::new(),
        };
        insta::assert_yaml_snapshot!(firbolg.abilities());
    }
}
