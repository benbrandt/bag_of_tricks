use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::weapons::Weapon;
use languages::Languages;
use rand::Rng;
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScoreType, Skill},
    equipment::StartingEquipment,
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
    fn gen(_: &mut impl Rng) -> Self {
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

impl StartingEquipment for Wizard {}

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
        let class = Wizard::gen(&mut rng);
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Wizard::gen(&mut rng);
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
    fn test_snapshot_proficiences() {
        let class = Wizard;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let class = Wizard;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }
}
