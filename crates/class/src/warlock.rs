use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::{
    adventuring_gear::{Gear, OtherGear},
    armor::{Armor, ArmorType},
    weapons::{Weapon, WeaponCategory},
};
use languages::Languages;
use rand::Rng;
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScoreType, AbilityScores, Skill},
    equipment::{Equipment, EquipmentOption, Item, Pack, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption, WeaponProficiency},
};

use super::Class;

#[derive(Default, Deserialize, Serialize)]
pub struct Warlock;

impl Backstory for Warlock {}

impl Citations for Warlock {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 105)])
    }
}

impl Class for Warlock {
    fn gen(_: &mut impl Rng, _: &AbilityScores) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Charisma],
            vec![AbilityScoreType::Constitution],
        )
    }
}

impl Features for Warlock {}

impl Languages for Warlock {}

impl Pantheons for Warlock {}

impl Proficiencies for Warlock {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Armor(ArmorType::Light),
            Proficiency::SavingThrow(AbilityScoreType::Charisma),
            Proficiency::SavingThrow(AbilityScoreType::Wisdom),
            Proficiency::Weapon(WeaponProficiency::Category(WeaponCategory::Simple)),
        ]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(
            Some(vec![
                Skill::Arcana,
                Skill::Deception,
                Skill::History,
                Skill::Intimidation,
                Skill::Investigation,
                Skill::Nature,
                Skill::Religion,
            ]),
            2,
        )]
    }
}

impl StartingEquipment for Warlock {
    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::new(Item::Armor(Armor::Leather), 1),
            Equipment::new(Item::Weapon(Weapon::Dagger), 2),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![
            EquipmentOption::FromOptions(
                vec![
                    EquipmentOption::From(
                        vec![Equipment::new(Item::Weapon(Weapon::CrossbowLight), 1)],
                        1,
                    ),
                    EquipmentOption::Weapon(Some(WeaponCategory::Simple), None, 1),
                ],
                1,
            ),
            EquipmentOption::FromOptions(
                vec![
                    EquipmentOption::From(
                        vec![Equipment::new(
                            Item::Gear(Gear::Other(OtherGear::ComponentPouch)),
                            1,
                        )],
                        1,
                    ),
                    EquipmentOption::ArcaneFocus,
                ],
                1,
            ),
            EquipmentOption::Pack(Some(vec![Pack::Dungeoneer, Pack::Scholar])),
            EquipmentOption::Weapon(Some(WeaponCategory::Simple), None, 1),
        ]
    }
}

impl fmt::Display for Warlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Warlock")
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
        let class = Warlock::gen(&mut rng, &AbilityScores::default());
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Warlock::gen(&mut rng, &AbilityScores::default());
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Warlock::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Warlock;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let class = Warlock;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let class = Warlock;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_equipment() {
        let class = Warlock;
        insta::assert_yaml_snapshot!(class.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let class = Warlock;
        insta::assert_yaml_snapshot!(class.addl_equipment());
    }
}
