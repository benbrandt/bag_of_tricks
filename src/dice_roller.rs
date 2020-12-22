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
pub(crate) struct DieRoll {
    /// Die type
    pub(crate) die: Die,
    /// Rolled number
    pub(crate) roll: i32,
}

/// Roll a die
fn roll_die(rng: &mut impl Rng, die: Die) -> DieRoll {
    DieRoll {
        die,
        roll: rng.gen_range(1..=die as i32),
    }
}

/// Roll multiple dice
pub(crate) fn roll_dice(rng: &mut impl Rng, die: Die, quantity: u32) -> Vec<DieRoll> {
    (1..=quantity).map(|_| roll_die(rng, die)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_roll_die_d4() {
        let mut rng = Pcg64::from_entropy();
        let DieRoll { die, roll } = roll_die(&mut rng, Die::D4);
        assert_eq!(die, Die::D4);
        assert!(roll >= 1 && roll <= 4);
    }

    #[test]
    fn test_roll_die_d6() {
        let mut rng = Pcg64::from_entropy();
        let DieRoll { die, roll } = roll_die(&mut rng, Die::D6);
        assert_eq!(die, Die::D6);
        assert!(roll >= 1 && roll <= 6);
    }

    #[test]
    fn test_roll_die_d8() {
        let mut rng = Pcg64::from_entropy();
        let DieRoll { die, roll } = roll_die(&mut rng, Die::D8);
        assert_eq!(die, Die::D8);
        assert!(roll >= 1 && roll <= 8);
    }

    #[test]
    fn test_roll_die_d10() {
        let mut rng = Pcg64::from_entropy();
        let DieRoll { die, roll } = roll_die(&mut rng, Die::D10);
        assert_eq!(die, Die::D10);
        assert!(roll >= 1 && roll <= 10);
    }

    #[test]
    fn test_roll_die_d12() {
        let mut rng = Pcg64::from_entropy();
        let DieRoll { die, roll } = roll_die(&mut rng, Die::D12);
        assert_eq!(die, Die::D12);
        assert!(roll >= 1 && roll <= 12);
    }

    #[test]
    fn test_roll_die_d20() {
        let mut rng = Pcg64::from_entropy();
        let DieRoll { die, roll } = roll_die(&mut rng, Die::D20);
        assert_eq!(die, Die::D20);
        assert!(roll >= 1 && roll <= 20);
    }

    #[test]
    fn test_roll_die_d100() {
        let mut rng = Pcg64::from_entropy();
        let DieRoll { die, roll } = roll_die(&mut rng, Die::D100);
        assert_eq!(die, Die::D100);
        assert!(roll >= 1 && roll <= 100);
    }

    #[test]
    fn test_roll_dice() {
        let mut rng = Pcg64::from_entropy();
        let rolls = roll_dice(&mut rng, Die::D6, 4);
        assert_eq!(rolls.len(), 4);
        for DieRoll { die, roll } in rolls {
            assert_eq!(die, Die::D6);
            assert!(roll >= 1 && roll <= 6);
        }
    }
}
