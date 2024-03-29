#![deny(clippy::all)]
#![warn(clippy::pedantic)]
pub mod names;

use std::{fmt, ops::RangeInclusive};

use dice_roller::RollCmd;
use names::human::Ethnicity;
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

/// An range of ages a given adventurer could be.
pub struct AgeRange(pub RangeInclusive<u16>);

impl AgeRange {
    /// Generate a random age within the given range.
    fn gen(&self, rng: &mut impl Rng) -> u16 {
        rng.gen_range(self.0.clone())
    }
}

/// Really only here to help decide on names, not core to a character choice.
#[derive(Deserialize, Display, EnumIter, Serialize)]
pub enum Gender {
    Female,
    Male,
}

impl Gender {
    /// Choose a random gender
    fn gen(rng: &mut impl Rng) -> Self {
        Gender::iter().choose(rng).unwrap()
    }
}

/// Weight modifier
#[derive(Debug, Eq, PartialEq)]
pub enum WeightMod {
    /// Fixed additional weight
    Fixed(u16),
    /// Role to find additional weight
    Roll(RollCmd),
}

/// Convert feet + inches to just inches (allow for better readability in tables)
#[must_use]
pub const fn in_inches(feet: u8, inches: u8) -> u8 {
    feet * 12 + inches
}

/// Data from a table to generate a height and weight for a given character.
#[derive(Debug, Eq, PartialEq)]
pub struct HeightAndWeightTable {
    /// Base height for this character
    pub base_height: u8,
    /// Base weight for this character
    pub base_weight: u16,
    /// Modifier to apply to the height
    pub height_mod: RollCmd,
    /// Modifier to apply to the weight (informed by height)
    pub weight_mod: WeightMod,
}

impl HeightAndWeightTable {
    /// Generate a random height and weight for a character
    pub fn gen(&self, rng: &mut impl Rng) -> (usize, usize) {
        let h = self.height_mod.roll(rng).total();
        // Weight modifier is multiplied by height
        let w = h * match &self.weight_mod {
            WeightMod::Fixed(f) => usize::from(*f),
            WeightMod::Roll(r) => r.roll(rng).total(),
        };
        (
            usize::from(self.base_height) + h,
            usize::from(self.base_weight) + w,
        )
    }
}

/// Size of character (there are more options for monsters)
#[derive(Deserialize, Display, EnumIter, Serialize)]
pub enum Size {
    Small,
    Medium,
}

/// Types of movement speeds
#[derive(Clone, Copy, Debug, Deserialize, EnumIter, Eq, PartialEq, Serialize)]
pub enum Speed {
    Climbing(u8),
    Flying(u8),
    Swimming(u8),
    Walking(u8),
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Climbing(s) => write!(f, "Climbing Speed: {}ft", s),
            Self::Flying(s) => write!(f, "Flying Speed: {}ft", s),
            Self::Swimming(s) => write!(f, "Swimming Speed: {}ft", s),
            Self::Walking(s) => write!(f, "Walking Speed: {}ft", s),
        }
    }
}

/// Physical characteristics about a character.
#[derive(Deserialize, Serialize)]
pub struct CharacteristicDetails {
    /// Age of the character
    pub age: u16,
    /// Base speed of the character
    pub base_speeds: Vec<Speed>,
    // Human ethnicity (if applicable)
    pub ethnicity: Option<Ethnicity>,
    /// Gender of the character (only used for name choices)
    pub gender: Gender,
    /// Height of the character
    pub height: usize,
    /// Size of the character
    pub size: Size,
    /// Weight of the character
    pub weight: usize,
}

impl fmt::Display for CharacteristicDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Age: {}", self.age)?;
        if let Some(ethnicity) = self.ethnicity {
            writeln!(f, "Human ethnicity: {}", ethnicity)?;
            if let Some(language) = ethnicity.human_language() {
                writeln!(f, "Human language known: {}", language)?;
            }
        }
        writeln!(f, "Gender: {}", self.gender)?;
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "Height: {}'{}\"", self.height / 12, self.height % 12)?;
        writeln!(f, "Weight: {} lb.", self.weight)
    }
}

/// Trait for object (usually race) to generate appropriate characteristics about a character
///
/// Separate from race since it uses associated constants (which can't be on a trait object)
pub trait Characteristics {
    /// Generate characteristics for this race
    fn gen_characteristics(&self, rng: &mut impl Rng) -> CharacteristicDetails {
        let (height, weight) = self.get_height_and_weight_table().gen(rng);
        CharacteristicDetails {
            age: self.get_age_range().gen(rng),
            base_speeds: self.get_base_speeds(),
            ethnicity: if self.has_human_ancestry() {
                Some(Ethnicity::gen(rng))
            } else {
                None
            },
            gender: Gender::gen(rng),
            height,
            size: self.get_size(),
            weight,
        }
    }

    /// Age range to choose from
    fn get_age_range(&self) -> AgeRange;

    /// Base speeds
    fn get_base_speeds(&self) -> Vec<Speed>;

    /// Calculate the height and weight table to generate from
    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable;

    /// Character size
    fn get_size(&self) -> Size;

    // Does this race have human ancestry?
    fn has_human_ancestry(&self) -> bool {
        false
    }
}

pub trait Appearance {
    fn appearance(&self) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_age_range() {
        let mut rng = Pcg64::from_entropy();
        let age = AgeRange(1..=100).gen(&mut rng);
        assert!((1..=100).contains(&age));
    }

    /// Verify that our snapshot remains the same.
    #[test]
    fn test_characteristics() {
        let mut rng = Pcg64::seed_from_u64(1);
        let characteristics = CharacteristicDetails {
            age: 100,
            base_speeds: vec![Speed::Walking(30)],
            ethnicity: Some(Ethnicity::Chondathan),
            gender: Gender::gen(&mut rng),
            height: 75,
            size: Size::Medium,
            weight: 300,
        };
        insta::assert_yaml_snapshot!(characteristics);
    }

    /// Verify that our snapshot remains the same.
    #[test]
    fn test_characteristics_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let characteristics = CharacteristicDetails {
            age: 100,
            base_speeds: vec![Speed::Walking(30)],
            ethnicity: Some(Ethnicity::Chondathan),
            gender: Gender::gen(&mut rng),
            height: 75,
            size: Size::Medium,
            weight: 300,
        };
        insta::assert_snapshot!(format!("{}", characteristics));
    }
}
