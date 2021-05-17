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
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScores, Skill},
    equipment::{Equipment, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency},
};
use strum::{Display, EnumIter, IntoEnumIterator};

use super::Background;

const SKILLS: &[Skill] = &[Skill::Arcana, Skill::History];
pub(crate) const BONDS: &[&str] = &[
    "It is my duty to protect my students.",
    "I have an ancient text that holds terrible secrets that must not fall into the wrong hands.",
    "I work to preserve a library, university, scriptorium, or monastery.",
    "My life's work is a series of tomes related to a specific field of lore.",
    "I've been searching my whole life for the answer to a certain question.",
    "I sold my soul for knowledge. I hope to do great deeds and win it back.",
];
pub(crate) const FLAWS: &[&str] = &[
    "I am easily distracted by the promise of information.",
    "Most people scream and run when they see a demon. I stop and take notes on its anatomy.",
    "Unlocking an ancient mystery is worth the price of a civilization.",
    "I overlook obvious solutions in favor of complicated ones.",
    "I speak without really thinking through my words, invariably insulting others.",
    "I can't keep a secret to save my life, or anyone else's.",
];
pub(crate) const IDEALS: &[(&str, Influence)] = &[
    (
        "Knowledge. The path to power and self-improvement is through knowledge.",
        Influence::Neutral,
    ),
    (
        "Beauty. What is beautiful points us beyond itself toward what is true.",
        Influence::Good,
    ),
    (
        "Logic. Emotions must not cloud our logical thinking.",
        Influence::Lawful,
    ),
    (
        "No Limits. Nothing should fetter the infinite possibility inherent in all existence.",
        Influence::Chaotic,
    ),
    (
        "Power. Knowledge is the path to power and domination.",
        Influence::Evil,
    ),
    (
        "Self-Improvement. The goal of a life of study is the betterment of oneself.",
        Influence::Any,
    ),
];
pub(crate) const TRAITS: &[&str] = &[
    "I use polysyllabic words that convey the impression of great erudition.",
    "I've read every book in the world's greatest libraries \u{2014} or I like to boast that I have.",
    "I'm used to helping out those who aren't as smart as I am, and I patiently explain anything and everything to others.",
    "There's nothing I like more than a good mystery.",
    "I'm willing to listen to every side of an argument before I make my own judgment.",
    "I . . . speak . . . slowly . . . when talking . . . to idiots, . . . which . . . almost . . . everyone . . . is . . . compared . . . to me.",
    "I am horribly, horribly awkward in social situations.",
    "I'm convinced that people are always trying to steal my secrets.",
];

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Specialty {
    Alchemist,
    Astronomer,
    #[strum(serialize = "Discredited Academic")]
    DiscreditedAcademic,
    Librarian,
    Professor,
    Researcher,
    #[strum(serialize = "WizardsApprentice")]
    WizardsApprentice,
    Scribe,
}

impl Default for Specialty {
    fn default() -> Self {
        Self::Alchemist
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Sage {
    specialty: Specialty,
}

impl Background for Sage {
    fn gen(rng: &mut impl Rng, _: &AbilityScores, _: &[Proficiency], _: i16) -> Self {
        Self {
            specialty: Specialty::iter().choose(rng).unwrap(),
        }
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for Sage {}

impl Citations for Sage {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 137)])
    }
}

impl Features for Sage {
    fn features(&self) -> Vec<Feature> {
        // When you attempt to learn or recall a piece of lore, if you do not know that information, you often know where and from whom you can obtain it. Usually, this information comes from a library, scriptorium, university, or a sage or other learned person or creature. Your DM might rule that the knowledge you seek is secreted away in an almost inaccessible place, or that it simply cannot be found. Unearthing the deepest secrets of the multiverse can require an adventure or even a whole campaign.
        vec![Feature {
            title: "Researcher",
            citation: Citation(Book::Phb, 138),
        }]
    }
}

impl Languages for Sage {
    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (2, None)
    }
}

impl Pantheons for Sage {}

impl PersonalityOptions for Sage {
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

impl Proficiencies for Sage {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }
}

impl StartingEquipment for Sage {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 10)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Other("a bottle of black ink".into()),
            Equipment::Other("a quill".into()),
            Equipment::Other("a small knife".into()),
            Equipment::Other("a letter from a dead colleague posing a question you have not yet been able to answer".into()),
            Equipment::Gear(Gear::Other(OtherGear::ClothesCommon)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }
}

impl fmt::Display for Sage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sage ({})", self.specialty)
    }
}
