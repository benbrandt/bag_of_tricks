use rand::Rng;

/// Dice types
enum Dice {
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
    D20 = 20,
    D100 = 100,
}

/// Roll a die
fn roll_die(rng: &mut impl Rng, dice: Dice) -> u32 {
    rng.gen_range(1..=dice as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_roll_die() {
        let mut rng = Pcg64::from_entropy();
        let roll = roll_die(&mut rng, Dice::D4);
        assert!(roll >= 1 && roll <= 4);
        let roll = roll_die(&mut rng, Dice::D6);
        assert!(roll >= 1 && roll <= 6);
        let roll = roll_die(&mut rng, Dice::D8);
        assert!(roll >= 1 && roll <= 8);
        let roll = roll_die(&mut rng, Dice::D10);
        assert!(roll >= 1 && roll <= 10);
        let roll = roll_die(&mut rng, Dice::D12);
        assert!(roll >= 1 && roll <= 12);
        let roll = roll_die(&mut rng, Dice::D20);
        assert!(roll >= 1 && roll <= 20);
        let roll = roll_die(&mut rng, Dice::D100);
        assert!(roll >= 1 && roll <= 100);
    }
}
