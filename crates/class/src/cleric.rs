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
pub struct Cleric;

impl Backstory for Cleric {}

impl Citations for Cleric {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 56)])
    }
}

impl Class for Cleric {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Wisdom],
            vec![AbilityScoreType::Strength, AbilityScoreType::Constitution],
        )
    }
}

impl Features for Cleric {}

impl Languages for Cleric {}

impl Pantheons for Cleric {
    fn deity_required(&self) -> bool {
        true
    }
}

impl Proficiencies for Cleric {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Armor(ArmorType::Light),
            Proficiency::Armor(ArmorType::Medium),
            Proficiency::Armor(ArmorType::Shield),
            Proficiency::SavingThrow(AbilityScoreType::Charisma),
            Proficiency::SavingThrow(AbilityScoreType::Wisdom),
            Proficiency::Weapon(WeaponProficiency::Category(WeaponCategory::Simple)),
        ]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(
            Some(vec![
                Skill::History,
                Skill::Insight,
                Skill::Medicine,
                Skill::Persuasion,
                Skill::Religion,
            ]),
            2,
        )]
    }
}

impl StartingEquipment for Cleric {}

impl fmt::Display for Cleric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cleric")
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
        let class = Cleric::gen(&mut rng);
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Cleric::gen(&mut rng);
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Cleric::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Cleric;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_deity_required() {
        let class = Cleric;
        insta::assert_yaml_snapshot!(class.deity_required());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let class = Cleric;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let class = Cleric;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }
}
