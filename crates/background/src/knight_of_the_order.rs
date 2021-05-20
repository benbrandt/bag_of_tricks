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
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScores, Skill},
    equipment::{Equipment, EquipmentOption, Item, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
};
use strum::{Display, EnumIter, IntoEnumIterator};

use super::{
    soldier::{BONDS, FLAWS, IDEALS, TRAITS},
    Background,
};

const SKILLS: &[Skill] = &[Skill::Persuasion];
const ADDL_SKILLS: &[Skill] = &[
    Skill::Arcana,
    Skill::History,
    Skill::Nature,
    Skill::Religion,
];

#[derive(Clone, Copy, Deserialize, Display, EnumIter, Serialize)]
enum KnightlyOrder {
    #[strum(serialize = "Kelemvor's Eternal Order")]
    Kelemvor,
    #[strum(serialize = "Mystra's Knights of the Mystic Fire")]
    Mystra,
    #[strum(serialize = "Warlock Knights of Vaasa")]
    Vaasa,
    #[strum(serialize = "Knights of the Shield")]
    Shield,
    #[strum(serialize = "Knights of the Unicorn")]
    Unicorn,
    #[strum(serialize = "Knights of Myth Drannor")]
    MythDrannor,
    #[strum(serialize = "Knights of the Silver Chalice")]
    SilverChalice,
}

impl Default for KnightlyOrder {
    fn default() -> Self {
        Self::Kelemvor
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct KnightOfTheOrder {
    knightly_order: KnightlyOrder,
}

impl Background for KnightOfTheOrder {
    fn gen(rng: &mut impl Rng, _: &AbilityScores, _: &[Proficiency], _: i16) -> Self {
        Self {
            knightly_order: KnightlyOrder::iter().choose(rng).unwrap(),
        }
    }

    fn skills() -> Vec<Skill> {
        let mut skills = SKILLS.to_vec();
        skills.extend(ADDL_SKILLS);
        skills
    }
}

impl Backstory for KnightOfTheOrder {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Knightly Order: {}", self.knightly_order)]
    }
}

impl Citations for KnightOfTheOrder {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 151)])
    }
}

impl Features for KnightOfTheOrder {
    fn features(&self) -> Vec<Feature> {
        // You receive shelter and succor from members of your knightly order and those who are sympathetic to its aims. If your order is a religious one, you can gain aid from temples and other religious communities of your deity. Knights of civic orders can get help from the community – whether a lone settlement or a great nation that they serve, and knights of philosophical orders can find help from those they have aided in pursuit of their ideals, and those who share those ideals.
        // This help comes in the form of shelter and meals, and healing when appropriate, as well as occasionally risky assistance, such as a band of local citizens rallying to aid a sorely pressed knight in a fight, or those who support the order helping to smuggle a knight out of town when he or she is being hunted unjustly.
        vec![Feature {
            title: "Knightly Regard",
            citation: Citation(Book::Scag, 151),
        }]
    }
}

impl Languages for KnightOfTheOrder {
    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (1, None)
    }
}

impl Pantheons for KnightOfTheOrder {}

impl PersonalityOptions for KnightOfTheOrder {
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

impl Proficiencies for KnightOfTheOrder {
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

impl StartingEquipment for KnightOfTheOrder {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 10)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::new(Item::Gear(Gear::Other(OtherGear::ClothesTravelers)), 1),
            Equipment::new(Item::Gear(Gear::Other(OtherGear::Pouch)), 1),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![EquipmentOption::From(
            ["signet", "banner", "seal"]
                .iter()
                .map(|i| {
                    Equipment::new(
                        Item::Other(format!(
                            "a {} representing your place or rank in the order",
                            i
                        )),
                        1,
                    )
                })
                .collect(),
            1,
        )]
    }
}

impl fmt::Display for KnightOfTheOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Knight of the Order")
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
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.traits());
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(KnightOfTheOrder::skills());
    }

    #[test]
    fn test_snapshot_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.backstory());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = KnightOfTheOrder::gen(&mut rng, &AbilityScores::default(), &[], 2);
        insta::assert_yaml_snapshot!(background.addl_equipment());
    }
}
