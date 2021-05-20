use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::weapons::{Weapon, WeaponCategory};
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
pub struct Monk;

impl Backstory for Monk {}

impl Citations for Monk {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 76)])
    }
}

impl Class for Monk {
    fn gen(_: &mut impl Rng, _: &AbilityScores) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Dexterity],
            vec![AbilityScoreType::Wisdom],
        )
    }
}

impl Features for Monk {}

impl Languages for Monk {}

impl Pantheons for Monk {}

impl Proficiencies for Monk {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::SavingThrow(AbilityScoreType::Dexterity),
            Proficiency::SavingThrow(AbilityScoreType::Strength),
            Proficiency::Weapon(WeaponProficiency::Category(WeaponCategory::Simple)),
            Proficiency::Weapon(WeaponProficiency::Specific(Weapon::Shortsword)),
        ]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![
            ProficiencyOption::FromOptions(
                vec![
                    ProficiencyOption::ArtisansTools,
                    ProficiencyOption::MusicalInstrument(1),
                ],
                1,
            ),
            ProficiencyOption::Skill(
                Some(vec![
                    Skill::Acrobatics,
                    Skill::Athletics,
                    Skill::History,
                    Skill::Insight,
                    Skill::Religion,
                    Skill::Stealth,
                ]),
                2,
            ),
        ]
    }
}

impl StartingEquipment for Monk {
    fn equipment(&self) -> Vec<Equipment> {
        vec![Equipment::new(Item::Weapon(Weapon::Dart), 10)]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![
            EquipmentOption::FromOptions(
                vec![
                    EquipmentOption::From(
                        vec![Equipment::new(Item::Weapon(Weapon::Shortsword), 1)],
                        1,
                    ),
                    EquipmentOption::Weapon(Some(WeaponCategory::Simple), None, 1),
                ],
                1,
            ),
            EquipmentOption::Pack(Some(vec![Pack::Dungeoneer, Pack::Explorer])),
        ]
    }
}

impl fmt::Display for Monk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monk")
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
        let class = Monk::gen(&mut rng, &AbilityScores::default());
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Monk::gen(&mut rng, &AbilityScores::default());
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Monk::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Monk;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let class = Monk;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let class = Monk;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_equipment() {
        let class = Monk;
        insta::assert_yaml_snapshot!(class.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let class = Monk;
        insta::assert_yaml_snapshot!(class.addl_equipment());
    }
}
