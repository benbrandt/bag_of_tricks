use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::{
    armor::{Armor, ArmorType},
    tools::{MusicalInstrument, Tool},
    weapons::{Weapon, WeaponCategory},
};
use languages::Languages;
use rand::Rng;
use serde::{Deserialize, Serialize};
use stats::{
    ability::AbilityScoreType,
    equipment::{Equipment, EquipmentOption, Item, Pack, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption, WeaponProficiency},
};

use super::Class;

#[derive(Default, Deserialize, Serialize)]
pub struct Bard;

impl Backstory for Bard {}

impl Citations for Bard {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 51)])
    }
}

impl Class for Bard {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Charisma],
            vec![AbilityScoreType::Dexterity],
        )
    }
}

impl Features for Bard {}

impl Languages for Bard {}

impl Pantheons for Bard {}

impl Proficiencies for Bard {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Armor(ArmorType::Light),
            Proficiency::SavingThrow(AbilityScoreType::Charisma),
            Proficiency::SavingThrow(AbilityScoreType::Dexterity),
            Proficiency::Weapon(WeaponProficiency::Category(WeaponCategory::Simple)),
            Proficiency::Weapon(WeaponProficiency::Specific(Weapon::CrossbowHand)),
            Proficiency::Weapon(WeaponProficiency::Specific(Weapon::Longsword)),
            Proficiency::Weapon(WeaponProficiency::Specific(Weapon::Rapier)),
            Proficiency::Weapon(WeaponProficiency::Specific(Weapon::Shortsword)),
        ]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![
            ProficiencyOption::MusicalInstrument(3),
            ProficiencyOption::Skill(None, 3),
        ]
    }
}

impl StartingEquipment for Bard {
    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::new(Item::Armor(Armor::Leather), 1),
            Equipment::new(Item::Weapon(Weapon::Dagger), 1),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![
            EquipmentOption::FromOptions(
                vec![
                    EquipmentOption::From(
                        vec![Equipment::new(Item::Weapon(Weapon::Longsword), 1)],
                        1,
                    ),
                    EquipmentOption::From(vec![Equipment::new(Item::Weapon(Weapon::Rapier), 1)], 1),
                    EquipmentOption::Weapon(Some(WeaponCategory::Simple), None, 1),
                ],
                1,
            ),
            EquipmentOption::Pack(Some(vec![Pack::Diplomat, Pack::Entertainer])),
            EquipmentOption::FromOptions(
                vec![
                    EquipmentOption::From(
                        vec![Equipment::new(
                            Item::Tool(Tool::MusicalInstrument(MusicalInstrument::Lute)),
                            1,
                        )],
                        1,
                    ),
                    EquipmentOption::MusicalInstrument,
                ],
                1,
            ),
        ]
    }
}

impl fmt::Display for Bard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bard")
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
        let class = Bard::gen(&mut rng);
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Bard::gen(&mut rng);
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Bard::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Bard;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let class = Bard;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let class = Bard;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_equipment() {
        let class = Bard;
        insta::assert_yaml_snapshot!(class.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let class = Bard;
        insta::assert_yaml_snapshot!(class.addl_equipment());
    }
}
