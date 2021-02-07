use std::fmt;
use strum_macros::Display;

use rand::Rng;

/// Dice types
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub(crate) enum Die {
    #[strum(serialize = "d4")]
    D4 = 4,
    #[strum(serialize = "d6")]
    D6 = 6,
    #[strum(serialize = "d8")]
    D8 = 8,
    #[strum(serialize = "d10")]
    D10 = 10,
    #[strum(serialize = "d12")]
    D12 = 12,
    #[strum(serialize = "d20")]
    D20 = 20,
    #[strum(serialize = "d100")]
    D100 = 100,
}

/// Rolled value of a die
pub(crate) struct DieResult {
    /// Die type
    pub(crate) die: Die,
    /// Rolled number
    pub(crate) roll: u8,
}

/// Result of dice rolls (multiple if more than one was requested)
pub(crate) struct RollResult(pub(crate) Vec<DieResult>);

impl RollResult {
    /// Total of all rolls
    pub(crate) fn total(&self) -> usize {
        self.0.iter().fold(0, |acc, d| acc + usize::from(d.roll))
    }
}

/// Roll a die
fn roll_die(rng: &mut impl Rng, die: Die) -> DieResult {
    DieResult {
        die,
        roll: rng.gen_range(1..=die as u8),
    }
}

/// Roll multiple dice
pub(crate) struct RollCmd(pub(crate) usize, pub(crate) Die);

impl RollCmd {
    pub(crate) fn roll(&self, rng: &mut impl Rng) -> RollResult {
        RollResult((1..=self.0).map(|_| roll_die(rng, self.1)).collect())
    }
}

impl fmt::Display for RollCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_roll_die_d4() {
        let mut rng = Pcg64::from_entropy();
        let DieResult { die, roll } = roll_die(&mut rng, Die::D4);
        assert_eq!(die, Die::D4);
        assert!((1..=4).contains(&roll));
    }

    #[test]
    fn test_roll_die_d6() {
        let mut rng = Pcg64::from_entropy();
        let DieResult { die, roll } = roll_die(&mut rng, Die::D6);
        assert_eq!(die, Die::D6);
        assert!((1..=6).contains(&roll));
    }

    #[test]
    fn test_roll_die_d8() {
        let mut rng = Pcg64::from_entropy();
        let DieResult { die, roll } = roll_die(&mut rng, Die::D8);
        assert_eq!(die, Die::D8);
        assert!((1..=8).contains(&roll));
    }

    #[test]
    fn test_roll_die_d10() {
        let mut rng = Pcg64::from_entropy();
        let DieResult { die, roll } = roll_die(&mut rng, Die::D10);
        assert_eq!(die, Die::D10);
        assert!((1..=10).contains(&roll));
    }

    #[test]
    fn test_roll_die_d12() {
        let mut rng = Pcg64::from_entropy();
        let DieResult { die, roll } = roll_die(&mut rng, Die::D12);
        assert_eq!(die, Die::D12);
        assert!((1..=12).contains(&roll));
    }

    #[test]
    fn test_roll_die_d20() {
        let mut rng = Pcg64::from_entropy();
        let DieResult { die, roll } = roll_die(&mut rng, Die::D20);
        assert_eq!(die, Die::D20);
        assert!((1..=20).contains(&roll));
    }

    #[test]
    fn test_roll_die_d100() {
        let mut rng = Pcg64::from_entropy();
        let DieResult { die, roll } = roll_die(&mut rng, Die::D100);
        assert_eq!(die, Die::D100);
        assert!((1..=100).contains(&roll));
    }

    #[test]
    fn test_roll_dice() {
        let mut rng = Pcg64::from_entropy();
        let rolls = RollCmd(4, Die::D6).roll(&mut rng);
        assert_eq!(rolls.0.len(), 4);
        assert!((4..=24).contains(&rolls.total()));
        for DieResult { die, roll } in rolls.0 {
            assert_eq!(die, Die::D6);
            assert!((1..=6).contains(&roll));
        }
    }
}
