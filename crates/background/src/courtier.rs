use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::{Feature, Features};
use gear::{
    adventuring_gear::{Gear, OtherGear},
    currency::Coin,
};
use languages::{LanguageType, Languages};
use personality::{Influence, PersonalityOptions};
use rand::Rng;
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScores, Skill},
    equipment::{Equipment, Item, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency},
};

use super::{
    guild_artisan::{BONDS, FLAWS, IDEALS, TRAITS},
    Background,
};

const SKILLS: &[Skill] = &[Skill::Insight, Skill::Persuasion];

#[derive(Default, Deserialize, Serialize)]
pub struct Courtier;

impl Background for Courtier {
    fn gen(_: &mut impl Rng, _: &AbilityScores, _: &[Proficiency], _: i16) -> Self {
        Self
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for Courtier {}

impl Citations for Courtier {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 146)])
    }
}

impl Features for Courtier {
    fn features(&self) -> Vec<Feature> {
        // Your knowledge of how bureaucracies function lets you gain access to the records and inner workings of any noble court or government you encounter. You know who the movers and shakers are, whom to go to for the favors you seek, and what the current intrigues of interest in the group are.
        vec![Feature {
            title: "Court Functionary",
            citation: Citation(Book::Scag, 147),
        }]
    }
}

impl Languages for Courtier {
    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (2, None)
    }
}

impl Pantheons for Courtier {}

impl PersonalityOptions for Courtier {
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

impl Proficiencies for Courtier {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }
}

impl StartingEquipment for Courtier {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 5)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::new(Item::Gear(Gear::Other(OtherGear::ClothesFine)), 1),
            Equipment::new(Item::Gear(Gear::Other(OtherGear::Pouch)), 1),
        ]
    }
}

impl fmt::Display for Courtier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Courtier")
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
        let background = Courtier::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Courtier::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(Courtier::skills());
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Courtier::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Courtier::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Courtier::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = Courtier::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.traits());
    }

    #[test]
    fn test_snapshot_citations() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.equipment());
    }
}
