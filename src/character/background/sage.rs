use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    character::{
        ability::Skill,
        backstory::Backstory,
        features::{Feature, Features},
        languages::Languages,
        proficiencies::{Proficiencies, Proficiency},
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{Background, Influence, Personality, PersonalityOptions};

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

#[derive(Deserialize, Serialize)]
pub(crate) struct Sage {
    specialty: Specialty,
}

#[typetag::serde]
impl Background for Sage {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Background>, Personality) {
        let background = Box::new(Self {
            specialty: Specialty::iter().choose(rng).unwrap(),
        });
        (background, Self::gen_personality(rng))
    }

    fn equipment(&self) -> String {
        String::from("A bottle of black ink, a quill, a small knife, a letter from a dead colleague posing a question you have not yet been able to answer, a set of common clothes, and a pouch containing 10 gp")
    }
}

impl Backstory for Sage {}

impl Citations for Sage {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::PHB, 137)])
    }
}

impl Features for Sage {
    fn features(&self) -> Vec<Feature> {
        // When you attempt to learn or recall a piece of lore, if you do not know that information, you often know where and from whom you can obtain it. Usually, this information comes from a library, scriptorium, university, or a sage or other learned person or creature. Your DM might rule that the knowledge you seek is secreted away in an almost inaccessible place, or that it simply cannot be found. Unearthing the deepest secrets of the multiverse can require an adventure or even a whole campaign.
        vec![Feature {
            title: "Researcher",
            citation: Citation(Book::PHB, 138),
        }]
    }
}

impl Languages for Sage {
    fn addl_languages(&self) -> usize {
        2
    }
}

impl PersonalityOptions for Sage {
    const BONDS: [&'static str; 6] = [
        "It is my duty to protect my students.",
        "I have an ancient text that holds terrible secrets that must not fall into the wrong hands.",
        "I work to preserve a library, university, scriptorium, or monastery.",
        "My life's work is a series of tomes related to a specific field of lore.",
        "I've been searching my whole life for the answer to a certain question.",
        "I sold my soul for knowledge. I hope to do great deeds and win it back.",
    ];
    const FLAWS: [&'static str; 6] = [
        "I am easily distracted by the promise of information.",
        "Most people scream and run when they see a demon. I stop and take notes on its anatomy.",
        "Unlocking an ancient mystery is worth the price of a civilization.",
        "I overlook obvious solutions in favor of complicated ones.",
        "I speak without really thinking through my words, invariably insulting others.",
        "I can't keep a secret to save my life, or anyone else's.",
    ];
    const IDEALS: [(&'static str, Influence); 6] = [
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
    const TRAITS: [&'static str; 8] = [
        "I use polysyllabic words that convey the impression of great erudition.",
        "I've read every book in the world's greatest libraries \u{2014} or I like to boast that I have.",
        "I'm used to helping out those who aren't as smart as I am, and I patiently explain anything and everything to others.",
        "There's nothing I like more than a good mystery.",
        "I'm willing to listen to every side of an argument before I make my own judgment.",
        "I . . . speak . . . slowly . . . when talking . . . to idiots, . . . which . . . almost . . . everyone . . . is . . . compared . . . to me.",
        "I am horribly, horribly awkward in social situations.",
        "I'm convinced that people are always trying to steal my secrets.",
    ];
}

impl Proficiencies for Sage {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Skill(Skill::Arcana),
            Proficiency::Skill(Skill::History),
        ]
    }
}

impl fmt::Display for Sage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sage ({})", self.specialty)
    }
}
