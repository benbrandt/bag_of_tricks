use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{human::Human, Race};
use crate::{character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores, Skill},
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, WeightMod,
        },
        features::{Feature, Features},
        languages::Language,
        names::{
            human::Names,
            orc::{FEMALE, MALE},
            Name,
        },
        proficiencies::Proficiency,
    }, citation::{Book, Citation, CitationList, Citations}, dice_roller::{Die, RollCmd}};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 10),
    base_weight: 140,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct HalfOrc;

impl Characteristics for HalfOrc {
    const AGE_RANGE: AgeRange = AgeRange(1..=75);
    const SIZE: Size = Size::Medium;

    fn get_base_speed(&self) -> u8 {
        30
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for HalfOrc {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation {
            book: Book::PHB,
            page: 40,
        }])
    }
}

impl Features for HalfOrc {
    fn features(&self) -> Vec<Feature> {
        vec![
            Feature {
                title: "Alignment",
                description: "Half-orcs inherit a tendency toward chaos from their orc parents and are not strongly inclined toward good. Half-orcs raised among orcs and willing to live out their lives among them are usually evil.",
                citation: Citation {
                    book: Book::PHB,
                    page: 41,
                },
            },
            Feature {
                title: "Darkvision",
                description: "Thanks to your orc blood, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
                citation: Citation {
                    book: Book::PHB,
                    page: 41,
                },
            },
            Feature {
                title: "Relentless Endurance",
                description: "When you are reduced to 0 hit points but not killed outright, you can drop to 1 hit point instead. You can't use this feature again until you finish a long rest.",
                citation: Citation {
                    book: Book::PHB,
                    page: 41,
                },
            },
            Feature {
                title: "Savage Attacks",
                description: "When you score a critical hit with a melee weapon attack, you can roll one of the weapon's damage dice one additional time and add it to the extra damage of the critical hit.",
                citation: Citation {
                    book: Book::PHB,
                    page: 41,
                },
            },
        ]
    }
}

impl Name for HalfOrc {
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        let names = Names::gen_names(rng);
        let orc_names = match characteristics.gender {
            Gender::Female => FEMALE,
            Gender::Male => MALE,
        };
        let first_name = *[
            Human::gen_first_name(rng, &names, characteristics),
            *orc_names.iter().choose(rng).unwrap(),
        ]
        .iter()
        .choose(rng)
        .unwrap();
        format!("{} {}", first_name, Human::gen_surname(rng, &names))
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

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Constitution, 1),
            AbilityScore(AbilityScoreType::Strength, 2),
        ])
    }

    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Orc]
    }

    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![Proficiency::Skill(Skill::Intimidation)]
    }
}

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
