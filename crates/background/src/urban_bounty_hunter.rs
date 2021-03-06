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
use rand::Rng;
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScores, Skill},
    equipment::{Equipment, Item, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
};

use super::{
    criminal::{BONDS, FLAWS, IDEALS, TRAITS},
    Background,
};

const SKILLS: &[Skill] = &[
    Skill::Deception,
    Skill::Insight,
    Skill::Persuasion,
    Skill::Stealth,
];

#[derive(Default, Deserialize, Serialize)]
pub struct UrbanBountyHunter;

impl Background for UrbanBountyHunter {
    fn gen(_: &mut impl Rng, _: &AbilityScores, _: &[Proficiency], _: i16) -> Self {
        Self
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

impl Pantheons for UrbanBountyHunter {}

impl PersonalityOptions for UrbanBountyHunter {
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

impl Proficiencies for UrbanBountyHunter {
    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![
            ProficiencyOption::Skill(Some(SKILLS.to_vec()), 2),
            ProficiencyOption::FromOptions(
                vec![
                    ProficiencyOption::GamingSet,
                    ProficiencyOption::MusicalInstrument(1),
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
            Equipment::new(
                Item::Other("a set of clothes appropriate to your duties".to_string()),
                1,
            ),
            Equipment::new(Item::Gear(Gear::Other(OtherGear::Pouch)), 1),
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
        let background = UrbanBountyHunter::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UrbanBountyHunter::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UrbanBountyHunter::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UrbanBountyHunter::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UrbanBountyHunter::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UrbanBountyHunter::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.traits());
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(UrbanBountyHunter::skills());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UrbanBountyHunter::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UrbanBountyHunter::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UrbanBountyHunter::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UrbanBountyHunter::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UrbanBountyHunter::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.equipment());
    }
}
