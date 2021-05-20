use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::{
    adventuring_gear::{Gear, OtherGear},
    armor::{Armor, ArmorType},
    weapons::{Ammunition, Weapon, WeaponCategory},
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
use strum::{EnumIter, IntoEnumIterator};

use super::Class;

#[derive(Deserialize, EnumIter, Serialize)]
enum Base {
    Strength,
    Dexterity,
}

impl Default for Base {
    fn default() -> Self {
        Self::Strength
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Fighter {
    base: Base,
}

impl Backstory for Fighter {}

impl Citations for Fighter {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 70)])
    }
}

impl Class for Fighter {
    fn gen(_: &mut impl Rng, ability_scores: &AbilityScores) -> Self {
        Self {
            base: if ability_scores.modifier(AbilityScoreType::Dexterity)
                > ability_scores.modifier(AbilityScoreType::Strength)
            {
                Base::Dexterity
            } else {
                Base::Strength
            },
        }
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Strength, AbilityScoreType::Dexterity],
            vec![
                AbilityScoreType::Constitution,
                AbilityScoreType::Intelligence,
            ],
        )
    }
}

impl Features for Fighter {}

impl Languages for Fighter {}

impl Pantheons for Fighter {}

impl Proficiencies for Fighter {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = ArmorType::iter().map(Proficiency::Armor).collect_vec();
        proficiencies.extend(
            WeaponCategory::iter().map(|c| Proficiency::Weapon(WeaponProficiency::Category(c))),
        );
        proficiencies.extend(vec![
            Proficiency::SavingThrow(AbilityScoreType::Constitution),
            Proficiency::SavingThrow(AbilityScoreType::Strength),
        ]);
        proficiencies
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(
            Some(vec![
                Skill::Acrobatics,
                Skill::AnimalHandling,
                Skill::Athletics,
                Skill::History,
                Skill::Insight,
                Skill::Intimidation,
                Skill::Perception,
                Skill::Survival,
            ]),
            2,
        )]
    }
}

impl StartingEquipment for Fighter {
    fn equipment(&self) -> Vec<Equipment> {
        match self.base {
            Base::Strength => vec![Equipment::new(Item::Armor(Armor::ChainMail), 1)],
            Base::Dexterity => vec![
                Equipment::new(Item::Ammunition(Ammunition::Arrows), 20),
                Equipment::new(Item::Armor(Armor::Leather), 1),
                Equipment::new(Item::Gear(Gear::Other(OtherGear::Quiver)), 1),
                Equipment::new(Item::Weapon(Weapon::Longbow), 1),
            ],
        }
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
            EquipmentOption::From(
                vec![
                    Equipment::new(Item::Weapon(Weapon::CrossbowLight), 1),
                    Equipment::new(Item::Weapon(Weapon::Handaxe), 2),
                ],
                1,
            ),
            EquipmentOption::Pack(Some(vec![Pack::Dungeoneer, Pack::Explorer])),
        ]
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fighter")
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
        let class = Fighter::gen(&mut rng, &AbilityScores::default());
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Fighter::gen(&mut rng, &AbilityScores::default());
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Fighter::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Fighter {
            base: Base::Dexterity,
        };
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let class = Fighter {
            base: Base::Dexterity,
        };
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let class = Fighter {
            base: Base::Dexterity,
        };
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_equipment() {
        insta::assert_yaml_snapshot!(Base::iter()
            .map(|base| (Fighter { base }).equipment())
            .collect::<Vec<Vec<Equipment>>>());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let class = Fighter {
            base: Base::Dexterity,
        };
        insta::assert_yaml_snapshot!(class.addl_equipment());
    }
}
