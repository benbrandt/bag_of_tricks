use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use features::{Feature, Features};
use gear::{
    adventuring_gear::{Gear, OtherGear},
    currency::Coin,
};
use languages::{LanguageType, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScoreType, AbilityScores, Skill},
    equipment::{Equipment, EquipmentOption, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
};
use strum::{Display, EnumIter, IntoEnumIterator};

use super::Background;

const SKILLS: &[Skill] = &[Skill::Insight];
const BONDS: &[&str] = &[
    "I would die to recover an ancient relic of my faction that was lost long ago.",
    "I will someday get revenge on the corrupt faction hierarchy who branded me a traitor.",
    "I owe my life to the faction member who took me in when my parents died.",
    "Everything I do is for the common people.",
    "I will do anything to protect the network where I served.",
    "I seek to preserve a secret text that my enemies consider heretical and seek to destroy.",
];
const FLAWS: &[&str] = &[
    "I judge others harshly, and myself even more severely.",
    "I put too much trust in those who wield power within my faction's hierarchy.",
    "My loyalty sometimes leads me to blindly trust those that profess membership in my faction.",
    "I am inflexible in my thinking.",
    "I am suspicious of strangers and expect the worst of them.",
    "Once I pick a goal, I become obsessed with it to the detriment of everything else in my life.",
];
const IDEALS: &[(&str, Influence)] = &[
    ("Tradition. The ancient traditions of membership and secrecy must be preserved and upheld.", Influence::Lawful),
    ("Charity. I always try to help those in need, no matter what the personal cost.", Influence::Good),
    ("Change. We must help bring about the changes the faction is constantly working in the world", Influence::Chaotic),
    ("Power. I hope to one day rise to the top of my faction's hierarchy.", Influence::Lawful),
    ("Faith. I trust that my faction will guide my actions. I have faith that if I work hard, things will go well.", Influence::Lawful),
    ("Aspiration. I seek to prove myself worthy of my faction's favor by matching my actions against their teachings.", Influence::Any),
];
const TRAITS: &[&str] = &[
    "I idolize a particular hero of my faction, and constantly refer to that person's deeds and example.",
    "I can find common ground between the fiercest enemies, empathizing with them and always working toward peace.",
    "I see omens in every event and action. The gods try to speak to us, we just need to listen.",
    "Nothing can shake my optimistic attitude.",
    "I quote (or misquote) faction texts and proverbs in almost every situation.",
    "I am tolerant (or intolerant) of other factions and respect (or condemn) the membership in other factions.",
    "I've enjoyed fine food, drink, and high society among my faction's elite. Rough living grates on me.",
    "I've spent so long in the faction that I have little practical experience dealing with people in the outside world.",
];

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Faction {
    #[strum(serialize = "The Harpers")]
    Harpers,
    #[strum(serialize = "The Order of the Gauntlet")]
    OrderOfTheGauntlet,
    #[strum(serialize = "The Emerald Enclave")]
    EmeraldEnclave,
    #[strum(serialize = "The Lord's Alliance")]
    LordsAlliance,
    #[strum(serialize = "The Zhentarim")]
    Zhentarim,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct FactionAgent {
    faction: Faction,
}

impl FactionAgent {
    fn addl_skills() -> Vec<Skill> {
        Skill::iter()
            .filter(|s| {
                [
                    AbilityScoreType::Charisma,
                    AbilityScoreType::Intelligence,
                    AbilityScoreType::Wisdom,
                ]
                .contains(&s.ability_score_type())
            })
            .collect::<Vec<_>>()
    }
}

#[typetag::serde]
impl Background for FactionAgent {
    fn gen(
        rng: &mut impl Rng,
        _: &AbilityScores,
        _: &[Proficiency],
        _: i16,
    ) -> Box<dyn Background> {
        Box::new(Self {
            faction: Faction::iter().choose(rng).unwrap(),
        })
    }

    fn skills() -> Vec<Skill> {
        let mut skills = SKILLS.to_vec();
        skills.extend(Self::addl_skills());
        skills
    }
}

impl Backstory for FactionAgent {}

impl Citations for FactionAgent {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 147)])
    }
}

impl Features for FactionAgent {
    fn features(&self) -> Vec<Feature> {
        // As a faction agent, you have access to a secret network of supporters and operatives who can provide assistance on your adventures. You know a set of secret signs and passwords you can use to identify such operatives, who can provide you with access to a hidden safe house, free room and board, or assistance in finding information. These agents never risk their lives for you or risk revealing their true identities.
        vec![Feature {
            title: "Safe Haven",
            citation: Citation(Book::Scag, 147),
        }]
    }
}

impl Languages for FactionAgent {
    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (2, None)
    }
}

impl PersonalityOptions for FactionAgent {
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

impl Proficiencies for FactionAgent {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(Some(Self::addl_skills()), 1)]
    }
}

impl StartingEquipment for FactionAgent {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 15)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Gear(Gear::Other(OtherGear::ClothesCommon)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![
            EquipmentOption::From(
                ["badge", "emblem"]
                    .iter()
                    .map(|i| Equipment::Other(format!("the {} of your faction", i)))
                    .collect(),
            ),
            EquipmentOption::From(
                [
                    "a copy of a seminal faction text",
                    "a code-book for a covert faction",
                ]
                .iter()
                .map(|&i| Equipment::Other(i.to_string()))
                .collect(),
            ),
        ]
    }
}

impl fmt::Display for FactionAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Faction Agent", self.faction)
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
        let background = FactionAgent::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = FactionAgent::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = FactionAgent::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = FactionAgent::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = FactionAgent::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = FactionAgent::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.traits());
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(FactionAgent::skills());
    }

    #[test]
    fn test_snapshot_citations() {
        let background = FactionAgent {
            faction: Faction::EmeraldEnclave,
        };
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let background = FactionAgent {
            faction: Faction::EmeraldEnclave,
        };
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let background = FactionAgent {
            faction: Faction::EmeraldEnclave,
        };
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let background = FactionAgent {
            faction: Faction::EmeraldEnclave,
        };
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let background = FactionAgent {
            faction: Faction::EmeraldEnclave,
        };
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let background = FactionAgent {
            faction: Faction::EmeraldEnclave,
        };
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let background = FactionAgent {
            faction: Faction::EmeraldEnclave,
        };
        insta::assert_yaml_snapshot!(background.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let background = FactionAgent {
            faction: Faction::EmeraldEnclave,
        };
        insta::assert_yaml_snapshot!(background.addl_equipment());
    }
}
