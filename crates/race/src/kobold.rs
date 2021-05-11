use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::Resistances;
use backstory::{Backstory, MONSTROUS_ORIGIN};
use characteristics::{
    in_inches,
    names::{kobold::NAMES, Name},
    AgeRange, Appearance, CharacteristicDetails, Characteristics, HeightAndWeightTable, Size,
    Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use dice_roller::{Die, RollCmd};
use features::{Feature, Features};
use languages::{Language, Languages};
use personality::PersonalityOptions;
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScore, AbilityScoreType},
    proficiencies::Proficiencies,
};
use strum::{Display, EnumIter, IntoEnumIterator};
use trinkets::Trinkets;

use super::Race;

#[derive(Clone, Copy, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum ScaleColor {
    Black,
    Blue,
    Brown,
    Gray,
    Green,
    Orange,
    #[strum(serialize = "Orange-brown")]
    OrangeBrown,
    Red,
    #[strum(serialize = "Red-brown")]
    RedBrown,
    Tan,
    White,
    Patterned,
}

impl ScaleColor {
    fn pattern(rng: &mut impl Rng) -> Vec<Self> {
        Self::iter()
            .filter(|c| c != &Self::Patterned)
            .collect::<Vec<_>>()
            .choose_multiple_weighted(rng, 2, |c| c.weight())
            .unwrap()
            .copied()
            .collect::<Vec<_>>()
    }

    /// VGTM 66
    fn weight(self) -> u8 {
        match self {
            Self::Black
            | Self::Blue
            | Self::Gray
            | Self::Green
            | Self::Orange
            | Self::Red
            | Self::White => 5,
            Self::Brown | Self::OrangeBrown | Self::RedBrown => 15,
            Self::Tan | Self::Patterned => 10,
        }
    }

    /// VGTM 66 Patterned (roll twice, ignoring duplicate results)
    fn gen(rng: &mut impl Rng) -> Vec<Self> {
        let color = *Self::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |c| c.weight())
            .unwrap();
        match color {
            Self::Black
            | Self::Blue
            | Self::Brown
            | Self::Gray
            | Self::Green
            | Self::Orange
            | Self::OrangeBrown
            | Self::Red
            | Self::RedBrown
            | Self::Tan
            | Self::White => vec![color],
            Self::Patterned => Self::pattern(rng),
        }
    }
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum ScalePattern {
    Mottled,
    Reticulated,
    Shaded,
    Spotted,
    Striped,
}

impl ScalePattern {
    fn gen(rng: &mut impl Rng) -> Self {
        Self::iter().choose(rng).unwrap()
    }
}

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(2, 1),
    base_weight: 25,
    height_mod: RollCmd(2, Die::D4),
    weight_mod: WeightMod::Fixed(1),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Kobold {
    origin: String,
    scale_color: Vec<ScaleColor>,
    scale_pattern: ScalePattern,
}

impl AlignmentInfluences for Kobold {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Lawful]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Evil]
    }
}

impl Appearance for Kobold {
    fn appearance(&self) -> Vec<String> {
        vec![
            format!("Scale Pattern: {}", self.scale_pattern,),
            format!(
                "Scale Color: {}",
                self.scale_color
                    .iter()
                    .map(|c| format!("{}", c))
                    .collect::<Vec<_>>()
                    .join(" and ")
            ),
        ]
    }
}

impl Backstory for Kobold {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Origin: {}", self.origin)]
    }
}

impl Characteristics for Kobold {
    const SIZE: Size = Size::Small;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(3..=120)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Kobold {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 119)])
    }
}

