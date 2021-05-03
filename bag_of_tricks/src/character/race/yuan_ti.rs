use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use citation::{Book, Citation, CitationList, Citations};
use dice_roller::{Die, RollCmd};
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::character::{
    ability::{AbilityScore, AbilityScoreType},
    attack::{DamageType, Resistances},
    backstory::Backstory,
    characteristics::{
        in_inches, AgeRange, Appearance, CharacteristicDetails, Characteristics,
        HeightAndWeightTable, Size, Speed, WeightMod,
    },
    features::{Feature, Features},
    languages::{Language, Languages},
    names::{yuan_ti::NAMES, Name},
    proficiencies::Proficiencies,
};

use super::{human::Human, origins::MONSTROUS_ORIGIN, Race};

#[derive(Clone, Copy, Deserialize, Display, EnumIter, Serialize)]
enum SkinColor {
    #[strum(serialize = "Dark brown")]
    Dark,
    #[strum(serialize = "Green-brown")]
    Green,
    #[strum(serialize = "Light brown")]
    Light,
    #[strum(serialize = "Medium brown")]
    Medium,
    #[strum(serialize = "Pale brown")]
    Pale,
    #[strum(serialize = "Red-brown")]
    Red,
    #[strum(serialize = "Yellow-brown")]
    Yellow,
}

impl SkinColor {
    fn weight(self) -> u8 {
        match self {
            Self::Dark | Self::Light => 4,
            Self::Green | Self::Pale => 1,
            Self::Medium => 6,
            Self::Red | Self::Yellow => 2,
        }
    }

    fn gen(rng: &mut impl Rng) -> Self {
        *Self::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |c| c.weight())
            .unwrap()
    }
}

#[derive(Clone, Copy, Deserialize, Display, EnumIter, Serialize)]
enum ScaleColor {
    Black,
    #[strum(serialize = "Black and brown")]
    BlackAndBrown,
    #[strum(serialize = "Black and green")]
    BlackAndGreen,
    #[strum(serialize = "Black and red")]
    BlackAndRed,
    #[strum(serialize = "Black and white")]
    BlackAndWhite,
    #[strum(serialize = "Black and yellow")]
    BlackAndYellow,
    #[strum(serialize = "Black, gold, and red")]
    BlackGoldAndRed,
    #[strum(serialize = "Black, red, and white")]
    BlackRedAndWhite,
    Blue,
    #[strum(serialize = "Blue and black")]
    BlueAndBlack,
    #[strum(serialize = "Blue and gray")]
    BlueAndGray,
    #[strum(serialize = "Blue and yellow")]
    BlueAndYellow,
    Brown,
    #[strum(serialize = "Brown and green")]
    BrownAndGreen,
    Green,
    #[strum(serialize = "Green and tan")]
    GreenAndTan,
    #[strum(serialize = "Green and white")]
    GreenAndWhite,
    #[strum(serialize = "Green and yellow")]
    GreenAndYellow,
    #[strum(serialize = "Red and tan")]
    RedAndTan,
    Albino,
}

impl ScaleColor {
    fn weight(self) -> u8 {
        match self {
            Self::Black
            | Self::BlackAndBrown
            | Self::BlackAndGreen
            | Self::BlackGoldAndRed
            | Self::BlackRedAndWhite
            | Self::Brown
            | Self::BrownAndGreen
            | Self::GreenAndTan
            | Self::GreenAndYellow
            | Self::RedAndTan => 6,
            Self::BlackAndRed | Self::GreenAndWhite => 5,
            Self::BlackAndWhite
            | Self::Blue
            | Self::BlueAndBlack
            | Self::BlueAndGray
            | Self::BlueAndYellow => 3,
            Self::BlackAndYellow | Self::Albino => 4,
            Self::Green => 7,
        }
    }

    fn gen(rng: &mut impl Rng) -> Self {
        *Self::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |c| c.weight())
            .unwrap()
    }
}

