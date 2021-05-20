use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::{
    armor::ArmorType,
    weapons::{Weapon, WeaponCategory, WeaponClassification},
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
pub struct Barbarian;

impl Backstory for Barbarian {}

impl Citations for Barbarian {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 46)])
    }
}

impl Class for Barbarian {
    fn gen(_: &mut impl Rng, _: &AbilityScores) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Strength],
            vec![AbilityScoreType::Constitution],
        )
    }
}

impl Features for Barbarian {}

impl Languages for Barbarian {}

impl Pantheons for Barbarian {}

impl Proficiencies for Barbarian {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Armor(ArmorType::Light),
            Proficiency::Armor(ArmorType::Medium),
            Proficiency::Armor(ArmorType::Shield),
            Proficiency::SavingThrow(AbilityScoreType::Constitution),
            Proficiency::SavingThrow(AbilityScoreType::Strength),
            Proficiency::Weapon(WeaponProficiency::Category(WeaponCategory::Martial)),
            Proficiency::Weapon(WeaponProficiency::Category(WeaponCategory::Simple)),
        ]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(
            Some(vec![
                Skill::AnimalHandling,
                Skill::Athletics,
                Skill::Intimidation,
                Skill::Nature,
                Skill::Perception,
                Skill::Survival,
            ]),
            2,
        )]
    }
}

impl StartingEquipment for Barbarian {
    fn equipment(&self) -> Vec<Equipment> {
        let mut equipment = Pack::Explorer.equipment();
        equipment.push(Equipment::new(Item::Weapon(Weapon::Javelin), 4));
        equipment
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        let mut addl_equipment = Pack::Explorer.addl_equipment();
        addl_equipment.extend(vec![
            EquipmentOption::FromOptions(
                vec![
                    // TODO: This weapon not valid for all players
                    EquipmentOption::From(
                        vec![Equipment::new(Item::Weapon(Weapon::Greataxe), 1)],
                        1,
                    ),
                    EquipmentOption::Weapon(
                        Some(WeaponCategory::Martial),
                        Some(WeaponClassification::Melee),
                        1,
                    ),
                ],
                1,
            ),
            EquipmentOption::FromOptions(
                vec![
                    EquipmentOption::From(
                        vec![Equipment::new(Item::Weapon(Weapon::Handaxe), 2)],
                        1,
                    ),
                    EquipmentOption::Weapon(Some(WeaponCategory::Simple), None, 1),
                ],
                1,
            ),
        ]);
        addl_equipment
    }
}

impl fmt::Display for Barbarian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Barbarian")
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
        let class = Barbarian::gen(&mut rng, &AbilityScores::default());
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Barbarian::gen(&mut rng, &AbilityScores::default());
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Barbarian::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Barbarian;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let class = Barbarian;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let class = Barbarian;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_equipment() {
        let class = Barbarian;
        insta::assert_yaml_snapshot!(class.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let class = Barbarian;
        insta::assert_yaml_snapshot!(class.addl_equipment());
    }
}
