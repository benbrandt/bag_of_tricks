use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::{fmt, ops::RangeInclusive};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

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

#[derive(Deserialize, Serialize)]
pub(crate) struct Characteristics {
    pub(crate) age: u16,
    pub(crate) gender: Gender,
}

impl Characteristics {
    pub(crate) fn gen(rng: &mut impl Rng, age_range: &AgeRange) -> Self {
        Self {
            age: age_range.gen(rng),
            gender: Gender::gen(rng),
        }
    }
}

impl fmt::Display for Characteristics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Age: {}", self.age)?;
        writeln!(f, "Gender: {}", self.gender)
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
        assert!(age >= 1 && age <= 100);
    }

    /// Verify that our snapshot remains the same.
    #[test]
    fn test_characteristics_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let characteristics = Characteristics {
            gender: Gender::gen(&mut rng),
            age: 100,
        };
        // Struct Snapshot
        insta::assert_yaml_snapshot!(characteristics);
        // fmt::Display Snapshot
        insta::assert_snapshot!(format!("{}", characteristics));
    }
}