#[derive(Clone, Copy, Deserialize, Display, EnumIter, Serialize)]
enum ScalePattern {
    Mottled,
    Random,
    Reticulated,
    Speckled,
    Striped,
}

impl ScalePattern {
    fn weight(self) -> u8 {
        match self {
            Self::Mottled | Self::Speckled | Self::Striped => 5,
            Self::Random => 2,
            Self::Reticulated => 3,
        }
    }

    fn gen(rng: &mut impl Rng) -> Self {
        *Self::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |c| c.weight())
            .unwrap()
    }
}

#[derive(Clone, Copy, Deserialize, Display, EnumIter, Serialize)]
enum TongueColor {
    Black,
    Blue,
    Orange,
    Pale,
    Red,
}

impl TongueColor {
    fn weight(self) -> u8 {
        match self {
            Self::Black | Self::Blue | Self::Orange | Self::Pale => 1,
            Self::Red => 2,
        }
    }

    fn gen(rng: &mut impl Rng) -> Self {
        *Self::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |c| c.weight())
            .unwrap()
    }
}

#[derive(Clone, Copy, Deserialize, Display, EnumIter, Serialize)]
enum EyeColor {
    Blue,
    Brown,
    Green,
    Red,
    Tan,
    Yellow,
}

impl EyeColor {
    fn gen(rng: &mut impl Rng) -> Self {
        Self::iter().choose(rng).unwrap()
    }
}

#[derive(Clone, Copy, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum PurebloodCharacteristics {
    Fangs,
    #[strum(serialize = "Forked tongue")]
    ForkedTongue,
    #[strum(serialize = "Scaly arms and hands")]
    ScalyArms,
    #[strum(serialize = "Scaly face")]
    ScalyFace,
    #[strum(serialize = "Scaly torso")]
    ScalyTorso,
    #[strum(serialize = "Sepentine eyes")]
    SepentineEyes,
    Multiple,
}

impl PurebloodCharacteristics {
    fn multiple(rng: &mut impl Rng) -> Vec<Self> {
        Self::iter()
            .filter(|c| c != &Self::Multiple)
            .collect::<Vec<_>>()
            .choose_multiple_weighted(rng, 2, |c| c.weight())
            .unwrap()
            .copied()
            .collect::<Vec<_>>()
    }

    fn weight(self) -> u8 {
        match self {
            Self::Fangs | Self::ScalyFace | Self::SepentineEyes => 3,
            Self::ForkedTongue | Self::Multiple => 2,
            Self::ScalyArms | Self::ScalyTorso => 4,
        }
    }

    fn gen(rng: &mut impl Rng) -> Vec<Self> {
        let characteristic = *Self::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |c| c.weight())
            .unwrap();
        match characteristic {
            Self::Fangs
            | Self::ForkedTongue
            | Self::ScalyArms
            | Self::ScalyFace
            | Self::ScalyTorso
            | Self::SepentineEyes => vec![characteristic],
            Self::Multiple => Self::multiple(rng),
        }
    }
}

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 8),
    base_weight: 110,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct YuanTiPureblood {
    eye_color: EyeColor,
    origin: String,
    pureblood_characteristics: Vec<PurebloodCharacteristics>,
    scale_color: ScaleColor,
    scale_pattern: ScalePattern,
    skin_color: SkinColor,
    tongue_color: TongueColor,
}

impl AlignmentInfluences for YuanTiPureblood {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Neutral]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Evil]
    }
}

impl Appearance for YuanTiPureblood {
    fn appearance(&self) -> Vec<String> {
        vec![
            format!("Eye color: {}", self.eye_color),
            format!(
                "Pureblood characteristics: {}",
                self.pureblood_characteristics
                    .iter()
                    .map(|c| format!("{}", c))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!("Scales: {} {}", self.scale_pattern, self.scale_color),
            format!("Skin color: {}", self.skin_color),
            format!("Tongue color: {}", self.tongue_color),
        ]
    }
}

impl Backstory for YuanTiPureblood {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Origin: {}", self.origin)]
    }
}

