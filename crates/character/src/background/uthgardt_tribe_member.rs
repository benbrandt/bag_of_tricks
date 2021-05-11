use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use languages::{LanguageType, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::{
    ability::{AbilityScores, Skill},
    equipment::{
        adventuring_gear::{Gear, OtherGear},
        currency::Coin,
        Equipment, EquipmentOption, StartingEquipment,
    },
    features::{Feature, Features},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
};

use super::{
    outlander::{BONDS, FLAWS, IDEALS, TRAITS},
    Background,
};

const SKILLS: &[Skill] = &[Skill::Athletics, Skill::Survival];

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Totem {
    #[strum(serialize = "BlackRaven")]
    BlackRaven,
    #[strum(serialize = "Blue Bear")]
    BlueBear,
    #[strum(serialize = "Black Lion")]
    BlackLion,
    Elk,
    #[strum(serialize = "Golden Eagle")]
    GoldenEagle,
    #[strum(serialize = "Gray Wolf")]
    GrayWolf,
    #[strum(serialize = "Great Worm")]
    GreatWorm,
    Griffon,
    #[strum(serialize = "Red Tiger")]
    RedTiger,
    #[strum(serialize = "Red Pony")]
    RedPony,
    #[strum(serialize = "Sky Pony")]
    SkyPony,
    Thunderbeast,
    #[strum(serialize = "Tree Ghost")]
    TreeGhost,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct UthgardtTribeMember {
    tribal_totem: Totem,
}

#[typetag::serde]
impl Background for UthgardtTribeMember {
    fn gen(
        rng: &mut impl Rng,
        _: &AbilityScores,
        _: &[Proficiency],
        _: i16,
    ) -> Box<dyn Background> {
        Box::new(Self {
            tribal_totem: Totem::iter().choose(rng).unwrap(),
        })
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for UthgardtTribeMember {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Tribal Totem: {}", self.tribal_totem)]
    }
}

impl Citations for UthgardtTribeMember {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 153)])
    }
}

impl Features for UthgardtTribeMember {
    fn features(&self) -> Vec<Feature> {
        // You have an excellent knowledge of not only your tribe's territory, but also the terrain and natural resources of the rest of the North. You are familiar enough with any wilderness area that you find twice as much food and water as you normally would when you forage there.
        // Additionally, you can call upon the hospitality of your people, and those folk allied with your tribe, often including members of druid circles, tribes of nomadic elves, the Harpers, and the priesthoods devoted to the gods of the First Circle.
        vec![Feature {
            title: "Uthgardt Heritage",
            citation: Citation(Book::Scag, 154),
        }]
    }
}

impl Languages for UthgardtTribeMember {
    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (1, None)
    }
}

impl PersonalityOptions for UthgardtTribeMember {
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

impl Proficiencies for UthgardtTribeMember {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::FromOptions(
            vec![
                ProficiencyOption::ArtisansTools,
                ProficiencyOption::MusicalInstrument,
            ],
            1,
        )]
    }
}

impl StartingEquipment for UthgardtTribeMember {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 10)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Gear(Gear::Other(OtherGear::HuntingTrap)),
            Equipment::Gear(Gear::Other(OtherGear::ClothesTravelers)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![EquipmentOption::From(
            ["totemic token", "set of tattoos"]
                .iter()
                .map(|i| {
                    Equipment::Other(format!(
                        "a {} marking your loyalty to Uthgar and your tribal totem",
                        i
                    ))
                })
                .collect(),
        )]
    }
}

impl fmt::Display for UthgardtTribeMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Uthgardt Tribe Member")
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
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.traits());
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(UthgardtTribeMember::skills());
    }

    #[test]
    fn test_snapshot_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.backstory());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_addl_languages() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = UthgardtTribeMember::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_equipment());
    }
}
