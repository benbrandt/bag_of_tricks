use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::{
    adventuring_gear::{Gear, OtherGear},
    weapons::Weapon,
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
pub struct Wizard;

impl Backstory for Wizard {}

impl Citations for Wizard {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 112)])
    }
}

impl Class for Wizard {
    fn gen(_: &mut impl Rng, _: &AbilityScores) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Intelligence],
            vec![
                AbilityScoreType::Charisma,
                AbilityScoreType::Constitution,
                AbilityScoreType::Dexterity,
            ],
        )
    }
}

impl Features for Wizard {}

impl Languages for Wizard {}

impl Pantheons for Wizard {}

impl Proficiencies for Wizard {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::SavingThrow(AbilityScoreType::Intelligence),
            Proficiency::SavingThrow(AbilityScoreType::Wisdom),
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
                Skill::History,
                Skill::Insight,
                Skill::Investigation,
                Skill::Medicine,
                Skill::Religion,
            ]),
            2,
        )]
    }
}

impl StartingEquipment for Wizard {
    fn equipment(&self) -> Vec<Equipment> {
        vec![Equipment::new(
            Item::Gear(Gear::Other(OtherGear::Spellbook)),
            1,
        )]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![
            EquipmentOption::From(
                vec![
                    Equipment::new(Item::Weapon(Weapon::Quarterstaff), 1),
                    Equipment::new(Item::Weapon(Weapon::Dagger), 1),
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
            EquipmentOption::Pack(Some(vec![Pack::Explorer, Pack::Scholar])),
        ]
    }
}

impl fmt::Display for Wizard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wizard")
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
        let class = Wizard::gen(&mut rng, &AbilityScores::default());
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Wizard::gen(&mut rng, &AbilityScores::default());
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Wizard::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Wizard;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let class = Wizard;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let class = Wizard;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_equipment() {
        let class = Wizard;
        insta::assert_yaml_snapshot!(class.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let class = Wizard;
        insta::assert_yaml_snapshot!(class.addl_equipment());
    }
}
