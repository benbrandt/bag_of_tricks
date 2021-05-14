use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::Resistances;
use backstory::Backstory;
use characteristics::{
    in_inches,
    names::{
        goliath::{BIRTH_NAMES, CLAN_NAMES, NICKNAMES},
        Name,
    },
    AgeRange, Appearance, CharacteristicDetails, Characteristics, HeightAndWeightTable, Size,
    Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use deities::{Pantheon, PantheonWeight, Pantheons};
use dice_roller::{Die, RollCmd};
use features::{Feature, Features};
use languages::{Language, Languages};
use personality::PersonalityOptions;
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScore, AbilityScoreType, Skill},
    proficiencies::{Proficiencies, Proficiency},
};
use trinkets::Trinkets;

use super::Race;

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(6, 2),
    base_weight: 200,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
};

#[derive(Default, Deserialize, Serialize)]
pub struct Goliath;

impl AlignmentInfluences for Goliath {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Lawful]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Neutral]
    }
}

impl Appearance for Goliath {}

impl Backstory for Goliath {}

impl Characteristics for Goliath {
    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=100)
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
}

impl Citations for Goliath {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 108)])
    }
}

impl Features for Goliath {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can focus yourself to occasionally shrug off injury. When you take damage, you can use your reaction to roll a d12. Add your Constitution modifier to the number rolled, and reduce the damage by that total. After you use this trait, you can’t use it again until you finish a short or long rest.
            Feature {
                title: "Stone's Endurance",
                citation: Citation(Book::Vgtm, 109),
            },
            // You count as one size larger when determining your carrying capacity and the weight you can push, drag, or lift.
            Feature {
                title: "Powerful Build",
                citation: Citation(Book::Vgtm, 109),
            },
            // You are acclimated to high altitude, including elevations above 20,000 feet. You're also naturally adapted to cold climates, as described in chapter 5 of the Dungeon Master's Guide.
            Feature {
                title: "Mountain Born",
                citation: Citation(Book::Vgtm, 109),
            },
        ]
    }
}

impl Languages for Goliath {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Giant]
    }
}

impl Name for Goliath {
    fn gen_name(&self, rng: &mut impl Rng, _: &CharacteristicDetails) -> String {
        format!(
            "{} \"{}\" {}",
            BIRTH_NAMES.choose(rng).unwrap(),
            NICKNAMES.choose(rng).unwrap(),
            CLAN_NAMES.choose(rng).unwrap(),
        )
    }
}

impl Pantheons for Goliath {
    fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {
        vec![(Pantheon::Giant, PantheonWeight::Likely)]
    }
}

impl PersonalityOptions for Goliath {}

impl Proficiencies for Goliath {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![Proficiency::Skill(Skill::Athletics)]
    }
}

impl Race for Goliath {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Strength, 2),
            AbilityScore(AbilityScoreType::Constitution, 1),
        ]
    }
}

impl Resistances for Goliath {}

impl Trinkets for Goliath {}

impl fmt::Display for Goliath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Goliath")
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
        let goliath = Goliath::gen(&mut rng);
        insta::assert_yaml_snapshot!(goliath);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let goliath = Goliath::gen(&mut rng);
        insta::assert_display_snapshot!(goliath);
    }

    #[test]
    fn test_attitude() {
        let goliath = Goliath;
        insta::assert_yaml_snapshot!(goliath.attitude());
    }

    #[test]
    fn test_morality() {
        let goliath = Goliath;
        insta::assert_yaml_snapshot!(goliath.morality());
    }

    #[test]
    fn test_characteristics() {
        let goliath = Goliath;
        assert_eq!(goliath.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(goliath.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let goliath = Goliath;
        insta::assert_yaml_snapshot!(goliath.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let goliath = Goliath;
        insta::assert_yaml_snapshot!(goliath.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let goliath = Goliath;
        insta::assert_yaml_snapshot!(goliath.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let goliath = Goliath;
        let characteristics = goliath.gen_characteristics(&mut rng);
        let name = goliath.gen_name(&mut rng, &&characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_addl_pantheons() {
        let goliath = Goliath;
        insta::assert_yaml_snapshot!(goliath.addl_pantheons());
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let goliath = Goliath;
        insta::assert_yaml_snapshot!(goliath.proficiencies());
    }

    #[test]
    fn test_snapshot_abilities() {
        let goliath = Goliath;
        insta::assert_yaml_snapshot!(goliath.abilities());
    }
}
