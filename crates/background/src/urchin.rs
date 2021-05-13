use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::{Feature, Features};
use gear::{
    adventuring_gear::{Gear, OtherGear},
    currency::Coin,
    tools::Tool,
};
use languages::Languages;
use personality::{Influence, PersonalityOptions};
use rand::Rng;
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScores, Skill},
    equipment::{Equipment, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency},
};

use super::Background;

const SKILLS: &[Skill] = &[Skill::SleightOfHand, Skill::Stealth];
const BONDS: &[&str] = &[
    "My town or city is my home, and I'll fight to defend it.",
    "I sponsor an orphanage to keep others from enduring what I was forced to endure.",
    "I owe my survival to another urchin who taught me to live on the streets.",
    "I owe a debt I can never repay to the person who took pity on me.",
    "I escaped my life of poverty by robbing an important person, and I'm wanted for it.",
    "No one else should have to endure the hardships I've been through.",
];
const FLAWS: &[&str] = &[
    "If I'm outnumbered, I will run away from a fight.",
    "Gold seems like a lot of money to me, and I'll do just about anything for more of it.",
    "I will never fully trust anyone other than myself.",
    "I'd rather kill someone in their sleep than fight fair.",
    "It's not stealing if I need it more than someone else.",
    "People who can't take care of themselves get what they deserve.",
];
const IDEALS: &[(&str, Influence)] = &[
    ("Respect. All people, rich or poor, deserve respect.", Influence::Good),
    ("Community. We have to take care of each other, because no one else is going to do it.", Influence::Lawful),
    ("Change. The low are lifted up, and the high and mighty are brought down. Change is the nature of things.", Influence::Chaotic),
    ("Retribution. The rich need to be shown what life and death are like in the gutters.", Influence::Evil),
    ("People. I help the people who help me \u{2014} that's what keeps us alive.", Influence::Neutral),
    ("Aspiration. I'm going to prove that I'm worthy of a better life.", Influence::Any),
];
const TRAITS: &[&str] = &[
    "I hide scraps of food and trinkets away in my pockets.",
    "I ask a lot of questions.",
    "I like to squeeze into small places where no one else can get to me.",
    "I sleep with my back to a wall or tree, with everything I own wrapped in a bundle in my arms.",
    "I eat like a pig and have bad manners.",
    "I think anyone who's nice to me is hiding evil intent.",
    "I don't like to bathe.",
    "I bluntly say what other people are hinting at or hiding.",
];

#[derive(Deserialize, Serialize)]
pub(crate) struct Urchin;

#[typetag::serde]
impl Background for Urchin {
    fn gen(_: &mut impl Rng, _: &AbilityScores, _: &[Proficiency], _: i16) -> Box<dyn Background> {
        Box::new(Self)
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for Urchin {}

impl Citations for Urchin {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 141)])
    }
}

impl Features for Urchin {
    fn features(&self) -> Vec<Feature> {
        // You know the secret patterns and flow to cities and can find passages through the urban sprawl that others would miss. When you are not in combat, you (and companions you lead) can travel between any two locations in the city twice as fast as your speed would normally allow.
        vec![Feature {
            title: "City Secrets",
            citation: Citation(Book::Phb, 141),
        }]
    }
}

impl Languages for Urchin {}

impl Pantheons for Urchin {}

impl PersonalityOptions for Urchin {
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

impl Proficiencies for Urchin {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![
            Proficiency::Tool(Tool::DisguiseKit),
            Proficiency::Tool(Tool::ThievesTools),
        ];
        proficiencies.extend(SKILLS.iter().map(|&s| Proficiency::Skill(s)));
        proficiencies
    }
}

impl StartingEquipment for Urchin {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 10)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Other("a small knife".into()),
            Equipment::Other("a map of the city you grew up in".into()),
            Equipment::Other("a pet mouse".into()),
            Equipment::Other("a token to remember your parents by".into()),
            Equipment::Gear(Gear::Other(OtherGear::ClothesCommon)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }
}

impl fmt::Display for Urchin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Urchin")
    }
}
