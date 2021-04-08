use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Display, EnumIter)]
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

#[derive(Clone, Copy, Display, EnumIter)]
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

#[derive(Clone, Copy, Display, EnumIter)]
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

#[derive(Clone, Copy, Display, EnumIter)]
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

#[derive(Clone, Copy, Display, EnumIter)]
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

#[derive(Clone, Copy, Display, EnumIter, PartialEq)]
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
            .cloned()
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
