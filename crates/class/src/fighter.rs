use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::{armor::ArmorType, weapons::WeaponCategory};
use itertools::Itertools;
use languages::Languages;
use rand::Rng;
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScoreType, Skill},
    equipment::StartingEquipment,
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption, WeaponProficiency},
};
use strum::IntoEnumIterator;

use super::Class;

#[derive(Default, Deserialize, Serialize)]
pub struct Fighter;

impl Backstory for Fighter {}

impl Citations for Fighter {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 70)])
    }
}

impl Class for Fighter {
    fn gen(_: &mut impl Rng) -> Self {
        Self
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

impl StartingEquipment for Fighter {}

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
        let class = Fighter::gen(&mut rng);
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Fighter::gen(&mut rng);
        insta::assert_display_snapshot!(class);
    }

    #[test]
    fn test_ability_rank() {
        insta::assert_yaml_snapshot!(Fighter::ability_rank());
    }

    #[test]
    fn test_snapshot_citations() {
        let class = Fighter;
        insta::assert_yaml_snapshot!(class.citations());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let class = Fighter;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let class = Fighter;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }
}
