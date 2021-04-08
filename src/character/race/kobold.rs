use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
#[derive(Clone, Copy, Display, EnumIter, PartialEq)]
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
            .cloned()
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

#[derive(Display, EnumIter)]
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
