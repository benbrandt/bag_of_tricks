use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{human::Human, orc::Orc, Race};
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
        names::{human::Names, Name},
        proficiencies::{Proficiencies, Proficiency},
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 10),
    base_weight: 140,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct HalfOrc;

impl AlignmentInfluences for HalfOrc {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Evil]
    }
}

impl Backstory for HalfOrc {}

impl Characteristics for HalfOrc {
    const AGE_RANGE: AgeRange = AgeRange(8..=75);
    const SIZE: Size = Size::Medium;

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
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
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        let names = Names::gen_names(rng);
        let first_name = *[
            Human::gen_first_name(rng, &names, characteristics),
            Orc::gen_first_name(rng, characteristics),
        ]
        .choose(rng)
        .unwrap();
        let last_name = *[Human::gen_surname(rng, &names), Orc::gen_epithet(rng)]
            .choose(rng)
            .unwrap();
        format!("{} {}", first_name, last_name)
    }
}

impl Proficiencies for HalfOrc {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![Proficiency::Skill(Skill::Intimidation)]
    }
}

#[typetag::serde]
impl Race for HalfOrc {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self);
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Constitution, 1),
            AbilityScore(AbilityScoreType::Strength, 2),
        ]
    }
}

impl Resistances for HalfOrc {}

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
        let (half_orc, _name, _characteristics) = HalfOrc::gen(&mut rng);
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
}
