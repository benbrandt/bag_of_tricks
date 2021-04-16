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
        names::{kenku::NAMES, Name},
        proficiencies::{Proficiencies, ProficiencyOption},
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

use super::Race;

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 4),
    base_weight: 50,
    height_mod: RollCmd(2, Die::D8),
    weight_mod: WeightMod::Roll(RollCmd(1, Die::D6)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Kenku;

impl AlignmentInfluences for Kenku {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Neutral]
    }
}

impl Backstory for Kenku {}

impl Characteristics for Kenku {
    const SIZE: Size = Size::Medium;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(6..=60)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Kenku {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 109)])
    }
}

impl Features for Kenku {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can duplicate other creatures' handwriting and craftwork. You have advantage on all checks made to produce forgeries or duplicates of existing objects.
            Feature {
                title: "Expert Forgery",
                citation: Citation(Book::Vgtm, 111),
            },
            // You can mimic sounds you have heard, including voices. A creature that hears the sounds you make can tell they are imitations with a successful Wisdom (Insight) check opposed by your Charisma (Deception) check.
            Feature {
                title: "Mimicry",
                citation: Citation(Book::Vgtm, 111),
            },
        ]
    }
}

impl Languages for Kenku {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Auran]
    }
}

impl Name for Kenku {
    fn gen_name(rng: &mut impl Rng, _: &CharacteristicDetails) -> String {
        (*NAMES.choose(rng).unwrap()).to_string()
    }
}

impl Proficiencies for Kenku {
    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::Skill(
            Some(vec![
                Skill::Acrobatics,
                Skill::Deception,
                Skill::Stealth,
                Skill::SleightOfHand,
            ]),
            2,
        )]
    }
}

#[typetag::serde]
impl Race for Kenku {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self);
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            AbilityScore(AbilityScoreType::Wisdom, 1),
        ]
    }
}

impl Resistances for Kenku {}

impl fmt::Display for Kenku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Kenku")
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
        let kenku = Kenku::gen(&mut rng);
        insta::assert_yaml_snapshot!(kenku);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (kenku, _name, _characteristics) = Kenku::gen(&mut rng);
        insta::assert_display_snapshot!(kenku);
    }

    #[test]
    fn test_attitude() {
        let kenku = Kenku;
        insta::assert_yaml_snapshot!(kenku.attitude());
    }

    #[test]
    fn test_morality() {
        let kenku = Kenku;
        insta::assert_yaml_snapshot!(kenku.morality());
    }

    #[test]
    fn test_characteristics() {
        let kenku = Kenku;
        assert_eq!(kenku.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(kenku.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (kenku, _name, _characteristics) = Kenku::gen(&mut rng);
        insta::assert_yaml_snapshot!(kenku.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let kenku = Kenku;
        insta::assert_yaml_snapshot!(kenku.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let kenku = Kenku;
        insta::assert_yaml_snapshot!(kenku.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let kenku = Kenku;
        let characteristics = kenku.gen_characteristics(&mut rng);
        let name = Kenku::gen_name(&mut rng, &characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_proficiencies() {
        let kenku = Kenku;
        insta::assert_yaml_snapshot!(kenku.addl_proficiencies());
    }

    #[test]
    fn test_snapshot_abilities() {
        let kenku = Kenku;
        insta::assert_yaml_snapshot!(kenku.abilities());
    }
}
