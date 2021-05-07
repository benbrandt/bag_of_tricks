use std::fmt;

use citation::{Book, Citation, CitationList, Citations};
use personality::{Influence, PersonalityOptions};
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::{
    ability::Skill,
    backstory::Backstory,
    equipment::{
        adventuring_gear::{ArcaneFocus, Gear, OtherGear},
        currency::Coin,
        Equipment, StartingEquipment,
    },
    features::{Feature, Features},
    languages::Languages,
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
    Character,
};

use super::Background;

const SKILLS: &[Skill] = &[Skill::Athletics, Skill::Survival];
pub(crate) const BONDS: &[&str] = &[
    "My family, clan, or tribe is the most important thing in my life, even when they are far from me.",
    "An injury to the unspoiled wilderness of my home is an injury to me.",
    "I will bring terrible wrath down on the evildoers who destroyed my homeland.",
    "I am the last of my tribe, and it is up to me to ensure their names enter legend.",
    "I suffer awful visions of a coming disaster and will do anything to prevent it.",
    "It is my duty to provide children to sustain my tribe.",
];
pub(crate) const FLAWS: &[&str] = &[
    "I am too enamored of ale, wine, and other intoxicants.",
    "There's no room for caution in a life lived to the fullest.",
    "I remember every insult I've received and nurse a silent resentment toward anyone who's ever wronged me.",
    "I am slow to trust members of other races, tribes, and societies.",
    "Violence is my answer to almost any challenge.",
    "Don't expect me to save those who can't save themselves. It is nature's way that the strong thrive and the weak perish.",
];
pub(crate) const IDEALS: &[(&str, Influence)] = &[
    ("Change. Life is like the seasons, in constant change, and we must change with it.", Influence::Chaotic),
    ("Greater Good. It is each person's responsibility to make the most happiness for the whole tribe.", Influence::Good),
    ("Honor. If I dishonor myself, I dishonor my whole clan.", Influence::Lawful),
    ("Might. The strongest are meant to rule.", Influence::Evil),
    ("Nature. The natural world is more important than all the constructs of civilization.", Influence::Neutral),
    ("Glory. I must earn glory in battle, for myself and my clan.", Influence::Any),
];
pub(crate) const TRAITS: &[&str] = &[
    "I'm driven by a wanderlust that led me away from home.",
    "I watch over my friends as if they were a litter of newborn pups.",
    "I once ran twenty-five miles without stopping to warn to my clan of an approaching orc horde. I'd do it again if I had to.",
    "I have a lesson for every situation, drawn from observing nature.",
    "I place no stock in wealthy or well-mannered folk. Money and manners won't save you from a hungry owlbear.",
    "I'm always picking things up, absently fiddling with them, and sometimes accidentally breaking them.",
    "I feel far more comfortable around animals than people.",
    "I was, in fact, raised by wolves.",
];

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Origin {
    Forester,
    Trapper,
    Homesteader,
    Guide,
    #[strum(serialize = "Exile or outcast")]
    Exile,
    #[strum(serialize = "Bounty hunter")]
    BountyHunter,
    Pilgrim,
    #[strum(serialize = "Tribal nomad")]
    TribalNomad,
    #[strum(serialize = "Hunter-gatherer")]
    HunterGatherer,
    #[strum(serialize = "Tribal marauder")]
    TribalMarauder,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Outlander {
    origin: Origin,
}

#[typetag::serde]
impl Background for Outlander {
    fn gen(rng: &mut impl Rng, _: &Character) -> Box<dyn Background> {
        Box::new(Self {
            origin: Origin::iter().choose(rng).unwrap(),
        })
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for Outlander {}

impl Citations for Outlander {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 136)])
    }
}

impl Features for Outlander {
    fn features(&self) -> Vec<Feature> {
        // You have an excellent memory for maps and geography, and you can always recall the general layout of terrain, settlements, and other features around you. In addition, you can find food and fresh water for yourself and up to five other people each day, provided that the land offers berries, small game, water, and so forth.
        vec![Feature {
            title: "Wanderer",
            citation: Citation(Book::Phb, 136),
        }]
    }
}

impl Languages for Outlander {
    fn addl_languages(&self) -> usize {
        1
    }
}

impl PersonalityOptions for Outlander {
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

impl Proficiencies for Outlander {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::MusicalInstrument]
    }
}

impl StartingEquipment for Outlander {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 10)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Gear(Gear::ArcaneFocus(ArcaneFocus::Staff)),
            Equipment::Gear(Gear::Other(OtherGear::HuntingTrap)),
            Equipment::Other("a trophy from an animal you killed".into()),
            Equipment::Gear(Gear::Other(OtherGear::ClothesTravelers)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }
}

impl fmt::Display for Outlander {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Outlander ({})", self.origin)
    }
}
