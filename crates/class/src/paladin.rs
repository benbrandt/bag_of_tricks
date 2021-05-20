use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::{
    armor::{Armor, ArmorType},
    weapons::{Weapon, WeaponCategory, WeaponClassification},
};
use itertools::Itertools;
use languages::Languages;
use rand::Rng;
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScoreType, AbilityScores, Skill},
    equipment::{Equipment, EquipmentOption, Item, Pack, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption, WeaponProficiency},
};
use strum::IntoEnumIterator;

use super::Class;

#[derive(Default, Deserialize, Serialize)]
pub struct Paladin;

impl Backstory for Paladin {}

impl Citations for Paladin {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 82)])
    }
}

impl Class for Paladin {
    fn gen(_: &mut impl Rng, _: &AbilityScores) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Strength],
            vec![AbilityScoreType::Charisma],
        )
    }
}

impl Features for Paladin {}

impl Languages for Paladin {}

impl Pantheons for Paladin {
    fn deity_required(&self) -> bool {
        true
    }
}

impl Proficiencies for Paladin {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = ArmorType::iter().map(Proficiency::Armor).collect_vec();
        proficiencies.extend(
            WeaponCategory::iter().map(|c| Proficiency::Weapon(WeaponProficiency::Category(c))),
        );
        proficiencies.extend(vec![
            Proficiency::SavingThrow(AbilityScoreType::Charisma),
            Proficiency::SavingThrow(AbilityScoreType::Wisdom),
        ]);
        proficiencies
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(
            Some(vec![
                Skill::Athletics,
                Skill::Insight,
                Skill::Intimidation,
                Skill::Medicine,
                Skill::Persuasion,
                Skill::Religion,
            ]),
            2,
        )]
    }
}

impl StartingEquipment for Paladin {
    fn equipment(&self) -> Vec<Equipment> {
        vec![Equipment::new(Item::Armor(Armor::ChainMail), 1)]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![
            EquipmentOption::Weapon(Some(WeaponCategory::Martial), None, 1),
            EquipmentOption::FromOptions(
                vec![
                    EquipmentOption::From(vec![Equipment::new(Item::Armor(Armor::Shield), 1)], 1),
                    EquipmentOption::Weapon(Some(WeaponCategory::Martial), None, 1),
                ],
                1,
            ),
            EquipmentOption::FromOptions(
                vec![
                    EquipmentOption::From(
                        vec![Equipment::new(Item::Weapon(Weapon::Javelin), 5)],
                        1,
                    ),
                    EquipmentOption::Weapon(
                        Some(WeaponCategory::Simple),
                        Some(WeaponClassification::Melee),
                        1,
                    ),
                ],
                1,
            ),
            EquipmentOption::Pack(Some(vec![Pack::Explorer, Pack::Priest])),
            EquipmentOption::HolySymbol,
        ]
    }
}

impl fmt::Display for Paladin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Paladin")
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
        let class = Paladin::gen(&mut rng, &AbilityScores::default());
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Paladin::gen(&mut rng, &AbilityScores::default());
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Paladin::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Paladin;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_deity_required() {
        let class = Paladin;
        insta::assert_yaml_snapshot!(class.deity_required());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let class = Paladin;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let class = Paladin;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_equipment() {
        let class = Paladin;
        insta::assert_yaml_snapshot!(class.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let class = Paladin;
        insta::assert_yaml_snapshot!(class.addl_equipment());
    }
}
