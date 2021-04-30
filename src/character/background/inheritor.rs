use std::fmt;

use rand::{prelude::SliceRandom, Rng};
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
            trinkets::TRINKETS,
            Equipment, EquipmentOption, StartingEquipment,
        },
        features::{Feature, Features},
        languages::Languages,
        proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
        Character,
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{folk_hero::FolkHero, Background, Personality, PersonalityOptions};

const SKILLS: &[Skill] = &[Skill::Survival];
const ADDL_SKILLS: &[Skill] = &[Skill::Arcana, Skill::History, Skill::Religion];

#[derive(Clone, Copy, Deserialize, Display, EnumIter, Serialize)]
enum Inheritance {
    #[strum(serialize = "A document such as a map, a letter, or a journal")]
    Document,
    #[strum(serialize = "A trinket")]
    Trinket,
    #[strum(serialize = "An article of clothing")]
    Clothing,
    #[strum(serialize = "A piece of jewelry")]
    Jewelry,
    #[strum(serialize = "An arcane book or formulary")]
    Book,
    #[strum(serialize = "A written story, song, poem, or secret")]
    Story,
    #[strum(serialize = "A tattoo or other body marking")]
    Tattoo,
}

impl Inheritance {
    fn gen(rng: &mut impl Rng) -> Equipment {
        let options = Self::iter().collect::<Vec<_>>();
        let inheritance = options.choose_weighted(rng, |i| i.weight()).unwrap();
        match inheritance {
            Self::Trinket => {
                Equipment::Other(format!("Inheritance: {}", TRINKETS.choose(rng).unwrap()))
            }
            Self::Document
            | Self::Clothing
            | Self::Jewelry
            | Self::Book
            | Self::Story
            | Self::Tattoo => Equipment::Other(format!("Inheritance: {}", inheritance)),
        }
    }

    fn weight(self) -> usize {
        match self {
            Self::Trinket => 2,
            Self::Document
            | Self::Clothing
            | Self::Jewelry
            | Self::Book
            | Self::Story
            | Self::Tattoo => 1,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Inheritor {
    inheritance: Equipment,
}

#[typetag::serde]
impl Background for Inheritor {
    fn gen(rng: &mut impl Rng, _: &Character) -> (Box<dyn Background>, Personality) {
        (
            Box::new(Self {
                inheritance: Inheritance::gen(rng),
            }),
            FolkHero::gen_personality(rng),
        )
    }

    fn skills() -> Vec<Skill> {
        let mut skills = SKILLS.to_vec();
        skills.extend(ADDL_SKILLS);
        skills
    }
}

impl Backstory for Inheritor {}

impl Citations for Inheritor {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 150)])
    }
}

impl Features for Inheritor {
    fn features(&self) -> Vec<Feature> {
        // Choose or randomly determine your inheritance from among the possibilities in the table below. Work with your DM to come up with details: Why is your inheritance so important, and what is its full story? You might prefer for the DM to invent these details as part of the game, allowing you to learn more about your inheritance as your character does.
        // The DM is free to use your inheritance as a story hook, sending you on quests to learn more about its history or true nature, or confronting you with foes who want to claim it for themselves or prevent you from learning what you seek. The DM also determines the properties of your inheritance and how they figure into the item's history and importance. For instance, the object might be a minor magic item, or one that begins with a modest ability and increases in potency with the passage of time. Or, the true nature of your inheritance might not be apparent at first and is revealed only when certain conditions are met.
        // When you begin your adventuring career, you can decide whether to tell your companions about your inheritance right away. Rather than attracting attention to yourself, you might want to keep your inheritance a secret until you learn more about what it means to you and what it can do for you.
        vec![Feature {
            title: "Inheritance",
            citation: Citation(Book::Scag, 150),
        }]
    }
}

impl Languages for Inheritor {
    fn addl_languages(&self) -> usize {
        1
    }
}

impl Proficiencies for Inheritor {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![
            ProficiencyOption::Skill(Some(ADDL_SKILLS.to_vec()), 1),
            ProficiencyOption::FromOptions(
                vec![
                    ProficiencyOption::GamingSet,
                    ProficiencyOption::MusicalInstrument,
                ],
                1,
            ),
        ]
    }
}

impl StartingEquipment for Inheritor {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 15)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            self.inheritance.clone(),
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

impl fmt::Display for Inheritor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Inheritor")
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
        let background = Inheritor::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = Inheritor::gen(&mut rng, &Character::default());
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(Inheritor::skills());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = Inheritor::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = Inheritor::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = Inheritor::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = Inheritor::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = Inheritor::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = Inheritor::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = Inheritor::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = Inheritor::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_equipment());
    }
}
