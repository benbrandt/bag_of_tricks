use std::fmt;

use citation::{Book, Citation, CitationList, Citations};
use personality::PersonalityOptions;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    ability::Skill,
    backstory::Backstory,
    equipment::{
        adventuring_gear::{Gear, OtherGear},
        currency::Coin,
        vehicles::VehicleProficiency,
        Equipment, EquipmentOption, StartingEquipment,
    },
    features::{Feature, Features},
    languages::Languages,
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
    Character,
};

use super::{
    soldier::{BONDS, FLAWS, IDEALS, TRAITS},
    Background,
};

const SKILLS: &[Skill] = &[Skill::Athletics, Skill::Persuasion];

#[derive(Deserialize, Serialize)]
pub(crate) struct MercenaryVeteran;

#[typetag::serde]
impl Background for MercenaryVeteran {
    fn gen(_: &mut impl Rng, _: &Character) -> Box<dyn Background> {
        Box::new(Self)
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for MercenaryVeteran {}

impl Citations for MercenaryVeteran {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 152)])
    }
}

impl Features for MercenaryVeteran {
    fn features(&self) -> Vec<Feature> {
        // You know the mercenary life as only someone who has experienced it can. You are able to identify mercenary companies by their emblems, and you know a little about any such company, including the names and reputations of its commanders and leaders, and who has hired them recently. You can find the taverns and festhalls where mercenaries abide in any area, as long as you speak the language. You can find mercenary work between adventures sufficient to maintain a comfortable lifestyle.
        vec![Feature {
            title: "Mercenary Life",
            citation: Citation(Book::Scag, 152),
        }]
    }
}

impl Languages for MercenaryVeteran {}

impl PersonalityOptions for MercenaryVeteran {
    fn bonds(&self) -> Vec<String> {
        BONDS.iter().map(|&s| s.to_string()).collect()
    }

    fn flaws(&self) -> Vec<String> {
        FLAWS.iter().map(|&s| s.to_string()).collect()
    }

    fn ideals(&self) -> Vec<(String, personality::Influence)> {
        IDEALS.iter().map(|&(s, i)| (s.to_string(), i)).collect()
    }

    fn traits(&self) -> Vec<String> {
        TRAITS.iter().map(|&s| s.to_string()).collect()
    }
}

impl Proficiencies for MercenaryVeteran {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = SKILLS
            .iter()
            .map(|&s| Proficiency::Skill(s))
            .collect::<Vec<_>>();
        proficiencies.push(Proficiency::Vehicle(VehicleProficiency::Land));
        proficiencies
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::GamingSet]
    }
}

impl StartingEquipment for MercenaryVeteran {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 10)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Other("an insignia of your rank".to_string()),
            Equipment::Gear(Gear::Other(OtherGear::ClothesTravelers)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![EquipmentOption::GamingSet]
    }
}

impl fmt::Display for MercenaryVeteran {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mercenary Veteran")
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
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.traits());
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(MercenaryVeteran::skills());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiencies() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = MercenaryVeteran::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.addl_equipment());
    }
}
