use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::Resistances;
use backstory::Backstory;
use characteristics::{
    in_inches, names::Name, AgeRange, Appearance, CharacteristicDetails, Characteristics,
    HeightAndWeightTable, Size, Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use deities::{Pantheon, PantheonWeight, Pantheons};
use dice_roller::{Die, RollCmd};
use features::{Feature, Features};
use languages::{Language, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScore, AbilityScoreType, Skill},
    proficiencies::{Proficiencies, Proficiency},
};
use trinkets::Trinkets;

use super::{
    human::Human,
    orc::{Orc, BONDS, FLAWS, IDEALS, TRAITS},
    Race,
};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 10),
    base_weight: 140,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
};

#[derive(Default, Deserialize, Serialize)]
pub struct HalfOrc;

impl AlignmentInfluences for HalfOrc {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Evil]
    }
}

impl Appearance for HalfOrc {}

impl Backstory for HalfOrc {}

impl Characteristics for HalfOrc {
    fn get_age_range(&self) -> AgeRange {
        AgeRange(8..=75)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }

    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn has_human_ancestry(&self) -> bool {
        true
    }
}

impl Citations for HalfOrc {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 40)])
    }
}

impl Features for HalfOrc {
    fn features(&self) -> Vec<Feature> {
        vec![
            // Thanks to your orc blood, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Phb, 41),
            },
            // When you are reduced to 0 hit points but not killed outright, you can drop to 1 hit point instead. You can't use this feature again until you finish a long rest.
            Feature {
                title: "Relentless Endurance",
                citation: Citation(Book::Phb, 41),
            },
            // When you score a critical hit with a melee weapon attack, you can roll one of the weapon's damage dice one additional time and add it to the extra damage of the critical hit.
            Feature {
                title: "Savage Attacks",
                citation: Citation(Book::Phb, 41),
            },
        ]
    }
}

impl Languages for HalfOrc {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Orc]
    }
}

impl Name for HalfOrc {
    /// First name can be either orc or human name
    fn gen_name(&self, rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        let first_name = *[
            Human::gen_first_name(rng, characteristics),
            Orc::gen_first_name(rng, characteristics),
        ]
        .choose(rng)
        .unwrap();
        let last_name = *[
            Human::gen_surname(rng, characteristics),
            Orc::gen_epithet(rng),
        ]
        .choose(rng)
        .unwrap();
        format!("{} {}", first_name, last_name)
    }
}

impl Pantheons for HalfOrc {
    fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {
        vec![(Pantheon::Orc, PantheonWeight::Likely)]
    }
}

impl PersonalityOptions for HalfOrc {
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

impl Proficiencies for HalfOrc {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![Proficiency::Skill(Skill::Intimidation)]
    }
}

impl Race for HalfOrc {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Constitution, 1),
            AbilityScore(AbilityScoreType::Strength, 2),
        ]
    }
}

impl Resistances for HalfOrc {}

impl Trinkets for HalfOrc {}

impl fmt::Display for HalfOrc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Half-Orc")
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
        let half_orc = HalfOrc::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_orc);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let half_orc = HalfOrc::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", half_orc));
    }

    #[test]
    fn test_snapshot_abilities() {
        let half_orc = HalfOrc;
        insta::assert_yaml_snapshot!(half_orc.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let half_orc = HalfOrc;
        insta::assert_yaml_snapshot!(half_orc.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let half_orc = HalfOrc;
        insta::assert_yaml_snapshot!(half_orc.features());
    }

    #[test]
    fn test_snapshot_addl_pantheons() {
        let half_orc = HalfOrc;
        insta::assert_yaml_snapshot!(half_orc.addl_pantheons());
    }

    #[test]
    fn test_bonds() {
        let half_orc = HalfOrc;
        insta::assert_yaml_snapshot!(half_orc.bonds());
    }

    #[test]
    fn test_flaws() {
        let half_orc = HalfOrc;
        insta::assert_yaml_snapshot!(half_orc.flaws());
    }

    #[test]
    fn test_ideals() {
        let half_orc = HalfOrc;
        insta::assert_yaml_snapshot!(half_orc.ideals());
    }

    #[test]
    fn test_traits() {
        let half_orc = HalfOrc;
        insta::assert_yaml_snapshot!(half_orc.traits());
    }
}
