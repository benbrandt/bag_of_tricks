use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::{
    adventuring_gear::{Gear, OtherGear},
    weapons::{Weapon, WeaponCategory},
};
use languages::Languages;
use rand::Rng;
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScoreType, Skill},
    equipment::{Equipment, EquipmentOption, Item, Pack, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption, WeaponProficiency},
};

use super::Class;

#[derive(Default, Deserialize, Serialize)]
pub struct Sorcerer;

impl Backstory for Sorcerer {}

impl Citations for Sorcerer {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 99)])
    }
}

impl Class for Sorcerer {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Charisma],
            vec![AbilityScoreType::Constitution],
        )
    }
}

impl Features for Sorcerer {}

impl Languages for Sorcerer {}

impl Pantheons for Sorcerer {}

impl Proficiencies for Sorcerer {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::SavingThrow(AbilityScoreType::Charisma),
            Proficiency::SavingThrow(AbilityScoreType::Constitution),
            Proficiency::Weapon(WeaponProficiency::Specific(Weapon::CrossbowLight)),
            Proficiency::Weapon(WeaponProficiency::Specific(Weapon::Dagger)),
            Proficiency::Weapon(WeaponProficiency::Specific(Weapon::Dart)),
            Proficiency::Weapon(WeaponProficiency::Specific(Weapon::Quarterstaff)),
            Proficiency::Weapon(WeaponProficiency::Specific(Weapon::Sling)),
        ]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(
            Some(vec![
                Skill::Arcana,
                Skill::Deception,
                Skill::Insight,
                Skill::Intimidation,
                Skill::Persuasion,
                Skill::Religion,
            ]),
            2,
        )]
    }
}

impl StartingEquipment for Sorcerer {
    fn equipment(&self) -> Vec<Equipment> {
        vec![Equipment::new(Item::Weapon(Weapon::Dagger), 2)]
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
            EquipmentOption::Pack(Some(vec![Pack::Dungeoneer, Pack::Explorer])),
        ]
    }
}

impl fmt::Display for Sorcerer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sorcerer")
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
        let class = Sorcerer::gen(&mut rng);
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Sorcerer::gen(&mut rng);
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Sorcerer::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Sorcerer;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let class = Sorcerer;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let class = Sorcerer;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_equipment() {
        let class = Sorcerer;
        insta::assert_yaml_snapshot!(class.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let class = Sorcerer;
        insta::assert_yaml_snapshot!(class.addl_equipment());
    }
}
