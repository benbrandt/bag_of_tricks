use std::fmt;

use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    alignment::{AlignmentInfluences, Attitude, Morality},
    character::{
        ability::{AbilityScore, AbilityScoreType, Skill},
        attack::Resistances,
        backstory::Backstory,
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, HeightAndWeightTable,
            Size, Speed, WeightMod,
        },
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{
            goliath::{BIRTH_NAMES, CLAN_NAMES, NICKNAMES},
            Name,
        },
        proficiencies::{Proficiencies, Proficiency},
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

use super::Race;

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(6, 2),
    base_weight: 200,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Goliath;

impl AlignmentInfluences for Goliath {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Lawful]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Neutral]
    }
}

impl Backstory for Goliath {}

impl Characteristics for Goliath {
    const AGE_RANGE: AgeRange = AgeRange(10..=100);
    const SIZE: Size = Size::Medium;

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
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
    fn gen_name(rng: &mut impl Rng, _: &CharacteristicDetails) -> String {
        format!(
            "{} \"{}\" {}",
            BIRTH_NAMES.choose(rng).unwrap(),
            NICKNAMES.choose(rng).unwrap(),
            CLAN_NAMES.choose(rng).unwrap(),
        )
    }
}

impl Proficiencies for Goliath {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![Proficiency::Skill(Skill::Athletics)]
    }
}

#[typetag::serde]
impl Race for Goliath {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self);
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Strength, 2),
            AbilityScore(AbilityScoreType::Constitution, 1),
        ]
    }
}

impl Resistances for Goliath {}

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
        let (goliath, _name, _characteristics) = Goliath::gen(&mut rng);
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
        let name = Goliath::gen_name(&mut rng, &&characteristics);
        insta::assert_yaml_snapshot!(name);
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
