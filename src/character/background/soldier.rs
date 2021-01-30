use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    character::{
        ability::Skill,
        backstory::Backstory,
        equipment::tools::{Tool, Vehicles},
        features::{Feature, Features},
        languages::Languages,
        proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{Background, Influence, Personality, PersonalityOptions};

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Specialty {
    Officer,
    Scout,
    Infantry,
    Cavalry,
    Healer,
    Quartermaster,
    #[strum(serialize = "Standard bearer")]
    StandardBearer,
    #[strum(serialize = "Support staff (cook, blacksmith, or the like)")]
    SupportStaff,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Soldier {
    specialty: Specialty,
}

#[typetag::serde]
impl Background for Soldier {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Background>, Personality) {
        let background = Box::new(Self {
            specialty: Specialty::iter().choose(rng).unwrap(),
        });
        (background, Self::gen_personality(rng))
    }

    fn equipment(&self) -> String {
        String::from("An insignia of rank, a trophy taken from a fallen enemy (a dagger, broken blade, or piece of a banner), a set of bone dice or deck of cards, a set of common clothes, and a pouch containing 10 gp")
    }
}

impl Backstory for Soldier {}

impl Citations for Soldier {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::PHB, 140)])
    }
}

impl Features for Soldier {
    fn features(&self) -> Vec<Feature> {
        // You have a military rank from your career as a soldier. Soldiers loyal to your former military organization still recognize your authority and influence, and they defer to you if they are of a lower rank. You can invoke your rank to exert influence over other soldiers and requisition simple equipment or horses for temporary use. You can also usually gain access to friendly military encampments and fortresses where your rank is recognized.
        vec![Feature {
            title: "Military Rank",
            citation: Citation(Book::PHB, 140),
        }]
    }
}

impl Languages for Soldier {}

impl PersonalityOptions for Soldier {
    const BONDS: [&'static str; 6] = [
        "I would still lay down my life for the people I served with.",
        "Someone saved my life on the battlefield. To this day, I will never leave a friend behind.",
        "My honor is my life.",
        "I'll never forget the crushing defeat my company suffered or the enemies who dealt it.",
        "Those who fight beside me are those worth dying for.",
        "I fight for those who cannot fight for themselves.",
    ];
    const FLAWS: [&'static str; 6] = [
        "The monstrous enemy we faced in battle still leaves me quivering with fear.",
        "I have little respect for anyone who is not a proven warrior.",
        "I made a terrible mistake in battle that cost many lives, and I would do anything to keep that mistake secret.",
        "My hatred of my enemies is blind and unreasoning.",
        "I obey the law, even if the law causes misery.",
        "I'd rather eat my armor than admit when I'm wrong.",
    ];
    const IDEALS: [(&'static str, Influence); 6] = [
        (
            "Greater Good. Our lot is to lay down our lives in defense of others.",
            Influence::Good,
        ),
        (
            "Responsibility. I do what I must and obey just authority.",
            Influence::Lawful,
        ),
        (
            "Independence. When people follow orders blindly, they embrace a kind of tyranny.",
            Influence::Chaotic,
        ),
        (
            "Might. In life as in war, the stronger force wins.",
            Influence::Evil,
        ),
        (
            "Live and Let Live. Ideals aren't worth killing over or going to war for.",
            Influence::Neutral,
        ),
        (
            "Nation. My city, nation, or people are all that matter.",
            Influence::Any,
        ),
    ];
    const TRAITS: [&'static str; 8] = [
        "I'm always polite and respectful.",
        "I'm haunted by memories of war. I can't get the images of violence out of my mind.",
        "I've lost too many friends, and I'm slow to make new ones.",
        "I'm full of inspiring and cautionary tales from my military experience relevant to almost every combat situation.",
        "I can stare down a hell hound without flinching.",
        "I enjoy being strong and like breaking things.",
        "I have a crude sense of humor.",
        "I face problems head-on. A simple, direct solution is the best path to success.",
    ];
}

impl Proficiencies for Soldier {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Skill(Skill::Athletics),
            Proficiency::Skill(Skill::Intimidation),
            Proficiency::Tool(Tool::Vehicles(Vehicles::Land)),
        ]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::GamingSet]
    }
}

impl fmt::Display for Soldier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Soldier ({})", self.specialty)
    }
}
