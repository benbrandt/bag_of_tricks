use std::fmt;

use citation::{Book, Citation, CitationList, Citations};
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
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
    Character,
};

use super::{noble::Noble, Background, Personality, PersonalityOptions};

const SKILLS: &[Skill] = &[Skill::History, Skill::Persuasion];

#[derive(Deserialize, Serialize)]
pub(crate) struct WaterdhavianNoble;

#[typetag::serde]
impl Background for WaterdhavianNoble {
    fn gen(rng: &mut impl Rng, _: &Character) -> (Box<dyn Background>, Personality) {
        (Box::new(Self), Noble::gen_personality(rng))
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for WaterdhavianNoble {}

impl Citations for WaterdhavianNoble {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 154)])
    }
}

impl Features for WaterdhavianNoble {
    fn features(&self) -> Vec<Feature> {
        // While you are in Waterdeep or elsewhere in the North your house sees to your everyday needs. Your name and signet are sufficient to cover most of your expenses; the inns, taverns, and festhalls you frequent are glad to record your debt and send an accounting to your family's estate in Waterdeep to settle what you owe.
        // This advantage enables you to live a comfortable lifestyle without having to pay 2 gp a day for it, or reduces the cost of a wealthy or aristocratic lifestyle by that amount. You may not maintain a less affluent lifestyle and use the difference as income – the benefit is a line of credit, not an actual monetary reward.
        vec![Feature {
            title: "Kept in Style",
            citation: Citation(Book::Scag, 154),
        }]
    }
}

impl Languages for WaterdhavianNoble {
    fn addl_languages(&self) -> usize {
        1
    }
}

impl Proficiencies for WaterdhavianNoble {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::FromOptions(
            vec![
                ProficiencyOption::GamingSet,
                ProficiencyOption::MusicalInstrument,
            ],
            1,
        )]
    }
}

impl StartingEquipment for WaterdhavianNoble {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 20)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Other("a scroll of pedigree".to_string()),
            Equipment::Gear(Gear::Other(OtherGear::ClothesFine)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![
            EquipmentOption::From(
                ["signet ring", "brooch"]
                    .iter()
                    .map(|&i| Equipment::Other(i.to_string()))
                    .collect(),
            ),
            EquipmentOption::From(
                ["zzar", "wine"]
                    .iter()
                    .map(|i| Equipment::Other(format!("a skin of fine {}", i)))
                    .collect(),
            ),
        ]
    }
}

impl fmt::Display for WaterdhavianNoble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Waterdhavian Noble")
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
        let background = WaterdhavianNoble::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = WaterdhavianNoble::gen(&mut rng, &Character::default());
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(WaterdhavianNoble::skills());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = WaterdhavianNoble::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = WaterdhavianNoble::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_addl_languages() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = WaterdhavianNoble::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = WaterdhavianNoble::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = WaterdhavianNoble::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = WaterdhavianNoble::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = WaterdhavianNoble::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = WaterdhavianNoble::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_equipment());
    }
}
