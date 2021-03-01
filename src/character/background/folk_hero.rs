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
            vehicles::VehicleProficiency,
            Equipment, EquipmentOption, StartingEquipment,
        },
        features::{Feature, Features},
        languages::Languages,
        proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{Background, Influence, Personality, PersonalityOptions};

const DEFINING_EVENTS: &[&str] = &[
    "I stood up to a tyrant's agents.",
    "I saved people during a natural disaster.",
    "I stood alone against a terrible monster.",
    "I stole from a corrupt merchant to help the poor.",
    "I led a militia to fight off an invading army.",
    "I broke into a tyrant's castle and stole weapons to arm the people.",
    "I trained the peasantry to use farm implements as weapons against a tyrant's soldiers.",
    "A lord rescinded an unpopular decree after I led a symbolic act of protest against it.",
    "A lord rescinded an unpopular decree after I led a symbolic act of protest against it.",
    "Recruited into a lord's army, I rose to leadership and was commended for my heroism.",
];

#[derive(Deserialize, Serialize)]
pub(crate) struct FolkHero {
    defining_event: String,
}

impl FolkHero {
    fn gen_defining_event(rng: &mut impl Rng) -> String {
        String::from(*DEFINING_EVENTS.iter().choose(rng).unwrap())
    }
}

#[typetag::serde]
impl Background for FolkHero {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Background>, Personality) {
        let background = Box::new(Self {
            defining_event: Self::gen_defining_event(rng),
        });
        (background, Self::gen_personality(rng))
    }

    fn skills() -> Vec<Skill> {
        vec![Skill::AnimalHandling, Skill::Survival]
    }
}

impl Backstory for FolkHero {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Defining Event: {}", self.defining_event)]
    }
}

impl Citations for FolkHero {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 131)])
    }
}

impl Features for FolkHero {
    fn features(&self) -> Vec<Feature> {
        // Since you come from the ranks of the common folk, you fit in among them with ease. You can find a place to hide, rest, or recuperate among other commoners, unless you have shown yourself to be a danger to them. They will shield you from the law or anyone else searching for you, though they will not risk their lives for you.
        vec![Feature {
            title: "Rustic Hospitality",
            citation: Citation(Book::Phb, 131),
        }]
    }
}

impl Languages for FolkHero {}

impl PersonalityOptions for FolkHero {
    const BONDS: [&'static str; 6] = [
        "I have a family, but I have no idea where they are. One day, I hope to see them again.",
        "I worked the land, I love the land, and I will protect the land.",
        "A proud noble once gave me a horrible beating, and I will take my revenge on any bully I encounter.",
        "My tools are symbols of my past life, and I carry them so that I will never forget my roots.",
        "I protect those who cannot protect themselves.",
        "I wish my childhood sweetheart had come with me to pursue my destiny.",
    ];
    const FLAWS: [&'static str; 6] = [
        "The tyrant who rules my land will stop at nothing to see me killed.",
        "I'm convinced of the significance of my destiny, and blind to my shortcomings and the risk of failure.",
        "The people who knew me when I was young know my shameful secret, so I can never go home again.",
        "I have a weakness for the vices of the city, especially hard drink.",
        "Secretly, I believe that things would be better if I were a tyrant lording over the land.",
        "I have trouble trusting in my allies.",
    ];
    const IDEALS: [(&'static str, Influence); 6] = [
        ("Respect. People deserve to be treated with dignity and respect.", Influence::Good),
        ("Fairness. No one should get preferential treatment before the law, and no one is above the law.", Influence::Lawful),
        ("Freedom. Tyrants must not be allowed to oppress the people.", Influence::Chaotic),
        ("Might. If I become strong, I can take what I want \u{2014} what I deserve.", Influence::Evil),
        ("Sincerity. There's no good in pretending to be something I'm not.", Influence::Neutral),
        ("Destiny. Nothing and no one can steer me away from my higher calling.", Influence::Any),
    ];
    const TRAITS: [&'static str; 8] = [
        "I judge people by their actions, not their words.",
        "If someone is in trouble, I'm always ready to lend help.",
        "When I set my mind to something, I follow through no matter what gets in my way.",
        "I have a strong sense of fair play and always try to find the most equitable solution to arguments.",
        "I'm confident in my own abilities and do what I can to instill confidence in others.",
        "Thinking is for other people. I prefer action.",
        "I misuse long words in an attempt to sound smarter.",
        "I get bored easily. When am I going to get on with my destiny?",
    ];
}

impl Proficiencies for FolkHero {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![Proficiency::Vehicle(VehicleProficiency::Land)];
        proficiencies.extend(Self::skills().into_iter().map(Proficiency::Skill));
        proficiencies
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::ArtisansTools]
    }
}

impl StartingEquipment for FolkHero {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 10)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Gear(Gear::Other(OtherGear::Shovel)),
            Equipment::Gear(Gear::Other(OtherGear::PotIron)),
            Equipment::Gear(Gear::Other(OtherGear::ClothesCommon)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![EquipmentOption::ArtisansTools]
    }
}

impl fmt::Display for FolkHero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Folk Hero")
    }
}
