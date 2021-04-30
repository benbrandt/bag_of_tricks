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
        proficiencies::{Proficiencies, Proficiency},
        Character,
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{guild_artisan::GuildArtisan, Background, Personality, PersonalityOptions};

const SKILLS: &[Skill] = &[Skill::Insight, Skill::Persuasion];

#[derive(Deserialize, Serialize)]
pub(crate) struct Courtier;

#[typetag::serde]
impl Background for Courtier {
    fn gen(rng: &mut impl Rng, _: &Character) -> (Box<dyn Background>, Personality) {
        (Box::new(Self), GuildArtisan::gen_personality(rng))
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for Courtier {}

impl Citations for Courtier {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 146)])
    }
}

impl Features for Courtier {
    fn features(&self) -> Vec<Feature> {
        // Your knowledge of how bureaucracies function lets you gain access to the records and inner workings of any noble court or government you encounter. You know who the movers and shakers are, whom to go to for the favors you seek, and what the current intrigues of interest in the group are.
        vec![Feature {
            title: "Court Functionary",
            citation: Citation(Book::Scag, 147),
        }]
    }
}

impl Languages for Courtier {
    fn addl_languages(&self) -> usize {
        2
    }
}

impl Proficiencies for Courtier {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }
}

impl StartingEquipment for Courtier {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 5)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Gear(Gear::Other(OtherGear::ClothesFine)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }
}

impl fmt::Display for Courtier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Courtier")
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
        let background = Courtier::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (background, _personality) = Courtier::gen(&mut rng, &Character::default());
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(Courtier::skills());
    }

    #[test]
    fn test_snapshot_citations() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let background = Courtier;
        insta::assert_yaml_snapshot!(background.equipment());
    }
}
