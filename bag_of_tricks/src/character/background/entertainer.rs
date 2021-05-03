use std::fmt;

use citation::{Book, Citation, CitationList, Citations};
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::character::{
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
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
    Character,
};

use super::{Background, Influence, Personality, PersonalityOptions};

const SKILLS: &[Skill] = &[Skill::Acrobatics, Skill::Performance];

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Routine {
    Actor,
    Dancer,
    #[strum(serialize = "Fire-eater")]
    FireEater,
    Jester,
    Juggler,
    Instrumentalist,
    Poet,
    Singer,
    Storyteller,
    Tumbler,
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Variant {
    Entertainer,
    Gladiator,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Entertainer {
    routines: Vec<Routine>,
    variant: Variant,
}

#[typetag::serde]
impl Background for Entertainer {
    fn gen(rng: &mut impl Rng, _: &Character) -> (Box<dyn Background>, Personality) {
        let num_routines = rng.gen_range(1..=3);
        let background = Box::new(Self {
            routines: Routine::iter().choose_multiple(rng, num_routines),
            variant: Variant::iter().choose(rng).unwrap(),
        });
        (background, Self::gen_personality(rng))
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for Entertainer {
    fn backstory(&self) -> Vec<String> {
        vec![format!(
            "Entertainer Routine: {}",
            self.routines
                .iter()
                .map(|r| format!("{}", r))
                .collect::<Vec<String>>()
                .join(", ")
        )]
    }
}

impl Citations for Entertainer {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 130)])
    }
}

impl Features for Entertainer {
    fn features(&self) -> Vec<Feature> {
        // You can always find a place to perform, usually in an inn or tavern but possibly with a circus, at a theater, or even in a noble’s court. At such a place, you receive free lodging and food of a modest or comfortable standard (depending on the quality of the establishment), as long as you perform each night. In addition, your performance makes you something of a local figure. When strangers recognize you in a town where you have performed, they typically take a liking to you.
        // A gladiator is as much an entertainer as any minstrel or circus performer, trained to make the arts of combat into a spectacle the crowd can enjoy. This kind of flashy combat is your entertainer routine, though you might also have some skills as a tumbler or actor. Using your By Popular Demand feature, you can find a place to perform in any place that features combat for entertainment — perhaps a gladiatorial arena or secret pit fighting club. You can replace the musical instrument in your equipment package with an inexpensive but unusual weapon, such as a trident or net.
        vec![Feature {
            title: "By Popular Demand",
            citation: Citation(Book::Phb, 130),
        }]
    }
}

impl Languages for Entertainer {}

impl PersonalityOptions for Entertainer {
    const BONDS: &'static [&'static str] = &[
        "My instrument is my most treasured possession, and it reminds me of someone I love.",
        "Someone stole my precious instrument, and someday I'll get it back.",
        "I want to be famous, whatever it takes.",
        "I idolize a hero of the old tales and measure my deeds against that person's.",
        "I will do anything to prove myself superior to my hated rival.",
        "I would do anything for the other members of my old troupe.",
    ];
    const FLAWS: &'static [&'static str] = &[
        "I'll do anything to win fame and renown.",
        "I'm a sucker for a pretty face.",
        "A scandal prevents me from ever going home again. That kind of trouble seems to follow me around.",
        "I once satirized a noble who still wants my head. It was a mistake that I will likely repeat.",
        "I have trouble keeping my true feelings hidden. My sharp tongue lands me in trouble.",
        "Despite my best efforts, I am unreliable to my friends.",
    ];
    const IDEALS: &'static [(&'static str, Influence)] = &[
        (
            "Beauty. When I perform, I make the world better than it was.",
            Influence::Good,
        ),
        ("Tradition. The stories, legends, and songs of the past must never be forgotten, for they teach us who we are.", Influence::Lawful),
        (
            "Creativity. The world is in need of new ideas and bold action.",
            Influence::Chaotic,
        ),
        ("Greed. I'm only in it for the money and fame.", Influence::Evil),
        ("People. I like seeing the smiles on people's faces when I perform. That's all that matters.", Influence::Neutral),
        ("Honesty. Art should reflect the soul; it should come from within and reveal who we really are.", Influence::Any),
    ];
    const TRAITS: &'static [&'static str] = &[
        "I know a story relevant to almost every situation.",
        "Whenever I come to a new place, I collect local rumors and spread gossip.",
        "I'm a hopeless romantic, always searching for that \"special someone.\"",
        "Nobody stays angry at me or around me for long, since I can defuse any amount of tension.",
        "I love a good insult, even one directed at me.",
        "I get bitter if I'm not the center of attention.",
        "I'll settle for nothing less than perfection.",
        "I change my mood or my mind as quickly as I change key in a song.",
    ];
}

impl Proficiencies for Entertainer {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![Proficiency::Tool(Tool::DisguiseKit)];
        proficiencies.extend(SKILLS.iter().map(|&s| Proficiency::Skill(s)));
        proficiencies
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::MusicalInstrument]
    }
}

impl StartingEquipment for Entertainer {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 15)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Gear(Gear::Other(OtherGear::ClothesCostume)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![
            EquipmentOption::MusicalInstrument,
            EquipmentOption::From(
                ["love letter", "lock of hair", "trinket"]
                    .iter()
                    .map(|i| Equipment::Other(format!("{} (the favor of an admirer)", i)))
                    .collect(),
            ),
        ]
    }
}

impl fmt::Display for Entertainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.variant)
    }
}
