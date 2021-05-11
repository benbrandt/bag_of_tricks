use std::fmt;

use stats::{
    ability::{AbilityScores, Skill},
    equipment::{Equipment, EquipmentOption, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency},
};
use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use features::{Feature, Features};
use gear::{
    adventuring_gear::{Gear, OtherGear},
    currency::Coin,
    tools::Tool,
    vehicles::VehicleProficiency,
};
use languages::Languages;
use personality::{Influence, PersonalityOptions};
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};
use trinkets::TrinketOption;

use super::Background;

const SKILLS: &[Skill] = &[Skill::Athletics, Skill::Perception];
const BONDS: &[&str] = &[
    "I'm loyal to my captain first, everything else second.",
    "The ship is most important \u{2014} crewmates and captains come and go.",
    "I'll always remember my first ship.",
    "In a harbor town, I have a paramour whose eyes nearly stole me from the sea.",
    "I was cheated out of my fair share of the profits, and I want to get my due.",
    "Ruthless pirates murdered my captain and crewmates, plundered our ship, and left me to die. Vengeance will be mine.",
];
const FLAWS: &[&str] = &[
    "I follow orders, even if I think they're wrong.",
    "I'll say anything to avoid having to do extra work.",
    "Once someone questions my courage, I never back down no matter how dangerous the situation.",
    "Once I start drinking, it's hard for me to stop.",
    "I can't help but pocket loose coins and other trinkets I come across.",
    "My pride will probably lead to my destruction.",
];
const IDEALS: &[(&str, Influence)] = &[
    (
        "Respect. The thing that keeps a ship together is mutual respect between captain and crew.",
        Influence::Good,
    ),
    (
        "Fairness. We all do the work, so we all share in the rewards.",
        Influence::Lawful,
    ),
    (
        "Freedom. The sea is freedom \u{2014} the freedom to go anywhere and do anything.",
        Influence::Chaotic,
    ),
    (
        "Mastery. I'm a predator, and the other ships on the sea are my prey.",
        Influence::Evil,
    ),
    (
        "People. I'm committed to my crewmates, not to ideals.",
        Influence::Neutral,
    ),
    (
        "Aspiration. Someday I'll own my own ship and chart my own destiny.",
        Influence::Any,
    ),
];
const TRAITS: &[&str] = &[
    "My friends know they can rely on me, no matter what.",
    "I work hard so that I can play hard when the work is done.",
    "I enjoy sailing into new ports and making new friends over a flagon of ale.",
    "I stretch the truth for the sake of a good story.",
    "To me, a tavern brawl is a nice way to get to know a new city.",
    "I never pass up a friendly wager.",
    "My language is as foul as an otyugh nest.",
    "I like a job well done, especially if I can convince someone else to do it.",
];

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Variant {
    Pirate,
    Sailor,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Sailor {
    variant: Variant,
}

#[typetag::serde]
impl Background for Sailor {
    fn gen(
        rng: &mut impl Rng,
        _: &AbilityScores,
        _: &[Proficiency],
        _: i16,
    ) -> Box<dyn Background> {
        Box::new(Self {
            variant: Variant::iter().choose(rng).unwrap(),
        })
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for Sailor {}

impl Citations for Sailor {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 139)])
    }
}

impl Features for Sailor {
    fn features(&self) -> Vec<Feature> {
        match self.variant {
            Variant::Pirate => {
                // No matter where you go, people are afraid of you due to your reputation. When you are in a civilized settlement, you can get away with minor criminal offenses, such as refusing to pay for food at a tavern or breaking down doors at a local shop, since most people will not report your activity to the authorities.
                vec![Feature {
                    title: "Bad Reputation",
                    citation: Citation(Book::Phb, 139),
                }]
            }
            Variant::Sailor => {
                // When you need to, you can secure free passage on a sailing ship for yourself and your adventuring companions. You might sail on the ship you served on, or another ship you have good relations with (perhaps one captained by a former crewmate). Because you're calling in a favor, you can't be certain of a schedule or route that will meet your every need. Your Dungeon Master will determine how long it takes to get where you need to go. In return for your free passage, you and your companions are expected to assist the crew during the voyage.
                vec![Feature {
                    title: "Ship's Passage",
                    citation: Citation(Book::Phb, 139),
                }]
            }
        }
    }
}

impl Languages for Sailor {}

impl PersonalityOptions for Sailor {
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

impl Proficiencies for Sailor {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![
            Proficiency::Tool(Tool::NavigatorsTools),
            Proficiency::Vehicle(VehicleProficiency::Water),
        ];
        proficiencies.extend(SKILLS.iter().map(|&s| Proficiency::Skill(s)));
        proficiencies
    }
}

impl StartingEquipment for Sailor {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 10)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Other("a belaying pin (club)".into()),
            Equipment::Gear(Gear::Other(OtherGear::RopeSilk)),
            Equipment::Gear(Gear::Other(OtherGear::ClothesCommon)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![EquipmentOption::Trinket(
            Some("lucky charm"),
            Some(TrinketOption::Custom(vec![
                "a rabbit foot".to_string(),
                "a small stone with a hole in the center".to_string(),
            ])),
            true,
        )]
    }
}

impl fmt::Display for Sailor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.variant)
    }
}
