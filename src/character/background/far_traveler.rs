use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    character::{
        ability::Skill,
        backstory::Backstory,
        equipment::{
            adventuring_gear::{Gear, OtherGear},
            currency::Coin,
            tools::{GamingSet, MusicalInstrument, Tool},
            Equipment, EquipmentOption, StartingEquipment,
        },
        features::{Feature, Features},
        languages::Languages,
        proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
        Character,
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{Background, Influence, Personality, PersonalityOptions};

const SKILLS: &[Skill] = &[Skill::Insight, Skill::Perception];

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Homeland {
    Evermeet,
    Halruaa,
    #[strum(serialize = "Kara-Tur")]
    KaraTur,
    Mulhorand,
    Sossal,
    Zakhara,
    #[strum(serialize = "The Underdark")]
    Underdark,
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Reason {
    Emissary,
    Exile,
    Fugitive,
    Pilgrim,
    Sightseer,
    Wanderer,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct FarTraveler {
    homeland: Homeland,
    reason: Reason,
}

#[typetag::serde]
impl Background for FarTraveler {
    fn gen(rng: &mut impl Rng, _: &Character) -> (Box<dyn Background>, Personality) {
        (
            Box::new(Self {
                homeland: Homeland::iter().choose(rng).unwrap(),
                reason: Reason::iter().choose(rng).unwrap(),
            }),
            Self::gen_personality(rng),
        )
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for FarTraveler {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Homeland: {}", self.homeland)]
    }
}

impl Citations for FarTraveler {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 148)])
    }
}

impl Features for FarTraveler {
    fn features(&self) -> Vec<Feature> {
        // Your accent, mannerisms, figures of speech, and perhaps even your appearance all mark you as foreign. Curious glances are directed your way wherever you go, which can be a nuisance, but you also gain the friendly interest of scholars and others intrigued by far-off lands, to say nothing of everyday folk who are eager to hear stories of your homeland.
        // You can parley this attention into access to people and places you might not otherwise have, for you and your traveling companions. Noble lords, scholars, and merchant princes, to name a few, might be interested in hearing about your distant homeland and people.
        vec![Feature {
            title: "All Eyes on You",
            citation: Citation(Book::Scag, 149),
        }]
    }
}

impl Languages for FarTraveler {
    fn addl_languages(&self) -> usize {
        1
    }
}

impl PersonalityOptions for FarTraveler {
    const BONDS: &'static [&'static str] = &[
        "So long as I have this token from my homeland, I can face any adversity in this strange land.",
        "The gods of my people are a comfort to me so far from home.",
        "I hold no greater cause than my service to my people.",
        "My freedom is my most precious possession. I'll never let anyone take it from me again.",
        "I'm fascinated by the beauty and wonder of this new land.",
        "Though I had no choice, I lament having to leave my loved ones behind. I hope to see them again one day.",
    ];
    const FLAWS: &'static [&'static str] = &[
        "I am secretly (or not so secretly) convinced of the superiority of my own culture over that of this foreign land.",
        "I pretend not to understand the local language in order to avoid interactions I would rather not have.",
        "I have a weakness for the new intoxicants and other pleasures of this land.",
        "I don't take kindly to some of the actions and motivations of the people of this land, because these folk are different from me.",
        "I consider the adherents of other gods to be deluded innocents at best, or ignorant fools at worst.",
        "I have a weakness for the exotic beauty of the people of these lands.",
    ];
    const IDEALS: &'static [(&'static str, Influence)] = &[
        ("Open. I have much to learn from the kindly folk I meet along my way.", Influence::Good),
        ("Reserved. As someone new to these strange lands, I am cautious and respectful in my dealings.", Influence::Lawful),
        ("Adventure. I'm far from home, and everything is strange and wonderful!", Influence::Chaotic),
        ("Cunning. Though I may not know their ways, neither do they know mine, which can be to my advantage.", Influence::Evil),
        ("Inquisitive. Everything is new, but I have a thirst to learn.", Influence::Neutral),
        ("Suspicious. I must be careful, for I have no way of telling friend from foe here.", Influence::Any),
    ];
    const TRAITS: &'static [&'static str] = &[
        "I have different assumptions from those around me concerning personal space, blithely invading others' space in innocence, or reacting to ignorant invasion of my own.",
        "I have my own ideas about what is and is not food, and I find the eating habits of those around me fascinating, confusing, or revolting.",
        "I have a strong code of honor or sense of propriety that others don't comprehend.",
        "I express affection or contempt in ways that are unfamiliar to others.",
        "I honor my deities through practices that are foreign to this land.",
        "I begin or end my day with small traditional rituals that are unfamiliar to those around me.",
    ];
}

impl Proficiencies for FarTraveler {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::FromOptions(vec![
            ProficiencyOption::GamingSet,
            ProficiencyOption::MusicalInstrument,
        ])]
    }
}

impl StartingEquipment for FarTraveler {
    // TODO: a small piece of jewelry worth 10gp in the style of your homeland's craftsmanship, and a pouch containing 5gp
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 15)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Other(
                "poorly wrought maps from your homeland that depict where you are in Faer\u{fb}n"
                    .to_string(),
            ),
            Equipment::Gear(Gear::Other(OtherGear::ClothesTravelers)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        let mut options: Vec<Equipment> = GamingSet::iter()
            .map(|g| Equipment::Tool(Tool::GamingSet(g)))
            .collect();
        options
            .extend(MusicalInstrument::iter().map(|m| Equipment::Tool(Tool::MusicalInstrument(m))));
        vec![EquipmentOption::From(options)]
    }
}

impl fmt::Display for FarTraveler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Far Traveler ({})", self.reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_snapshot() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = FarTraveler::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = FarTraveler::gen(&mut rng, &Character::default());
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(FarTraveler::skills());
    }

    #[test]
    fn test_snapshot_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = FarTraveler::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.backstory());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = FarTraveler::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = FarTraveler::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = FarTraveler::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = FarTraveler::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = FarTraveler::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = FarTraveler::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = FarTraveler::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = FarTraveler::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_equipment());
    }
}
