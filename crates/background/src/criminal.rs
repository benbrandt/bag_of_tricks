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
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScores, Skill},
    equipment::{Equipment, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
};
use strum::{Display, EnumIter, IntoEnumIterator};

use super::Background;

const SKILLS: &[Skill] = &[Skill::Deception, Skill::Stealth];
pub(crate) const BONDS: &[&str] = &[
    "I'm trying to pay off an old debt I owe to a generous benefactor.",
    "My ill-gotten gains go to support my family.",
    "Something important was taken from me, and I aim to steal it back.",
    "I will become the greatest thief that ever lived.",
    "I'm guilty of a terrible crime. I hope I can redeem myself for it.",
    "Someone I loved died because of a mistake I made. That will never happen again.",
];
pub(crate) const FLAWS: &[&str] = &[
    "When I see something valuable, I can't think about anything but how to steal it.",
    "When faced with a choice between money and my friends, I usually choose the money.",
    "If there's a plan, I'll forget it. If I don't forget it, I'll ignore it.",
    "I have a \"tell\" that reveals when I'm lying.",
    "I turn tail and run when things look bad.",
    "An innocent person is in prison for a crime that I committed. I'm okay with that.",
];
pub(crate) const IDEALS: &[(&str, Influence)] = &[
    (
        "Honor. I don't steal from others in the trade.",
        Influence::Lawful,
    ),
    (
        "Freedom. Chains are meant to be broken, as are those who would forge them.",
        Influence::Chaotic,
    ),
    (
        "Charity. I steal from the wealthy so that I can help people in need.",
        Influence::Good,
    ),
    (
        "Greed. I will do whatever it takes to become wealthy.",
        Influence::Evil,
    ),
    ("People. I'm loyal to my friends, not to any ideals, and everyone else can take a trip down the Styx for all I care.", Influence::Neutral),
    ("Redemption. There's a spark of good in everyone.", Influence::Good),
];
pub(crate) const TRAITS: &[&str] = &[
    "I always have a plan for what to do when things go wrong.",
    "I am always calm, no matter what the situation. I never raise my voice or let my emotions control me.",
    "The first thing I do in a new place is note the locations of everything valuable \u{2014} or where such things could be hidden.",
    "I would rather make a new friend than a new enemy.",
    "I am incredibly slow to trust. Those who seem the fairest often have the most to hide.",
    "I don't pay attention to the risks in a situation. Never tell me the odds.",
    "The best way to get me to do something is to tell me I can't do it.",
    "I blow up at the slightest insult.",
];

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Specialty {
    Blackmailer,
    Burglar,
    Enforcer,
    Fence,
    #[strum(serialize = "Highway robber")]
    HighwayRobber,
    #[strum(serialize = "Hired killer")]
    HiredKiller,
    Pickpocket,
    Smuggler,
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Variant {
    Criminal,
    Spy,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Criminal {
    specialty: Specialty,
    variant: Variant,
}

#[typetag::serde]
impl Background for Criminal {
    fn gen(
        rng: &mut impl Rng,
        _: &AbilityScores,
        _: &[Proficiency],
        _: i16,
    ) -> Box<dyn Background> {
        Box::new(Self {
            specialty: Specialty::iter().choose(rng).unwrap(),
            variant: Variant::iter().choose(rng).unwrap(),
        })
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for Criminal {}

impl Citations for Criminal {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 129)])
    }
}

impl Features for Criminal {
    fn features(&self) -> Vec<Feature> {
        // You have a reliable and trustworthy contact who acts as your liaison to a network of other criminals. You know how to get messages to and from your contact, even over great distances; specifically, you know the local messengers, corrupt caravan masters, and seedy sailors who can deliver messages for you.
        vec![Feature {
            title: "Criminal Contact",
            citation: Citation(Book::Phb, 129),
        }]
    }
}

impl Languages for Criminal {}

impl Pantheons for Criminal {}

impl PersonalityOptions for Criminal {
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

impl Proficiencies for Criminal {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![Proficiency::Tool(Tool::ThievesTools)];
        proficiencies.extend(SKILLS.iter().map(|&s| Proficiency::Skill(s)));
        proficiencies
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::GamingSet]
    }
}

impl StartingEquipment for Criminal {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 15)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Gear(Gear::Other(OtherGear::Crowbar)),
            Equipment::Other("a set of dark common clothes including a hood".into()),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }
}

impl fmt::Display for Criminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.variant, self.specialty)
    }
}
