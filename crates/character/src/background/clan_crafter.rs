use std::fmt;

use citation::{Book, Citation, CitationList, Citations};
use languages::{Language, Languages};
use personality::{Influence, PersonalityOptions};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    ability::Skill,
    backstory::Backstory,
    equipment::{
        adventuring_gear::{Gear, OtherGear},
        currency::Coin,
        Equipment, EquipmentOption, StartingEquipment,
    },
    features::{Feature, Features},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
    Character,
};

use super::{
    guild_artisan::{BONDS, FLAWS, IDEALS, TRAITS},
    Background,
};

const SKILLS: &[Skill] = &[Skill::History, Skill::Insight];

#[derive(Deserialize, Serialize)]
pub(crate) struct ClanCrafter;

#[typetag::serde]
impl Background for ClanCrafter {
    fn gen(_: &mut impl Rng, _: &Character) -> Box<dyn Background> {
        Box::new(Self)
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for ClanCrafter {}

impl Citations for ClanCrafter {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Scag, 145)])
    }
}

impl Features for ClanCrafter {
    fn features(&self) -> Vec<Feature> {
        // As well respected as clan crafters are among outsiders, no one esteems them quite so highly as dwarves do. You always have free room and board in any place where shield dwarves or gold dwarves dwell, and the individuals in such a settlement might vie among themselves to determine who can offer you (and possibly your compatriots) the finest accommodations and assistance.
        vec![Feature {
            title: "Respect of the Stout Folk",
            citation: Citation(Book::Scag, 145),
        }]
    }
}

impl Languages for ClanCrafter {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Dwarvish]
    }
}

impl PersonalityOptions for ClanCrafter {
    fn bonds(&self) -> Vec<String> {
        BONDS
            .iter()
            .map(|&s| s.to_string().replace("guild", "clan"))
            .collect()
    }

    fn flaws(&self) -> Vec<String> {
        FLAWS
            .iter()
            .map(|&s| s.to_string().replace("guild", "clan"))
            .collect()
    }

    fn ideals(&self) -> Vec<(String, Influence)> {
        IDEALS
            .iter()
            .map(|&(s, i)| (s.to_string().replace("guild", "clan"), i))
            .collect()
    }

    fn traits(&self) -> Vec<String> {
        TRAITS
            .iter()
            .map(|&s| s.to_string().replace("guild", "clan"))
            .collect()
    }
}

impl Proficiencies for ClanCrafter {
    fn proficiencies(&self) -> Vec<Proficiency> {
        SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect()
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::ArtisansTools]
    }
}

impl StartingEquipment for ClanCrafter {
    // TODO: supposed to be 5gp + gem worth 10gp
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 15)
    }

    fn equipment(&self) -> Vec<Equipment> {
        vec![
            Equipment::Other("a maker's mark chisel used to mark your handiwork with the symbol of the clan of crafters you learned your skill from".into()),
            Equipment::Gear(Gear::Other(OtherGear::ClothesTravelers)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ]
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        vec![EquipmentOption::ArtisansTools]
    }
}

impl fmt::Display for ClanCrafter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Clan Crafter")
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
        let background = ClanCrafter::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = ClanCrafter::gen(&mut rng, &Character::default());
        insta::assert_display_snapshot!(background);
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = ClanCrafter::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = ClanCrafter::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = ClanCrafter::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let background = ClanCrafter::gen(&mut rng, &Character::default());
        insta::assert_yaml_snapshot!(background.traits());
    }

    #[test]
    fn test_skills() {
        insta::assert_yaml_snapshot!(ClanCrafter::skills());
    }

    #[test]
    fn test_snapshot_citations() {
        let background = ClanCrafter;
        insta::assert_yaml_snapshot!(background.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let background = ClanCrafter;
        insta::assert_yaml_snapshot!(background.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let background = ClanCrafter;
        insta::assert_yaml_snapshot!(background.languages());
    }

    #[test]
    fn test_snapshot_proficiences() {
        let background = ClanCrafter;
        insta::assert_yaml_snapshot!(background.proficiencies());
    }

    #[test]
    fn test_snapshot_addl_proficiences() {
        let background = ClanCrafter;
        insta::assert_yaml_snapshot!(background.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_coins() {
        let background = ClanCrafter;
        insta::assert_yaml_snapshot!(background.coins());
    }

    #[test]
    fn test_snapshot_equipment() {
        let background = ClanCrafter;
        insta::assert_yaml_snapshot!(background.equipment());
    }

    #[test]
    fn test_snapshot_addl_equipment() {
        let background = ClanCrafter;
        insta::assert_yaml_snapshot!(background.addl_equipment());
    }
}
