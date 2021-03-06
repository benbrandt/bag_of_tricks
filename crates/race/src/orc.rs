use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::Resistances;
use backstory::{Backstory, MONSTROUS_ORIGIN};
use characteristics::{
    in_inches,
    names::{
        orc::{EPITHET, FEMALE, MALE},
        Name,
    },
    AgeRange, Appearance, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable,
    Size, Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use deities::{Pantheon, PantheonWeight, Pantheons};
use dice_roller::{Die, RollCmd};
use features::{Feature, Features};
use languages::{Language, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScore, AbilityScoreType, Skill},
    proficiencies::{Proficiencies, Proficiency},
};
use trinkets::Trinkets;

use super::Race;

pub(crate) const BONDS: &[&str] = &[
    "I will defend my tribe to the death.",
    "Every serious choice I make must be decided by signs or omens from the gods.",
    "I carry the teeth of a great warrior. They inspire me to commit great deeds in battle.",
    "To avenge Gruumsh, I will kill every elf I see.",
    "I will seek and destroy those who murdered my tribe.",
    "I owe my survival to a non-orc.",
];
pub(crate) const FLAWS: &[&str] = &[
    "I have a calm temperament and let insults roll off my back.",
    "I don't fear the gods and have no patience for superstitions.",
    "I am slow to anger, but when I do become enraged I fight until my enemies are dead, no matter the cost.",
    "I understand the value of civilization and the order that society brings.",
    "I don't trust anyone.",
    "I believe in living to fight another day.",
];
pub(crate) const IDEALS: &[(&str, Influence)] = &[
    (
        "Strength. Showing superior strength brings honor to Gruumsh.",
        Influence::Any,
    ),
    (
        "Prowess. Killing all your enemies is the path to greatness.",
        Influence::Evil,
    ),
    (
        "Dominance. I will have achieved glory when all cower before my might.",
        Influence::Evil,
    ),
    (
        "Intimidation. I can get what I want from weaklings that fear me.",
        Influence::Evil,
    ),
    (
        "Glory. The goals of the tribe don't concern me. Personal glory is what I crave.",
        Influence::Chaotic,
    ),
    ("Savagery. I will not be controlled.", Influence::Chaotic),
];
pub(crate) const TRAITS: &[&str] = &[
    "I never relinquish my weapon.",
    "I welcome any chance to prove my battle skills.",
    "I always appear like I am about to kill everyone around me.",
    "I love a good brawl.",
    "I drink the blood of monsters to consume their power.",
    "I chant orcish war dirges during combat.",
];

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(5, 4),
    base_weight: 175,
    height_mod: RollCmd(2, Die::D8),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
};

#[derive(Default, Deserialize, Serialize)]
pub struct Orc {
    origin: String,
}

impl Orc {
    pub(crate) fn gen_first_name<'a>(
        rng: &mut impl Rng,
        CharacteristicDetails { gender, .. }: &CharacteristicDetails,
    ) -> &'a str {
        let first_names = match gender {
            Gender::Female => FEMALE,
            Gender::Male => MALE,
        };
        first_names.choose(rng).unwrap()
    }

    pub(crate) fn gen_epithet<'a>(rng: &mut impl Rng) -> &'a str {
        EPITHET.choose(rng).unwrap()
    }
}

impl AlignmentInfluences for Orc {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Evil]
    }
}

impl Appearance for Orc {}

impl Backstory for Orc {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Origin: {}", self.origin)]
    }
}

impl Characteristics for Orc {
    fn get_age_range(&self) -> AgeRange {
        AgeRange(6..=50)
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

impl Citations for Orc {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 120)])
    }
}

impl Features for Orc {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Vgtm, 120),
            },
            // As a bonus action, you can move up to your speed toward an enemy of your choice that you can see or hear. You must end this move closer to the enemy than you started.
            Feature {
                title: "Aggresive",
                citation: Citation(Book::Vgtm, 120),
            },
            // You count as one size larger when determining your carrying capacity and the weight you can push, drag, or lift.
            Feature {
                title: "Powerful Build",
                citation: Citation(Book::Vgtm, 120),
            },
        ]
    }
}

impl Languages for Orc {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Orc]
    }
}

impl Name for Orc {
    fn gen_name(&self, rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        format!(
            "{} {}",
            Self::gen_first_name(rng, characteristics),
            Self::gen_epithet(rng)
        )
    }
}

impl Pantheons for Orc {
    fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {
        vec![(Pantheon::Orc, PantheonWeight::Likely)]
    }
}

impl PersonalityOptions for Orc {
    fn bonds(&self) -> Vec<String> {
        BONDS.iter().map(|&s| s.to_string()).collect()
    }

    fn flaws(&self) -> Vec<String> {
        FLAWS.iter().map(|&s| s.to_string()).collect()
    }

    fn ideals(&self) -> Vec<(String, Influence)> {
        IDEALS.iter().map(|&(s, i)| (s.to_string(), i)).collect()
    }

    fn traits(&self) -> Vec<String> {
        TRAITS.iter().map(|&s| s.to_string()).collect()
    }
}

impl Proficiencies for Orc {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![Proficiency::Skill(Skill::Intimidation)]
    }
}

impl Race for Orc {
    fn gen(rng: &mut impl Rng) -> Self {
        Self {
            origin: (*MONSTROUS_ORIGIN.choose(rng).unwrap()).to_string(),
        }
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Strength, 2),
            AbilityScore(AbilityScoreType::Constitution, 1),
            AbilityScore(AbilityScoreType::Intelligence, -2),
        ]
    }
}

impl Resistances for Orc {}

impl Trinkets for Orc {}

impl fmt::Display for Orc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Orc")
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
        let orc = Orc::gen(&mut rng);
        insta::assert_yaml_snapshot!(orc);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let orc = Orc::gen(&mut rng);
        insta::assert_display_snapshot!(orc);
    }

    #[test]
    fn test_attitude() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.attitude());
    }

    #[test]
    fn test_morality() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let orc = Orc::gen(&mut rng);
        insta::assert_yaml_snapshot!(orc.backstory());
    }

    #[test]
    fn test_characteristics() {
        let orc = Orc {
            origin: String::new(),
        };
        assert_eq!(orc.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(orc.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let orc = Orc::gen(&mut rng);
        insta::assert_yaml_snapshot!(orc.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let orc = Orc {
            origin: String::new(),
        };
        let characteristics = orc.gen_characteristics(&mut rng);
        let name = orc.gen_name(&mut rng, &characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_addl_pantheons() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.addl_pantheons());
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let orc = Orc::gen(&mut rng);
        insta::assert_yaml_snapshot!(orc.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let orc = Orc::gen(&mut rng);
        insta::assert_yaml_snapshot!(orc.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let orc = Orc::gen(&mut rng);
        insta::assert_yaml_snapshot!(orc.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let orc = Orc::gen(&mut rng);
        insta::assert_yaml_snapshot!(orc.traits());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.proficiencies());
    }

    #[test]
    fn test_snapshot_abilities() {
        let orc = Orc {
            origin: String::new(),
        };
        insta::assert_yaml_snapshot!(orc.abilities());
    }
}
