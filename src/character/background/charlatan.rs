use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    character::{
        ability::Skill,
        backstory::Backstory,
        equipment::{
            adventuring_gear::{Gear, OtherGear},
            currency::Coin,
            tools::Tool,
            Equipment, EquipmentOption, StartingEquipment,
        },
        features::{Feature, Features},
        languages::Languages,
        proficiencies::{Proficiencies, Proficiency},
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{Background, Influence, Personality, PersonalityOptions};

const SCAMS: &[&str] = &[
    "I cheat at games of chance.",
    "I shave coins or forge documents.",
    "I insinuate myself into people's lives to prey on their weakness and secure their fortunes.",
    "I put on new identities like clothes.",
    "I run sleight-of-hand cons on street corners.",
    "I convince people that worthless junk is worth their hard-earned money.",
];

#[derive(Deserialize, Serialize)]
pub(crate) struct Charlatan {
    scam: String,
}

impl Charlatan {
    fn gen_scam(rng: &mut impl Rng) -> String {
        String::from(*SCAMS.iter().choose(rng).unwrap())
    }
}

#[typetag::serde]
impl Background for Charlatan {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Background>, Personality) {
        let background = Box::new(Self {
            scam: Self::gen_scam(rng),
        });
        (background, Self::gen_personality(rng))
    }

    fn skills() -> Vec<Skill> {
        vec![Skill::Deception, Skill::SleightOfHand]
    }
}

impl Backstory for Charlatan {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Favorite Scheme/Scam: {}", self.scam)]
    }
}

impl Citations for Charlatan {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::PHB, 128)])
    }
}

impl Features for Charlatan {
    fn features(&self) -> Vec<Feature> {
        // You have created a second identity that includes documentation, established acquaintances, and disguises that allow you to assume that persona. Additionally, you can forge documents including official papers and personal letters, as long as you have seen an example of the kind of document or the handwriting you are trying to copy.
        vec![Feature {
            title: "False Identity",
            citation: Citation(Book::PHB, 128),
        }]
    }
}

impl Languages for Charlatan {
    fn addl_languages(&self) -> usize {
        2
    }
}

impl PersonalityOptions for Charlatan {
    const BONDS: [&'static str; 6] = [
        "I fleeced the wrong person and must work to ensure that this individual never crosses paths with me or those I care about.",
        "I owe everything to my mentor \u{2014} a horrible person who's probably rotting in jail somewhere.",
        "Somewhere out there, I have a child who doesn't know me. I'm making the world better for him or her.",
        "I come from a noble family, and one day I'll reclaim my lands and title from those who stole them from me.",
        "A powerful person killed someone I love. Some day soon, I'll have my revenge.",
        "I swindled and ruined a person who didn't deserve it. I seek to atone for my misdeeds but might never be able to forgive myself.",
    ];
    const FLAWS: [&'static str; 6] = [
        "I can't resist a pretty face.",
        "I'm always in debt. I spend my ill-gotten gains on decadent luxuries faster than I bring them in.",
        "I'm convinced that no one could ever fool me the way I fool others.",
        "I'm too greedy for my own good. I can't resist taking a risk if there's money involved.",
        "I can't resist swindling people who are more powerful than me.",
        "I hate to admit it and will hate myself for it, but I'll run and preserve my own hide if the going gets tough.",
    ];
    const IDEALS: [(&'static str, Influence); 6] = [
        (
            "Independence. I am a free spirit \u{2014} no one tells me what to do.",
            Influence::Chaotic,
        ),
        (
            "Fairness. I never target people who can't afford to lose a few coins.",
            Influence::Lawful,
        ),
        (
            "Charity. I distribute the money I acquire to the people who really need it.",
            Influence::Good,
        ),
        (
            "Creativity. I never run the same con twice.",
            Influence::Chaotic,
        ),
        (
            "Friendship. Material goods come and go. Bonds of friendship last forever.",
            Influence::Good,
        ),
        (
            "Aspiration. I'm determined to make something of myself.",
            Influence::Any,
        ),
    ];
    const TRAITS: [&'static str; 8] = [
        "I fall in and out of love easily, and am always pursuing someone.",
        "I have a joke for every occasion, especially occasions where humor is inappropriate.",
        "Flattery is my preferred trick for getting what I want.",
        "I'm a born gambler who can't resist taking a risk for a potential payoff.",
        "I lie about almost everything, even when there's no good reason to.",
        "Sarcasm and insults are my weapons of choice.",
        "I keep multiple holy symbols on me and invoke whatever deity might come in useful at any given moment.",
        "I pocket anything I see that might have some value.",
    ];
}

impl Proficiencies for Charlatan {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![
            Proficiency::Tool(Tool::DisguiseKit),
            Proficiency::Tool(Tool::ForgerySet),
        ];
        proficiencies.extend(Self::skills().into_iter().map(Proficiency::Skill));
        proficiencies
    }
}

impl StartingEquipment for Charlatan {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 15)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Gear(Gear::Other(OtherGear::ClothesFine)),
            Equipment::Tool(Tool::DisguiseKit),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![EquipmentOption::From(
            [
                "ten stoppered bottles filled with colored liquid",
                "a set of weighted dice",
                "a deck of marked cards",
                "a signet ring of an imaginary duke",
            ]
            .iter()
            .map(|i| Equipment::Other(String::from(*i)))
            .collect(),
        )]
    }
}

impl fmt::Display for Charlatan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Charlatan")
    }
}
