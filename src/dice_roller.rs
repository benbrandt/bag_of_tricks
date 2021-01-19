use rand::Rng;

/// Dice types
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Die {
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
    D20 = 20,
    D100 = 100,
}

/// Rolled value of a die
pub(crate) struct DieResult {
    /// Die type
    pub(crate) die: Die,
    /// Rolled number
    pub(crate) roll: u8,
}

pub(crate) struct RollResult(pub(crate) Vec<DieResult>);

impl RollResult {
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