impl Characteristics for YuanTiPureblood {
    const HUMAN_ANCESTRY: bool = true;
    const SIZE: Size = Size::Medium;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=100)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for YuanTiPureblood {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Vgtm, 120)])
    }
}

impl Features for YuanTiPureblood {
    fn features(&self) -> Vec<Feature> {
        vec![
            // You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can’t discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Vgtm, 120),
            },
            // You know the poison spray cantrip. You can cast animal friendship an unlimited number of times with this trait, but you can target only snakes with it. Starting at 3rd level, you can also cast suggestion with this trait. Once you cast it, you can’t do so again until you finish a long rest. Charisma is your spellcasting ability for these spells.
            Feature {
                title: "Innate Spellcasting",
                citation: Citation(Book::Vgtm, 120),
            },
            // You have advantage on saving throws against spells and other magical effects.
            Feature {
                title: "Magic Resistance",
                citation: Citation(Book::Vgtm, 120),
            },
            // You are immune to poison damage and the poisoned condition.
            Feature {
                title: "Poison Immunity",
                citation: Citation(Book::Vgtm, 120),
            },
        ]
    }
}

impl Languages for YuanTiPureblood {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Abyssal, Language::Draconic]
    }
}

impl Name for YuanTiPureblood {
    /// Name also requires getting a set of human names (for human lineage)
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        let first_name = *[
            Human::gen_first_name(rng, characteristics),
            *NAMES.choose(rng).unwrap(),
        ]
        .choose(rng)
        .unwrap();
        format!(
            "{} {}",
            first_name,
            Human::gen_surname(rng, characteristics)
        )
    }
}

impl Proficiencies for YuanTiPureblood {}

#[typetag::serde]
impl Race for YuanTiPureblood {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            eye_color: EyeColor::gen(rng),
            origin: (*MONSTROUS_ORIGIN.choose(rng).unwrap()).to_string(),
            pureblood_characteristics: PurebloodCharacteristics::gen(rng),
            scale_color: ScaleColor::gen(rng),
            scale_pattern: ScalePattern::gen(rng),
            skin_color: SkinColor::gen(rng),
            tongue_color: TongueColor::gen(rng),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Charisma, 2),
            AbilityScore(AbilityScoreType::Intelligence, 1),
        ]
    }
}

impl Resistances for YuanTiPureblood {
    fn immunities(&self) -> Vec<DamageType> {
        vec![DamageType::Poison]
    }
}

impl fmt::Display for YuanTiPureblood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Yuan-ti Pureblood")
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
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (yuan_ti, _name, _characteristics) = YuanTiPureblood::gen(&mut rng);
        insta::assert_display_snapshot!(yuan_ti);
    }

    #[test]
    fn test_attitude() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (yuan_ti, _name, _characteristics) = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.attitude());
    }

    #[test]
    fn test_morality() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (yuan_ti, _name, _characteristics) = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (yuan_ti, _name, _characteristics) = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.backstory());
    }

    #[test]
    fn test_characteristics() {
        let yuan_ti = YuanTiPureblood {
            eye_color: EyeColor::Blue,
            origin: String::new(),
            pureblood_characteristics: vec![],
            scale_color: ScaleColor::Albino,
            scale_pattern: ScalePattern::Mottled,
            skin_color: SkinColor::Dark,
            tongue_color: TongueColor::Black,
        };
        assert_eq!(yuan_ti.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(yuan_ti.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (yuan_ti, _name, _characteristics) = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (yuan_ti, _name, _characteristics) = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (yuan_ti, _name, _characteristics) = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (_yuan_ti, name, _characteristics) = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_abilities() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (yuan_ti, _name, _characteristics) = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.abilities());
    }

    #[test]
    fn test_snapshot_immunities() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (yuan_ti, _name, _characteristics) = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.immunities());
    }
}
