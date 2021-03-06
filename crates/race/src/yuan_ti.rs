use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::{DamageType, Resistances};
use backstory::{Backstory, MONSTROUS_ORIGIN};
use characteristics::{
    in_inches,
    names::{yuan_ti::NAMES, Name},
    AgeRange, Appearance, CharacteristicDetails, Characteristics, HeightAndWeightTable, Size,
    Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use dice_roller::{Die, RollCmd};
use features::{Feature, Features};
use languages::{Language, Languages};
use personality::{Influence, PersonalityOptions};
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

use super::{human::Human, Race};

const BONDS: &[&str] = &[
    "I will see our empire rise again and, in so doing, win the favor of the serpent gods.",
    "I am enamored with the culture and trappings of another society and wish to be part of it.",
    "I respect my superiors and obey them without question. My fate is theirs to decide.",
    "I have an interest in an unsuitable mate, which I can't suppress.",
    "I respect and emulate a great hero or ancestor.",
    "An enemy destroyed something of value to me, and I will find where it lives and kill the offender.",
];
const FLAWS: &[&str] = &[
    "I feel twinges of emotion, and it shames me that I am imperfect in this way.",
    "I put too much credence in the dictates of a particular god.",
    "I frequently overindulge in food and wine, and I am impaired and lethargic for days afterward.",
    "I worship a forbidden god.",
    "I secretly believe things would be better if I was in charge.",
    "If I could get away with it, I would gladly kill and eat a superior yuan-ti.",
];
const IDEALS: &[(&str, Influence)] = &[
    ("Greed. I display my wealth as a sign of my power and prosperity.", Influence::Evil),
    ("Aspiration. I strive to follow the path toward becoming an anathema.", Influence::Evil),
    ("Unity. No leader shall put personal goals above those of our race.", Influence::Any),
    ("Kinship. My allegiance is to my caste and my city. Other settlements can burn for all I care.", Influence::Any),
    ("Inspiration. My actions set an example for the lesser castes to emulate.", Influence::Any),
    ("Power. Everything I choose to do is determined by whether it will make me smarter and stronger.", Influence::Evil),
];
const TRAITS: &[&str] = &[
    "I see omens in every event and action. The serpent gods continue to advise us.",
    "I have very high standards for food, drink, and physical pleasures.",
    "I prefer to be alone rather than among other creatures, including my own kind.",
    "I sometimes become consumed by philosophy.",
    "I believe I am superior to others of my caste.",
    "I am driven by wanderlust and want to explore lands far from our cities.",
    "I am interested in modern human culture, even as primitive as it is.",
    "I await the day when we again conquer lands by force, as we did in the old times.",
];

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

impl Default for SkinColor {
    fn default() -> Self {
        Self::Dark
    }
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

impl Default for ScaleColor {
    fn default() -> Self {
        Self::Black
    }
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

impl Default for ScalePattern {
    fn default() -> Self {
        Self::Mottled
    }
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

impl Default for TongueColor {
    fn default() -> Self {
        Self::Black
    }
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

impl Default for EyeColor {
    fn default() -> Self {
        EyeColor::Blue
    }
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

#[derive(Default, Deserialize, Serialize)]
pub struct YuanTiPureblood {
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

    fn has_human_ancestry(&self) -> bool {
        true
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
    fn gen_name(&self, rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
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

impl Pantheons for YuanTiPureblood {}

impl PersonalityOptions for YuanTiPureblood {
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

impl Proficiencies for YuanTiPureblood {}

impl Race for YuanTiPureblood {
    fn gen(rng: &mut impl Rng) -> Self {
        Self {
            eye_color: EyeColor::gen(rng),
            origin: (*MONSTROUS_ORIGIN.choose(rng).unwrap()).to_string(),
            pureblood_characteristics: PurebloodCharacteristics::gen(rng),
            scale_color: ScaleColor::gen(rng),
            scale_pattern: ScalePattern::gen(rng),
            skin_color: SkinColor::gen(rng),
            tongue_color: TongueColor::gen(rng),
        }
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

impl Trinkets for YuanTiPureblood {}

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
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_display_snapshot!(yuan_ti);
    }

    #[test]
    fn test_attitude() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.attitude());
    }

    #[test]
    fn test_morality() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.morality());
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
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
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.features());
    }

    #[test]
    fn test_snapshot_languages() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        let characteristics = yuan_ti.gen_characteristics(&mut rng);
        let name = yuan_ti.gen_name(&mut rng, &characteristics);
        insta::assert_yaml_snapshot!(name);
    }

    #[test]
    fn test_snapshot_abilities() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.abilities());
    }

    #[test]
    fn test_snapshot_immunities() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.immunities());
    }

    #[test]
    fn test_bonds() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.bonds());
    }

    #[test]
    fn test_flaws() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.flaws());
    }

    #[test]
    fn test_ideals() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.ideals());
    }

    #[test]
    fn test_traits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let yuan_ti = YuanTiPureblood::gen(&mut rng);
        insta::assert_yaml_snapshot!(yuan_ti.traits());
    }
}
