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
pub struct Ranger;

impl Backstory for Ranger {}

impl Citations for Ranger {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 89)])
    }
}

impl Class for Ranger {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Dexterity, AbilityScoreType::Strength],
            vec![AbilityScoreType::Wisdom],
        )
    }
}

impl Features for Ranger {}

impl Languages for Ranger {}

impl Pantheons for Ranger {}

impl Proficiencies for Ranger {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Armor(ArmorType::Light),
            Proficiency::Armor(ArmorType::Medium),
            Proficiency::Armor(ArmorType::Shield),
            Proficiency::SavingThrow(AbilityScoreType::Dexterity),
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
                Skill::Insight,
                Skill::Investigation,
                Skill::Nature,
                Skill::Perception,
                Skill::Stealth,
                Skill::Survival,
            ]),
            3,
        )]
    }
}

impl StartingEquipment for Ranger {}

impl fmt::Display for Ranger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ranger")
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
        let class = Ranger::gen(&mut rng);
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Ranger::gen(&mut rng);
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Ranger::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Ranger;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let class = Ranger;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let class = Ranger;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }
}
