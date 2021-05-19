use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use gear::weapons::{WeaponCategory, WeaponType};
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
pub struct Monk;

impl Backstory for Monk {}

impl Citations for Monk {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 76)])
    }
}

impl Class for Monk {
    fn gen(_: &mut impl Rng) -> Self {
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
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortsword)),
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

impl StartingEquipment for Monk {}

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
        let class = Monk::gen(&mut rng);
        insta::assert_yaml_snapshot!(class);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let class = Monk::gen(&mut rng);
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
    fn test_snapshot_proficiences() {
        let class = Monk;
        insta::assert_yaml_snapshot!(class.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let class = Monk;
        insta::assert_yaml_snapshot!(class.addl_proficiencies());
    }
}
