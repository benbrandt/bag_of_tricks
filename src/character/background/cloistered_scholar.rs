use std::fmt;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    character::{
        ability::Skill,
        backstory::Backstory,
        equipment::{
            adventuring_gear::{Gear, OtherGear},
            currency::Coin,
            Equipment, StartingEquipment,
        },
        features::{Feature, Features},
        languages::Languages,
        proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
        Character,
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{sage::Sage, Background, Personality, PersonalityOptions};

const SKILLS: &[Skill] = &[Skill::History];
const ADDL_SKILLS: &[Skill] = &[Skill::Arcana, Skill::Nature, Skill::Religion];

#[derive(Deserialize, Serialize)]
pub(crate) struct CloisteredScholar;

#[typetag::serde]
impl Background for CloisteredScholar {
    fn gen(rng: &mut impl Rng, _: &Character) -> (Box<dyn Background>, Personality) {
        (Box::new(Self), Sage::gen_personality(rng))
    }

    fn skills() -> Vec<Skill> {
        let mut skills = SKILLS.to_vec();
        skills.extend(ADDL_SKILLS);
        skills
    }
}

impl Backstory for CloisteredScholar {}

impl Citations for CloisteredScholar {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 146)])
    }
}

impl Features for CloisteredScholar {
    fn features(&self) -> Vec<Feature> {
        // Though others must often endure extensive interviews and significant fees to gain access to even the most common archives in your library, you have free and easy access to the majority of the library, though it might also have repositories of lore that are too valuable, magical, or secret to permit anyone immediate access.
        // You have a working knowledge of your cloister's personnel and bureaucracy, and you know how to navigate those connections with some ease.
        // Additionally, you are likely to gain preferential treatment at other libraries across the Realms, as professional courtesy shown to a fellow scholar.
        vec![Feature {
            title: "Library Access",
            citation: Citation(Book::Scag, 146),
        }]
    }
}

impl Languages for CloisteredScholar {
    fn addl_languages(&self) -> usize {
        2
    }
}

impl Proficiencies for CloisteredScholar {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(Some(ADDL_SKILLS.to_vec()), 1)]
    }
}

impl StartingEquipment for CloisteredScholar {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 10)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Other("the scholar's robes of your cloister".to_string()),
            Equipment::Other("a writing kit (small pouch with a quill, a bottle of ink, folded parchment, and a small penknife)".to_string()),
            Equipment::Other("a borrowed book on the subject of your current study".to_string()),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }
}

impl fmt::Display for CloisteredScholar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cloistered Scholar")
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
        let background = CloisteredScholar::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = CloisteredScholar::gen(&mut rng, &Character::default());
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(CloisteredScholar::skills());
    }

    #[test]
    fn test_snapshot_citations() {
        let background = CloisteredScholar;
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let background = CloisteredScholar;
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let background = CloisteredScholar;
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let background = CloisteredScholar;
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let background = CloisteredScholar;
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let background = CloisteredScholar;
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let background = CloisteredScholar;
        insta::assert_yaml_snapshot!(background.equipment());
    }
}
