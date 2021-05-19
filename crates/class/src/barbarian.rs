use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::{armor::ArmorType, weapons::WeaponCategory};
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
pub struct Barbarian;

impl Backstory for Barbarian {}

impl Citations for Barbarian {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 46)])
    }
}

impl Class for Barbarian {
    fn gen(_: &mut impl Rng) -> Self {
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

impl StartingEquipment for Barbarian {}

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
        let class = Barbarian::gen(&mut rng);
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Barbarian::gen(&mut rng);
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
    fn test_snapshot_proficiences() {
        let class = Barbarian;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let class = Barbarian;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }
}
