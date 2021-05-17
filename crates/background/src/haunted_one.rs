use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::{Feature, Features};
use gear::{
    adventuring_gear::{Gear, OtherGear},
    currency::Coin,
};
use languages::{LanguageType, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScores, Skill},
    equipment::{Equipment, EquipmentOption, Pack, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
};

use super::Background;

const HARROWING_EVENT: &[&str] = &[
    "A monster that slaughtered dozens of innocent people spared your life, and you don’t know why.You were born under a dark star. You can feel it watching you, coldly and distantly. Sometimes it beckons you in the dead of night.",
    "An apparition that has haunted your family for generations now haunts you. You don’t know what it wants, and it won’t leave you alone.",
    "Your family has a history of practicing the dark arts. You dabbled once and felt something horrible clutch at your soul, whereupon you fled in terror.",
    "An oni took your sibling one cold, dark night, and you were unable to stop it.",
    "You were cursed with lycanthropy and later cured. You are now haunted by the innocents you slaughtered.",
    "A hag kidnapped and raised you. You escaped, but the hag still has a magical hold over you and fills your mind with evil thoughts.",
    "You opened an eldritch tome and saw things unfit for a sane mind. You burned the book, but its words and images are burned into your psyche.",
    "A fiend possessed you as a child. You were locked away but escaped. The fiend is still inside you, but now you try to keep it locked away.",
    "You did terrible things to avenge the murder of someone you loved. You became a monster, and it haunts your waking dreams.",
];
const BONDS: &[&str] = &[
    "I keep my thoughts and discoveries in a journal. My journal is my legacy.",
    "I would sacrifice my life and my soul to protect the innocent.",
    "My torment drove away the person I love. I strive to win back the love I’ve lost.",
    "A terrible guilt consumes me. I hope that I can find redemption through my actions.",
    "There’s evil in me, I can feel it. It must never be set free.",
    "I have a child to protect. I must make the world a safer place for him (or her).",
];
const FLAWS: &[&str] = &[
    "I have certain rituals that I must follow every day. I can never break them.",
    "I assume the worst in people.",
    "I feel no compassion for the dead. They’re the lucky ones.",
    "I have an addiction.",
    "I am a purveyor of doom and gloom who lives in a world without hope.",
    "I talk to spirits that no one else can see.",
];
const IDEALS: &[(&str, Influence)] = &[
    (
        "I try to help those in need, no matter what the personal cost.",
        Influence::Good,
    ),
    (
        "I’ll stop the spirits that haunt me or die trying.",
        Influence::Any,
    ),
    (
        "I kill monsters to make the world a safer place, and to exorcise my own demons.",
        Influence::Good,
    ),
    (
        "I have a dark calling that puts me above the law.",
        Influence::Chaotic,
    ),
    (
        "I like to know my enemy’s capabilities and weaknesses before rushing into battle.",
        Influence::Lawful,
    ),
    (
        "I’m a monster that destroys other monsters, and anything else that gets in my way.",
        Influence::Evil,
    ),
];
const TRAITS: &[&str] = &[
    "I don’t run from evil. Evil runs from me.",
    "I like to read and memorize poetry. It keeps me calm and brings me fleeting moments of happiness.",
    "I spend money freely and live life to the fullest, knowing that tomorrow I might die.",
    "I live for the thrill of the hunt.",
    "I don’t talk about the thing that torments me. I’d rather not burden others with my curse.",
    "I expect danger around every corner.",
    "I refuse to become a victim, and I will not allow others to be victimized.",
    "I put no trust in divine beings.",
];

const SKILLS: &[Skill] = &[
    Skill::Arcana,
    Skill::Investigation,
    Skill::Religion,
    Skill::Survival,
];

#[derive(Default, Deserialize, Serialize)]
pub struct HauntedOne {
    harrowing_event: String,
}

impl HauntedOne {
    fn gen_harrowing_event(rng: &mut impl Rng) -> String {
        String::from(*HARROWING_EVENT.choose(rng).unwrap())
    }
}

impl Background for HauntedOne {
    fn gen(rng: &mut impl Rng, _: &AbilityScores, _: &[Proficiency], _: i16) -> Self {
        Self {
            harrowing_event: Self::gen_harrowing_event(rng),
        }
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for HauntedOne {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Harrowing Event: {}", self.harrowing_event)]
    }
}

impl Citations for HauntedOne {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Cos, 209)])
    }
}

impl Features for HauntedOne {
    fn features(&self) -> Vec<Feature> {
        // Those who look into your eyes can see that you have faced unimaginable horror and that you are no stranger to darkness. Though they might fear you, commoners will extend you every courtesy and do their utmost to help you. Unless you have shown yourself to be a danger to them, they will even take up arms to fight alongside you, should you find yourself facing an enemy alone.
        vec![Feature {
            title: "Heart of Darkness",
            citation: Citation(Book::Cos, 209),
        }]
    }
}

impl Languages for HauntedOne {
    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (1, Some(LanguageType::Exotic))
    }
}

impl Pantheons for HauntedOne {}

impl PersonalityOptions for HauntedOne {
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

impl Proficiencies for HauntedOne {
    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(Some(SKILLS.to_vec()), 2)]
    }
}

impl StartingEquipment for HauntedOne {
    fn coins(&self) -> (Coin, u8) {
        Pack::MonsterHunter.coins()
    }

    fn equipment(&self) -> Vec<Equipment> {
        let mut equipment = Pack::MonsterHunter.equipment();
        equipment.push(Equipment::Gear(Gear::Other(OtherGear::ClothesCommon)));
        equipment
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        let mut addl_equipment = Pack::MonsterHunter.addl_equipment();
        addl_equipment.push(EquipmentOption::Trinket(None, None, true));
        addl_equipment
    }
}

impl fmt::Display for HauntedOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Haunted One")
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
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.traits());
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(HauntedOne::skills());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_addl_languages() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = HauntedOne::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_equipment());
    }
}
