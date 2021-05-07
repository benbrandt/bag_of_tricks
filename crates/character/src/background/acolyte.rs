use std::fmt;

use citation::{Book, Citation, CitationList, Citations};
use personality::{Influence, PersonalityOptions};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    ability::Skill,
    backstory::Backstory,
    equipment::{
        adventuring_gear::{Gear, OtherGear},
        currency::Coin,
        Equipment, EquipmentOption, StartingEquipment,
    },
    features::{Feature, Features},
    languages::Languages,
    proficiencies::{Proficiencies, Proficiency},
    Character,
};

use super::Background;

const SKILLS: &[Skill] = &[Skill::Insight, Skill::Religion];

#[derive(Deserialize, Serialize)]
pub(crate) struct Acolyte;

#[typetag::serde]
impl Background for Acolyte {
    fn gen(_: &mut impl Rng, _: &Character) -> Box<dyn Background> {
        Box::new(Self)
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for Acolyte {}

impl Citations for Acolyte {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 127)])
    }
}

impl Features for Acolyte {
    fn features(&self) -> Vec<Feature> {
        // As an acolyte, you command the respect of those who share your faith, and you can perform the religious ceremonies of your deity. You and your adventuring companions can expect to receive free healing and care at a temple, shrine, or other established presence of your faith, though you must provide any material components needed for spells. Those who share your religion will support you (but only you) at a modest lifestyle.
        // You might also have ties to a specific temple dedicated to your chosen deity or pantheon, and you have a residence there. This could be the temple where you used to serve, if you remain on good terms with it, or a temple where you have found a new home. While near your temple, you can call upon the priests for assistance, provided the assistance you ask for is not hazardous and you remain in good standing with your temple.
        vec![Feature {
            title: "Shelter of the Faithful",
            citation: Citation(Book::Phb, 127),
        }]
    }
}

impl Languages for Acolyte {
    fn addl_languages(&self) -> usize {
        2
    }
}

impl PersonalityOptions for Acolyte {
    fn bonds(&self) -> Vec<String> {
        [
            "I would die to recover an ancient relic of my faith that was lost long ago.",
            "I will someday get revenge on the corrupt temple hierarchy who branded me a heretic.",
            "I owe my life to the priest who took me in when my parents died.",
            "Everything I do is for the common people.",
            "I will do anything to protect the temple where I served.",
            "I seek to preserve a sacred text that my enemies consider heretical and seek to destroy.",
        ].iter().map(|&s| s.to_string()).collect()
    }

    fn flaws(&self) -> Vec<String> {
        [
            "I judge others harshly, and myself even more severely.",
            "I put too much trust in those who wield power within my temple's hierarchy.",
            "My piety sometimes leads me to blindly trust those that profess faith in my god.",
            "I am inflexible in my thinking.",
            "I am suspicious of strangers and expect the worst of them.",
            "Once I pick a goal, I become obsessed with it to the detriment of everything else in my life.",
        ].iter().map(|&s| s.to_string()).collect()
    }

    fn ideals(&self) -> Vec<(String, Influence)> {
        vec![
            ("Tradition. The ancient traditions of worship and sacrifice must be preserved and upheld.".to_string(), Influence::Lawful),
            ("Charity. I always try to help those in need, no matter what the personal cost.".to_string(), Influence::Good),
            ("Change. We must help bring about the changes the gods are constantly working in the world".to_string(), Influence::Chaotic),
            ("Power. I hope to one day rise to the top of my faith's religious hierarchy.".to_string(), Influence::Lawful),
            ("Faith. I trust that my deity will guide my actions. I have faith that if I work hard, things will go well. ".to_string(), Influence::Lawful),
            ("Aspiration. I seek to prove myself worthy of my god's favor by matching my actions against his or her teachings.".to_string(), Influence::Any),
        ]
    }

    fn traits(&self) -> Vec<String> {
        [
            "I idolize a particular hero of my faith, and constantly refer to that person's deeds and example.",
            "I can find common ground between the fiercest enemies, empathizing with them and always working toward peace.",
            "I see omens in every event and action. The gods try to speak to us, we just need to listen.",
            "Nothing can shake my optimistic attitude.",
            "I quote (or misquote) sacred texts and proverbs in almost every situation.",
            "I am tolerant (or intolerant) of other faiths and respect (or condemn) the worship of other gods.",
            "I've enjoyed fine food, drink, and high society among my temple's elite. Rough living grates on me.",
            "I've spent so long in the temple that I have little practical experience dealing with people in the outside world.",
        ].iter().map(|&s| s.to_string()).collect()
    }
}

impl Proficiencies for Acolyte {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }
}

impl StartingEquipment for Acolyte {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 15)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Other("5 sticks of incense".into()),
            Equipment::Other("vestments".into()),
            Equipment::Gear(Gear::Other(OtherGear::ClothesCommon)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![
            EquipmentOption::HolySymbol,
            EquipmentOption::From(
                ["prayer book", "prayer wheel"]
                    .iter()
                    .map(|i| Equipment::Other(String::from(*i)))
                    .collect(),
            ),
        ]
    }
}

impl fmt::Display for Acolyte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Acolyte")
    }
}
