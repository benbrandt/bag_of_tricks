use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    character::{
        ability::Skill,
        backstory::Backstory,
        equipment::tools::Tool,
        features::{Feature, Features},
        languages::Languages,
        proficiencies::{Proficiencies, Proficiency},
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{Background, Influence, Personality, PersonalityOptions};

const LIFE_OF_SECLUSION: &[&str] = &[
    "I was searching for spiritual enlightenment.",
    "I was partaking of communal living in accordance with the dictates of a religious order.",
    "I was exiled for a crime I didn't commit.",
    "I retreated from society after a life-altering event.",
    "I needed a quiet place to work on my art, literature, music, or manifesto.",
    "I needed to commune with nature, far from civilization.",
    "I was the caretaker of an ancient ruin or relic.",
    "I was a pilgrim in search of a person, place, or relic of spiritual significance.",
];

#[derive(Deserialize, Serialize)]
pub(crate) struct Hermit {
    life_of_seclusion: String,
}

impl Hermit {
    fn gen_life_of_seclusion(rng: &mut impl Rng) -> String {
        String::from(*LIFE_OF_SECLUSION.iter().choose(rng).unwrap())
    }
}

#[typetag::serde]
impl Background for Hermit {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Background>, Personality) {
        let background = Box::new(Self {
            life_of_seclusion: Self::gen_life_of_seclusion(rng),
        });
        (background, Self::gen_personality(rng))
    }

    fn equipment(&self) -> String {
        String::from("A scroll case stuffed full of notes from your studies or prayers, a winter blanket, a set of common clothes, an herbalism kit, and 5 gp")
    }
}

impl Backstory for Hermit {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Life of Seclusion: {}", self.life_of_seclusion)]
    }
}

impl Citations for Hermit {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::PHB, 134)])
    }
}

impl Features for Hermit {
    fn features(&self) -> Vec<Feature> {
        // The quiet seclusion of your extended hermitage gave you access to a unique and powerful discovery. The exact nature of this revelation depends on the nature of your seclusion. It might be a great truth about the cosmos, the deities, the powerful beings of the outer planes, or the forces of nature. It could be a site that no one else has ever seen. You might have uncovered a fact that has long been forgotten, or unearthed some relic of the past that could rewrite history. It might be information that would be damaging to the people who consigned you to exile, and hence the reason for your return to society.
        // Work with your DM to determine the details of your discovery and its impact on the campaign.
        vec![Feature {
            title: "Discovery",
            citation: Citation(Book::PHB, 134),
        }]
    }
}

impl Languages for Hermit {
    fn addl_languages(&self) -> usize {
        1
    }
}

impl PersonalityOptions for Hermit {
    const BONDS: [&'static str; 6] = [
        "Nothing is more important than the other members of my hermitage, order, or association.",
        "I entered seclusion to hide from the ones who might still be hunting me. I must someday confront them.",
        "I'm still seeking the enlightenment I pursued in my seclusion, and it still eludes me.",
        "I entered seclusion because I loved someone I could not have.",
        "Should my discovery come to light, it could bring ruin to the world.",
        "My isolation gave me great insight into a great evil that only I can destroy.",
    ];
    const FLAWS: [&'static str; 6] = [
        "Now that I've returned to the world, I enjoy its delights a little too much.",
        "I harbor dark, bloodthirsty thoughts that my isolation and meditation failed to quell.",
        "I am dogmatic in my thoughts and philosophy.",
        "I let my need to win arguments overshadow friendships and harmony.",
        "I'd risk too much to uncover a lost bit of knowledge.",
        "I like keeping secrets and won't share them with anyone.",
    ];
    const IDEALS: [(&'static str, Influence); 6] = [
        ("Greater Good. My gifts are meant to be shared with all, not used for my own benefit.", Influence::Good),
        ("Logic. Emotions must not cloud our sense of what is right and true, or our logical thinking.", Influence::Lawful),
        ("Free Thinking. Inquiry and curiosity are the pillars of progress. ", Influence::Chaotic),
        ("Power. Solitude and contemplation are paths toward mystical or magical power.", Influence::Evil),
        ("Live and Let Live. Meddling in the affairs of others only causes trouble.", Influence::Neutral),
        ("Self-Knowledge. If you know yourself, there's nothing left to know.", Influence::Any),
    ];
    const TRAITS: [&'static str; 8] = [
        "I've been isolated for so long that I rarely speak, preferring gestures and the occasional grunt.",
        "I am utterly serene, even in the face of disaster.",
        "The leader of my community had something wise to say on every topic, and I am eager to share that wisdom.",
        "I feel tremendous empathy for all who suffer.",
        "I'm oblivious to etiquette and social expectations.",
        "I connect everything that happens to me to a grand, cosmic plan.",
        "I often get lost in my own thoughts and contemplation, becoming oblivious to my surroundings.",
        "I am working on a grand philosophical theory and love sharing my ideas.",
    ];
}

impl Proficiencies for Hermit {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Skill(Skill::Medicine),
            Proficiency::Skill(Skill::Religion),
            Proficiency::Tool(Tool::HerbalismKit),
        ]
    }
}

impl fmt::Display for Hermit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hermit")
    }
}
