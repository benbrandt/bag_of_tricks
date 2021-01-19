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
pub(crate) struct Characteristics {
    pub(crate) age: u16,
    pub(crate) gender: Gender,
    pub(crate) height: usize,
    pub(crate) size: Size,
    pub(crate) weight: usize,
}

impl Characteristics {
    pub(crate) fn gen(
        rng: &mut impl Rng,
        age_range: &AgeRange,
        size: Size,
        height_and_weight: &HeightAndWeightTable,
    ) -> Self {
        let age = age_range.gen(rng);
        let gender = Gender::gen(rng);
        let (height, weight) = height_and_weight.gen(rng);
        Self {
            age,
            gender,
            height,
            size,
            weight,
        }
    }
}

impl fmt::Display for Characteristics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Age: {}", self.age)?;
        writeln!(f, "Gender: {}", self.gender)?;
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "Height: {}'{}\"", self.height / 12, self.height % 12)?;
        writeln!(f, "Weight: {} lb.", self.weight)
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
    fn test_characteristics_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let characteristics = Characteristics {
            age: 100,
            gender: Gender::gen(&mut rng),
            height: 75,
            size: Size::Medium,
            weight: 300,
        };
        // Struct Snapshot
        insta::assert_yaml_snapshot!(characteristics);
        // fmt::Display Snapshot
        insta::assert_snapshot!(format!("{}", characteristics));
    }
}
