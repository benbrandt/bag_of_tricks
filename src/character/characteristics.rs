use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::{fmt, ops::RangeInclusive};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::dice_roller::RollCmd;

pub(crate) struct AgeRange(pub(crate) RangeInclusive<u16>);

impl AgeRange {
    fn gen(&self, rng: &mut impl Rng) -> u16 {
        rng.gen_range(self.0.clone())
    }
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
pub(crate) enum Gender {
    Female,
    Male,
}

impl Gender {
    fn gen(rng: &mut impl Rng) -> Self {
        Gender::iter().choose(rng).unwrap()
    }
}

pub(crate) enum WeightMod {
    Fixed(u16),
    Roll(RollCmd),
}

pub(crate) const fn in_inches(feet: u8, inches: u8) -> u8 {
    feet * 12 + inches
}

pub(crate) struct HeightAndWeightTable {
    pub(crate) base_height: u8,
    pub(crate) base_weight: u16,
    pub(crate) height_mod: RollCmd,
    pub(crate) weight_mod: WeightMod,
}

impl HeightAndWeightTable {
    pub(crate) fn gen(&self, rng: &mut impl Rng) -> (usize, usize) {
        let h = self.height_mod.roll(rng).total();
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
#[derive(Deserialize, Display, EnumIter, Serialize)]
pub(crate) enum Size {
    Small,
    Medium,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CharacteristicDetails {
    pub(crate) age: u16,
    pub(crate) base_speed: u8,
    pub(crate) gender: Gender,
    pub(crate) height: usize,
    pub(crate) size: Size,
    pub(crate) weight: usize,
}

impl fmt::Display for CharacteristicDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Age: {}", self.age)?;
        writeln!(f, "Gender: {}", self.gender)?;
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "Height: {}'{}\"", self.height / 12, self.height % 12)?;
        writeln!(f, "Weight: {} lb.", self.weight)
    }
}

pub(crate) trait Characteristics {
    const AGE_RANGE: AgeRange;
    const SIZE: Size;

    fn get_base_speed(&self) -> u8;

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable;

    fn gen_characteristics(&self, rng: &mut impl Rng) -> CharacteristicDetails {
        let (height, weight) = self.get_height_and_weight_table().gen(rng);
        CharacteristicDetails {
            age: Self::AGE_RANGE.gen(rng),
            base_speed: self.get_base_speed(),
            gender: Gender::gen(rng),
            height,
            size: Self::SIZE,
            weight,
        }
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
            base_speed: 30,
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
            base_speed: 30,
            gender: Gender::gen(&mut rng),
            height: 75,
            size: Size::Medium,
            weight: 300,
        };
        insta::assert_snapshot!(format!("{}", characteristics));
    }
}
