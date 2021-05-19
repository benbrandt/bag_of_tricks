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
pub struct Warlock;

impl Backstory for Warlock {}

impl Citations for Warlock {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 105)])
    }
}

impl Class for Warlock {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Charisma],
            vec![AbilityScoreType::Constitution],
        )
    }
}

impl Features for Warlock {}

impl Languages for Warlock {}

impl Pantheons for Warlock {}

impl Proficiencies for Warlock {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Armor(ArmorType::Light),
            Proficiency::SavingThrow(AbilityScoreType::Charisma),
            Proficiency::SavingThrow(AbilityScoreType::Wisdom),
            Proficiency::Weapon(WeaponProficiency::Category(WeaponCategory::Simple)),
        ]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(
            Some(vec![
                Skill::Arcana,
                Skill::Deception,
                Skill::History,
                Skill::Intimidation,
                Skill::Investigation,
                Skill::Nature,
                Skill::Religion,
            ]),
            2,
        )]
    }
}

impl StartingEquipment for Warlock {}

impl fmt::Display for Warlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Warlock")
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
        let class = Warlock::gen(&mut rng);
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Warlock::gen(&mut rng);
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Warlock::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Warlock;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let class = Warlock;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let class = Warlock;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }
}
