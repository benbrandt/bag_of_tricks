use std::fmt;

use citation::{Book, Citation, CitationList, Citations};
use languages::{LanguageType, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::{
    ability::Skill,
    backstory::Backstory,
    equipment::{
        adventuring_gear::{Gear, OtherGear},
        currency::Coin,
        Equipment, StartingEquipment,
    },
    features::{Feature, Features},
    proficiencies::{Proficiencies, Proficiency},
    Character,
};

use super::{
    max_skill_weight,
    soldier::{BONDS, FLAWS, IDEALS, TRAITS},
    Background,
};

#[derive(Copy, Clone, Deserialize, Display, EnumIter, Serialize)]
enum Variant {
    #[strum(serialize = "City Watch")]
    CityWatch,
    Investigator,
}

impl Variant {
    fn gen(rng: &mut impl Rng, character: &Character) -> Self {
        *Self::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |v| v.weight(character))
            .unwrap()
    }

    fn skills(self) -> Vec<Skill> {
        match self {
            Variant::CityWatch => vec![Skill::Athletics, Skill::Insight],
            Variant::Investigator => vec![Skill::Insight, Skill::Investigation],
        }
    }

    fn weight(self, character: &Character) -> f64 {
        max_skill_weight(&self.skills(), character)
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CityWatch {
    variant: Variant,
}

#[typetag::serde]
impl Background for CityWatch {
    fn gen(rng: &mut impl Rng, character: &Character) -> Box<dyn Background> {
        Box::new(Self {
            variant: Variant::gen(rng, character),
        })
    }

    fn skills() -> Vec<Skill> {
        vec![Skill::Athletics, Skill::Insight, Skill::Investigation]
    }

    fn weight(character: &Character) -> f64 {
        Variant::iter()
            .map(|v| v.weight(character))
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }
}

impl Backstory for CityWatch {}

impl Citations for CityWatch {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 145)])
    }
}

impl Features for CityWatch {
    fn features(&self) -> Vec<Feature> {
        // Your experience in enforcing the law, and dealing with lawbreakers, gives you a feel for local laws and criminals. You can easily find the local outpost of the watch or a similar organization, and just as easily pick out the dens of criminal activity in a community, although you're more likely to be welcome in the former locations rather than the latter.
        vec![Feature {
            title: "Watcher's Eye",
            citation: Citation(Book::Scag, 145),
        }]
    }
}

impl Languages for CityWatch {
    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (2, None)
    }
}

impl PersonalityOptions for CityWatch {
    fn bonds(&self) -> Vec<String> {
        BONDS.iter().map(|&s| s.to_string()).collect()
    }

    fn flaws(&self) -> Vec<String> {
        FLAWS.iter().map(|&s| s.to_string()).collect()
    }

    fn ideals(&self) -> Vec<(String, Influence)> {
        IDEALS.iter().map(|&(s, i)| (s.to_string(), i)).collect()
    }

    fn traits(&self) -> Vec<String> {
        TRAITS.iter().map(|&s| s.to_string()).collect()
    }
}

impl Proficiencies for CityWatch {
    fn proficiencies(&self) -> Vec<Proficiency> {
        self.variant
            .skills()
            .iter()
            .map(|&s| Proficiency::Skill(s))
            .collect()
    }
}

impl StartingEquipment for CityWatch {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 10)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Other(
                "A uniform in the style of your unit and indicative of your rank".into(),
            ),
            Equipment::Other("a horn with which to summon help".into()),
            Equipment::Gear(Gear::Other(OtherGear::Manacles)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }
}

impl fmt::Display for CityWatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.variant)
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
        let background = CityWatch::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = CityWatch::gen(&mut rng, &Character::default());
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = CityWatch::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = CityWatch::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = CityWatch::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = CityWatch::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.traits());
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(CityWatch::skills());
    }

    #[test]
    fn test_weight() {
        insta::assert_yaml_snapshot!(CityWatch::weight(&Character::default()));
    }

    #[test]
    fn test_snapshot_citations() {
        let background = CityWatch {
            variant: Variant::CityWatch,
        };
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let background = CityWatch {
            variant: Variant::CityWatch,
        };
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let background = CityWatch {
            variant: Variant::CityWatch,
        };
        insta::assert_yaml_snapshot!(background.addl_languages());
    }

    #[test]
    fn test_snapshot_proficiences() {
        insta::assert_yaml_snapshot!(Variant::iter()
            .map(|variant| CityWatch { variant }.proficiencies())
            .collect::<Vec<Vec<Proficiency>>>());
    }

    #[test]
    fn test_snapshot_coins() {
        let background = CityWatch {
            variant: Variant::CityWatch,
        };
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let background = CityWatch {
            variant: Variant::CityWatch,
        };
        insta::assert_yaml_snapshot!(background.equipment());
    }
}
