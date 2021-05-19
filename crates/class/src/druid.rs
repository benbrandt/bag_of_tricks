use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::{armor::ArmorType, tools::Tool, weapons::WeaponType};
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
pub struct Druid;

impl Backstory for Druid {}

impl Citations for Druid {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 64)])
    }
}

impl Class for Druid {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Wisdom],
            vec![AbilityScoreType::Constitution],
        )
    }
}

impl Features for Druid {}

impl Languages for Druid {}

impl Pantheons for Druid {}

impl Proficiencies for Druid {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Armor(ArmorType::Light),
            Proficiency::Armor(ArmorType::Medium),
            Proficiency::Armor(ArmorType::Shield),
            Proficiency::SavingThrow(AbilityScoreType::Intelligence),
            Proficiency::SavingThrow(AbilityScoreType::Wisdom),
            Proficiency::Tool(Tool::HerbalismKit),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Club)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Dagger)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Dart)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Javelin)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Mace)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Quarterstaff)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Scimitar)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Sickle)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Sling)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Spear)),
        ]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(
            Some(vec![
                Skill::Arcana,
                Skill::AnimalHandling,
                Skill::Insight,
                Skill::Medicine,
                Skill::Nature,
                Skill::Perception,
                Skill::Religion,
                Skill::Survival,
            ]),
            2,
        )]
    }
}

impl StartingEquipment for Druid {}

impl fmt::Display for Druid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Druid")
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
        let class = Druid::gen(&mut rng);
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Druid::gen(&mut rng);
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Druid::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Druid;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let class = Druid;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let class = Druid;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }
}
