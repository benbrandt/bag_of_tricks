use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::{
    armor::ArmorType,
    tools::Tool,
    weapons::{WeaponCategory, WeaponType},
};
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
pub struct Rogue;

impl Backstory for Rogue {}

impl Citations for Rogue {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 94)])
    }
}

impl Class for Rogue {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Dexterity],
            vec![AbilityScoreType::Charisma, AbilityScoreType::Intelligence],
        )
    }
}

impl Features for Rogue {}

impl Languages for Rogue {}

impl Pantheons for Rogue {}

impl Proficiencies for Rogue {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![
            Proficiency::Armor(ArmorType::Light),
            Proficiency::SavingThrow(AbilityScoreType::Dexterity),
            Proficiency::SavingThrow(AbilityScoreType::Intelligence),
            Proficiency::Tool(Tool::ThievesTools),
            Proficiency::Weapon(WeaponProficiency::Category(WeaponCategory::Simple)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::CrossbowHand)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Longsword)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Rapier)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortsword)),
        ]
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(
            Some(vec![
                Skill::Acrobatics,
                Skill::Athletics,
                Skill::Deception,
                Skill::Insight,
                Skill::Intimidation,
                Skill::Investigation,
                Skill::Perception,
                Skill::Performance,
                Skill::Persuasion,
                Skill::SleightOfHand,
                Skill::Stealth,
            ]),
            4,
        )]
    }
}

impl StartingEquipment for Rogue {}

impl fmt::Display for Rogue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rogue")
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
        let class = Rogue::gen(&mut rng);
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Rogue::gen(&mut rng);
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Rogue::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Rogue;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let class = Rogue;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let class = Rogue;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }
}
