use std::fmt;

use citation::{Book, Citation, CitationList, Citations};
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::{
    ability::Skill,
    backstory::Backstory,
    equipment::{
        adventuring_gear::{Gear, OtherGear},
        currency::Coin,
        Equipment, StartingEquipment,
    },
    features::{Feature, Features},
    languages::Languages,
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
    Character,
};

use super::{Background, Influence, Personality, PersonalityOptions};

const SKILLS: &[Skill] = &[Skill::History, Skill::Persuasion];

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Variant {
    Knight,
    Noble,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Noble {
    variant: Variant,
}

#[typetag::serde]
impl Background for Noble {
    fn gen(rng: &mut impl Rng, _: &Character) -> (Box<dyn Background>, Personality) {
        let background = Box::new(Self {
            variant: Variant::iter().choose(rng).unwrap(),
        });
        (background, Self::gen_personality(rng))
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for Noble {}

impl Citations for Noble {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 135)])
    }
}

impl Features for Noble {
    fn features(&self) -> Vec<Feature> {
        match self.variant {
            Variant::Knight => {
                // You have the service of three retainers loyal to your family. These retainers can be attendants or messengers, and one might be a majordomo. Your retainers are commoners who can perform mundane tasks for you, but they do not fight for you, will not follow you into obviously dangerous areas (such as dungeons), and will leave if they are frequently endangered or abused.
                vec![Feature {
                    title: "Retainers",
                    citation: Citation(Book::Phb, 136),
                }]
            }
            Variant::Noble => {
                // You have the service of three retainers loyal to your family. These retainers can be attendants or messengers, and one might be a majordomo. Your retainers are commoners who can perform mundane tasks for you, but they do not fight for you, will not follow you into obviously dangerous areas (such as dungeons), and will leave if they are frequently endangered or abused.
                vec![Feature {
                    title: "Position of Privilege",
                    citation: Citation(Book::Phb, 135),
                }]
            }
        }
    }
}

impl Languages for Noble {
    fn addl_languages(&self) -> usize {
        1
    }
}

impl PersonalityOptions for Noble {
    const BONDS: &'static [&'static str] = &[
        "I will face any challenge to win the approval of my family.",
        "My house's alliance with another noble family must be sustained at all costs.",
        "Nothing is more important than the other members of my family.",
        "I am in love with the heir of a family that my family despises.",
        "My loyalty to my sovereign is unwavering.",
        "The common folk must see me as a hero of the people.",
    ];
    const FLAWS: &'static [&'static str] = &[
        "I secretly believe that everyone is beneath me.",
        "I hide a truly scandalous secret that could ruin my family forever.",
        "I too often hear veiled insults and threats in every word addressed to me, and I'm quick to anger.",
        "I have an insatiable desire for carnal pleasures.",
        "In fact, the world does revolve around me.",
        "By my words and actions, I often bring shame to my family.",
    ];
    const IDEALS: &'static [(&'static str, Influence)] = &[
        ("Respect. Respect is due to me because of my position, but all people regardless of station deserve to be treated with dignity.", Influence::Good),
        ("Responsibility. It is my duty to respect the authority of those above me, just as those below me must respect mine.", Influence::Lawful),
        ("Independence. I must prove that I can handle myself without the coddling of my family.", Influence::Chaotic),
        ("Power. If I can attain more power, no one will tell me what to do.", Influence::Evil),
        ("Family. Blood runs thicker than water.", Influence::Any),
        ("Noble Obligation. It is my duty to protect and care for the people beneath me.", Influence::Good),
    ];
    const TRAITS: &'static [&'static str] = &[
        "My eloquent flattery makes everyone I talk to feel like the most wonderful and important person in the world.",
        "The common folk love me for my kindness and generosity.",
        "No one could doubt by looking at my regal bearing that I am a cut above the unwashed masses.",
        "I take great pains to always look my best and follow the latest fashions.",
        "I don't like to get my hands dirty, and I won't be caught dead in unsuitable accommodations.",
        "Despite my noble birth, I do not place myself above other folk. We all have the same blood.",
        "My favor, once lost, is lost forever.",
        "If you do me an injury, I will crush you, ruin your name, and salt your fields.",
    ];
}

impl Proficiencies for Noble {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::GamingSet]
    }
}

impl StartingEquipment for Noble {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 25)
    }

    fn equipment(&self) -> Vec<Equipment> {
        let mut equipment = vec![
            Equipment::Gear(Gear::Other(OtherGear::ClothesFine)),
            Equipment::Gear(Gear::Other(OtherGear::SignetRing)),
            Equipment::Other("a scroll of pedigree".into()),
            Equipment::Other("a purse".into()),
        ];
        if let Variant::Knight = self.variant {
            equipment.push(Equipment::Other("a banner or other token from a noble lord or lady to whom you have given your heart \u{2014} in a chaste sort of devotion".into()));
        }
        equipment
    }
}

impl fmt::Display for Noble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.variant)
    }
}
