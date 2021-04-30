use std::fmt;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    character::{
        ability::Skill,
        backstory::Backstory,
        equipment::{
            adventuring_gear::{Gear, OtherGear},
            currency::Coin,
            tools::Tool,
            Equipment, StartingEquipment,
        },
        features::{Feature, Features},
        languages::Languages,
        proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
        Character,
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{criminal::Criminal, Background, Personality, PersonalityOptions};

const SKILLS: &[Skill] = &[
    Skill::Deception,
    Skill::Insight,
    Skill::Persuasion,
    Skill::Stealth,
];

#[derive(Deserialize, Serialize)]
pub(crate) struct UrbanBountyHunter;

#[typetag::serde]
impl Background for UrbanBountyHunter {
    fn gen(rng: &mut impl Rng, _: &Character) -> (Box<dyn Background>, Personality) {
        (Box::new(Self), Criminal::gen_personality(rng))
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for UrbanBountyHunter {}

impl Citations for UrbanBountyHunter {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 153)])
    }
}

impl Features for UrbanBountyHunter {
    fn features(&self) -> Vec<Feature> {
        // You are in frequent contact with people in the segment of society that your chosen quarries move through. These people might be associated with the criminal underworld, the rough-and-tumble folk of the streets, or members of high society. This connection comes in the form of a contact in any city you visit, a person who provides information about the people and places of the local area.
        vec![Feature {
            title: "Ear to the Ground",
            citation: Citation(Book::Scag, 153),
        }]
    }
}

impl Languages for UrbanBountyHunter {}

impl Proficiencies for UrbanBountyHunter {
    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![
            ProficiencyOption::Skill(Some(SKILLS.to_vec()), 2),
            ProficiencyOption::FromOptions(
                vec![
                    ProficiencyOption::GamingSet,
                    ProficiencyOption::MusicalInstrument,
                    // TODO: Figure out how to replace it if you already get it
                    ProficiencyOption::From(vec![Proficiency::Tool(Tool::ThievesTools)], 1),
                ],
                2,
            ),
        ]
    }
}

impl StartingEquipment for UrbanBountyHunter {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 20)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Other("a set of clothes appropriate to your duties".to_string()),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }
}

impl fmt::Display for UrbanBountyHunter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Urban Bounty Hunter")
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
        let background = UrbanBountyHunter::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = UrbanBountyHunter::gen(&mut rng, &Character::default());
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(UrbanBountyHunter::skills());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = UrbanBountyHunter::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = UrbanBountyHunter::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = UrbanBountyHunter::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = UrbanBountyHunter::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = UrbanBountyHunter::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.equipment());
    }
}