impl Features for Kobold {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Vgtm, 119),
            },
            // As an action on your turn, you can cower pathetically to distract nearby foes. Until the end of your next turn, your allies gain advantage on attack rolls against enemies within 10 feet of you that can see you. Once you use this trait, you can’t use it again until you finish a short or long rest.
            Feature {
                title: "Grovel, Cower, and Beg",
                citation: Citation(Book::Vgtm, 119),
            },
            // You have advantage on an attack roll against a creature if at least one of your allies is within 5 feet of the creature and the ally isn’t incapacitated.
            Feature {
                title: "Pack Tactics",
                citation: Citation(Book::Vgtm, 119),
            },
            // You have disadvantage on attack rolls and on Wisdom (Perception) checks that rely on sight when you, the target of your attack, or whatever you are trying to perceive is in direct sunlight.
            Feature {
                title: "Sunlight Sensitivity",
                citation: Citation(Book::Vgtm, 119),
            },
        ]
    }
}

impl Languages for Kobold {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Draconic]
    }
}

impl Name for Kobold {
    fn gen_name(rng: &mut impl Rng, _: &CharacteristicDetails) -> String {
        (*NAMES.choose(rng).unwrap()).to_string()
    }
}

impl PersonalityOptions for Kobold {}

impl Proficiencies for Kobold {}

#[typetag::serde]
impl Race for Kobold {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            origin: (*MONSTROUS_ORIGIN.choose(rng).unwrap()).to_string(),
            scale_color: ScaleColor::gen(rng),
            scale_pattern: ScalePattern::gen(rng),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            AbilityScore(AbilityScoreType::Strength, -2),
        ]
    }
}

impl Resistances for Kobold {}

impl Trinkets for Kobold {}

impl fmt::Display for Kobold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Kobold")
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
        let kobold = Kobold::gen(&mut rng);
        insta::assert_yaml_snapshot!(kobold);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (kobold, _name, _characteristics) = Kobold::gen(&mut rng);
        insta::assert_display_snapshot!(kobold);
    }

    #[test]
    fn test_attitude() {
        let kobold = Kobold {
            origin: String::new(),
            scale_color: vec![ScaleColor::Black],
            scale_pattern: ScalePattern::Mottled,
        };
        insta::assert_yaml_snapshot!(kobold.attitude());
    }

    #[test]
    fn test_morality() {
        let kobold = Kobold {
            origin: String::new(),
            scale_color: vec![ScaleColor::Black],
            scale_pattern: ScalePattern::Mottled,
        };
        insta::assert_yaml_snapshot!(kobold.morality());
    }

    #[test]
    fn test_appearance() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (kobold, _name, _characteristics) = Kobold::gen(&mut rng);
        insta::assert_yaml_snapshot!(kobold.appearance());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (kobold, _name, _characteristics) = Kobold::gen(&mut rng);
        insta::assert_yaml_snapshot!(kobold.backstory());
    }

    #[test]
    fn test_characteristics() {
        let kobold = Kobold {
            origin: String::new(),
            scale_color: vec![ScaleColor::Black],
            scale_pattern: ScalePattern::Mottled,
        };
        assert_eq!(kobold.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(kobold.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (kobold, _name, _characteristics) = Kobold::gen(&mut rng);
        insta::assert_yaml_snapshot!(kobold.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let kobold = Kobold {
            origin: String::new(),
            scale_color: vec![ScaleColor::Black],
            scale_pattern: ScalePattern::Mottled,
        };
        insta::assert_yaml_snapshot!(kobold.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let kobold = Kobold {
            origin: String::new(),
            scale_color: vec![ScaleColor::Black],
            scale_pattern: ScalePattern::Mottled,
        };
        insta::assert_yaml_snapshot!(kobold.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let kobold = Kobold {
            origin: String::new(),
            scale_color: vec![ScaleColor::Black],
            scale_pattern: ScalePattern::Mottled,
        };
        let characteristics = kobold.gen_characteristics(&mut rng);
        let name = Kobold::gen_name(&mut rng, &characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_abilities() {
        let kobold = Kobold {
            origin: String::new(),
            scale_color: vec![ScaleColor::Black],
            scale_pattern: ScalePattern::Mottled,
        };
        insta::assert_yaml_snapshot!(kobold.abilities());
    }
}
