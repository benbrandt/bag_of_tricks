use std::fmt;

use citation::{Book, Citation, CitationList, Citations};
use languages::{LanguageType, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    ability::Skill,
    backstory::Backstory,
    equipment::{
        adventuring_gear::{Gear, OtherGear},
        currency::Coin,
        tools::Tool,
        Equipment, EquipmentOption, StartingEquipment,
    },
    features::{Feature, Features},
    proficiencies::{Proficiencies, Proficiency},
    Character,
};

use super::Background;

const SCAMS: &[&str] = &[
    "I cheat at games of chance.",
    "I shave coins or forge documents.",
    "I insinuate myself into people's lives to prey on their weakness and secure their fortunes.",
    "I put on new identities like clothes.",
    "I run sleight-of-hand cons on street corners.",
    "I convince people that worthless junk is worth their hard-earned money.",
];

const SKILLS: &[Skill] = &[Skill::Deception, Skill::SleightOfHand];

#[derive(Deserialize, Serialize)]
pub(crate) struct Charlatan {
    scam: String,
}

impl Charlatan {
    fn gen_scam(rng: &mut impl Rng) -> String {
        String::from(*SCAMS.choose(rng).unwrap())
    }
}

#[typetag::serde]
impl Background for Charlatan {
    fn gen(rng: &mut impl Rng, _: &Character) -> Box<dyn Background> {
        Box::new(Self {
            scam: Self::gen_scam(rng),
        })
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for Charlatan {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Favorite Scheme/Scam: {}", self.scam)]
    }
}

impl Citations for Charlatan {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 128)])
    }
}

impl Features for Charlatan {
    fn features(&self) -> Vec<Feature> {
        // You have created a second identity that includes documentation, established acquaintances, and disguises that allow you to assume that persona. Additionally, you can forge documents including official papers and personal letters, as long as you have seen an example of the kind of document or the handwriting you are trying to copy.
        vec![Feature {
            title: "False Identity",
            citation: Citation(Book::Phb, 128),
        }]
    }
}

impl Languages for Charlatan {
    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (2, None)
    }
}

impl PersonalityOptions for Charlatan {
    fn bonds(&self) -> Vec<String> {
        [
            "I fleeced the wrong person and must work to ensure that this individual never crosses paths with me or those I care about.",
            "I owe everything to my mentor \u{2014} a horrible person who's probably rotting in jail somewhere.",
            "Somewhere out there, I have a child who doesn't know me. I'm making the world better for him or her.",
            "I come from a noble family, and one day I'll reclaim my lands and title from those who stole them from me.",
            "A powerful person killed someone I love. Some day soon, I'll have my revenge.",
            "I swindled and ruined a person who didn't deserve it. I seek to atone for my misdeeds but might never be able to forgive myself.",
        ].iter().map(|&s| s.to_string()).collect()
    }

    fn flaws(&self) -> Vec<String> {
        [
            "I can't resist a pretty face.",
            "I'm always in debt. I spend my ill-gotten gains on decadent luxuries faster than I bring them in.",
            "I'm convinced that no one could ever fool me the way I fool others.",
            "I'm too greedy for my own good. I can't resist taking a risk if there's money involved.",
            "I can't resist swindling people who are more powerful than me.",
            "I hate to admit it and will hate myself for it, but I'll run and preserve my own hide if the going gets tough.",
        ].iter().map(|&s| s.to_string()).collect()
    }

    fn ideals(&self) -> Vec<(String, Influence)> {
        vec![
            (
                "Independence. I am a free spirit \u{2014} no one tells me what to do.".to_string(),
                Influence::Chaotic,
            ),
            (
                "Fairness. I never target people who can't afford to lose a few coins.".to_string(),
                Influence::Lawful,
            ),
            (
                "Charity. I distribute the money I acquire to the people who really need it."
                    .to_string(),
                Influence::Good,
            ),
            (
                "Creativity. I never run the same con twice.".to_string(),
                Influence::Chaotic,
            ),
            (
                "Friendship. Material goods come and go. Bonds of friendship last forever."
                    .to_string(),
                Influence::Good,
            ),
            (
                "Aspiration. I'm determined to make something of myself.".to_string(),
                Influence::Any,
            ),
        ]
    }

    fn traits(&self) -> Vec<String> {
        vec![
            "I fall in and out of love easily, and am always pursuing someone.",
            "I have a joke for every occasion, especially occasions where humor is inappropriate.",
            "Flattery is my preferred trick for getting what I want.",
            "I'm a born gambler who can't resist taking a risk for a potential payoff.",
            "I lie about almost everything, even when there's no good reason to.",
            "Sarcasm and insults are my weapons of choice.",
            "I keep multiple holy symbols on me and invoke whatever deity might come in useful at any given moment.",
            "I pocket anything I see that might have some value.",
        ].iter().map(|&s| s.to_string()).collect()
    }
}

impl Proficiencies for Charlatan {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![
            Proficiency::Tool(Tool::DisguiseKit),
            Proficiency::Tool(Tool::ForgerySet),
        ];
        proficiencies.extend(SKILLS.iter().map(|&s| Proficiency::Skill(s)));
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
