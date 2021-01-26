use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    character::{
        ability::Skill,
        features::{Feature, Features},
        languages::Languages,
        proficiencies::{Proficiencies, Proficiency},
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{Background, Personality, PersonalityOptions};

#[derive(Deserialize, Serialize)]
pub(crate) struct Acolyte;

#[typetag::serde]
impl Background for Acolyte {
    fn gen(rng: &mut impl rand::Rng) -> (Box<dyn Background>, Personality) {
        (Box::new(Self), Self::gen_personality(rng))
    }

    fn equipment(&self) -> String {
        String::from("A holy symbol (a gift to you when you entered the priesthood), a prayer book or prayer wheel, 5 sticks of incense, vestments, a set of common clothes, and a pouch containing 15 gp")
    }
}

impl Citations for Acolyte {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::PHB, 127)])
    }
}

impl Features for Acolyte {
    fn features(&self) -> Vec<Feature> {
        vec![Feature {
            title: "Shelter of the Faithful",
            citation: Citation(Book::PHB, 127),
        }]
    }
}

impl Languages for Acolyte {
    fn addl_languages(&self) -> usize {
        2
    }
}

impl PersonalityOptions for Acolyte {
    const BONDS: [&'static str; 6] = [
        "I would die to recover an ancient relic of my faith that was lost long ago.",
        "I will someday get revenge on the corrupt temple hierarchy who branded me a heretic.",
        "I owe my life to the priest who took me in when my parents died.",
        "Everything I do is for the common people.",
        "I will do anything to protect the temple where I served.",
        "I seek to preserve a sacred text that my enemies consider heretical and seek to destroy.",
    ];
    const FLAWS: [&'static str; 6] = [
        "I judge others harshly, and myself even more severely.",
        "I put too much trust in those who wield power within my temple's hierarchy.",
        "My piety sometimes leads me to blindly trust those that profess faith in my god.",
        "I am inflexible in my thinking.",
        "I am suspicious of strangers and expect the worst of them.",
        "Once I pick a goal, I become obsessed with it to the detriment of everything else in my life.",
    ];
    const IDEALS: [&'static str; 6] = [
        "Tradition. The ancient traditions of worship and sacrifice must be preserved and upheld. (Lawful)",
        "Charity. I always try to help those in need, no matter what the personal cost. (Good)",
        "Change. We must help bring about the changes the gods are constantly working in the world. (Chaotic)",
        "Power. I hope to one day rise to the top of my faith's religious hierarchy. (Lawful)",
        "Faith. I trust that my deity will guide my actions. I have faith that if I work hard, things will go well. (Lawful)",
        "Aspiration. I seek to prove myself worthy of my god;s favor by matching my actions against his or her teachings. (Any)",
    ];
    const TRAITS: [&'static str; 8] = [
        "I idolize a particular hero of my faith, and constantly refer to that person's deeds and example.",
        "I can find common ground between the fiercest enemies, empathizing with them and always working toward peace.",
        "I see omens in every event and action. The gods try to speak to us, we just need to listen.",
        "Nothing can shake my optimistic attitude.",
        "I quote (or misquote) sacred texts and proverbs in almost every situation.",
        "I am tolerant (or intolerant) of other faiths and respect (or condemn) the worship of other gods.",
        "I've enjoyed fine food, drink, and high society among my temple's elite. Rough living grates on me.",
        "I've spent so long in the temple that I have little practical experience dealing with people in the outside world.",
    ];
}

impl Proficiencies for Acolyte {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Skill(Skill::Insight),
            Proficiency::Skill(Skill::Religion),
        ]
    }
}

impl fmt::Display for Acolyte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Acolyte")
    }
}
