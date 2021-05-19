use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::{Feature, Features};
use gear::{
    adventuring_gear::{Gear, OtherGear},
    currency::Coin,
    tools::{GamingSet, MusicalInstrument, Tool},
};
use languages::{LanguageType, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScores, Skill},
    equipment::{Equipment, EquipmentOption, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
};
use strum::{EnumIter, IntoEnumIterator};
use trinkets::TrinketOption;

use super::{
    folk_hero::{BONDS, FLAWS, IDEALS, TRAITS},
    Background,
};

const SKILLS: &[Skill] = &[Skill::Survival];
const ADDL_SKILLS: &[Skill] = &[Skill::Arcana, Skill::History, Skill::Religion];

#[derive(Clone, Copy, Deserialize, EnumIter, Serialize)]
enum Inheritance {
    Document,
    Trinket,
    Clothing,
    Jewelry,
    Book,
    Story,
    #[strum(serialize = "A tattoo or other body marking")]
    Tattoo,
}

impl Default for Inheritance {
    fn default() -> Self {
        Self::Document
    }
}

impl Inheritance {
    fn gen(rng: &mut impl Rng) -> Self {
        let options = Self::iter().collect::<Vec<_>>();
        *options.choose_weighted(rng, |i| i.weight()).unwrap()
    }

    fn equipment_option(self) -> EquipmentOption {
        match self {
            Inheritance::Trinket => EquipmentOption::Trinket(Some("inheritance"), None, true),
            Inheritance::Document
            | Inheritance::Clothing
            | Inheritance::Jewelry
            | Inheritance::Book
            | Inheritance::Story
            | Inheritance::Tattoo => EquipmentOption::Trinket(
                Some("inheritance"),
                Some(TrinketOption::Custom(self.items())),
                false,
            ),
        }
    }

    fn items(self) -> Vec<String> {
        let items: &[&str] = match self {
            Inheritance::Trinket => &[],
            Inheritance::Document => &["a map", "a letter", "a journal"],
            Inheritance::Clothing => &["an article of clothing"],
            Inheritance::Jewelry => &["a piece of jewelry"],
            Inheritance::Book => &["an arcane book", "a formulary"],
            Inheritance::Story => &[
                "a written story",
                "a written song",
                "a written poem",
                "a written secret",
            ],
            Inheritance::Tattoo => &["a tattoo or other body marking"],
        };
        items.iter().map(|&t| t.to_string()).collect()
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

#[derive(Default, Deserialize, Serialize)]
pub struct Inheritor {
    inheritance: Inheritance,
}

impl Background for Inheritor {
    fn gen(rng: &mut impl Rng, _: &AbilityScores, _: &[Proficiency], _: i16) -> Self {
        Self {
            inheritance: Inheritance::gen(rng),
        }
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
    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (1, None)
    }
}

impl Pantheons for Inheritor {}

impl PersonalityOptions for Inheritor {
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
                    ProficiencyOption::MusicalInstrument(1),
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
        vec![
            EquipmentOption::From(options),
            self.inheritance.equipment_option(),
        ]
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
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.traits());
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(Inheritor::skills());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Inheritor::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_equipment());
    }
}
