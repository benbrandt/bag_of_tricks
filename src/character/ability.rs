use crate::dice_roller::{roll_dice, Die};
use rand::Rng;

/// Value of a base ability score.
#[derive(Debug)]
pub(crate) struct AbilityScore(pub(crate) i32);

impl AbilityScore {
    /// Generate a new ability score based on dice rolls
    fn new(rng: &mut impl Rng) -> Self {
        // Roll 4 d6's
        let mut rolls = roll_dice(rng, Die::D6, 4);
        // Reverse sort, highest to lowest
        rolls.sort_by(|a, b| b.roll.cmp(&a.roll));
        // Sum top 3
        let score = rolls.drain(0..3).fold(0, |acc, d| acc + d.roll);
        Self(score)
    }

    /// Return modifier based on ability score.
    const fn modifier(&self) -> i32 {
        // Lower value to closest even number, subtract by 10 and divide by two
        (self.0 - self.0 % 2 - 10) / 2
    }
}

/// Full set of ability scores a character could have
#[derive(Debug)]
pub(crate) struct AbilityScores {
    pub(crate) charisma: AbilityScore,
    pub(crate) constitution: AbilityScore,
    pub(crate) dexterity: AbilityScore,
    pub(crate) intelligence: AbilityScore,
    pub(crate) strength: AbilityScore,
    pub(crate) wisdom: AbilityScore,
}

impl AbilityScores {
    /// Generate a set of ability scores for a character
    pub(crate) fn new(rng: &mut impl Rng) -> Self {
        Self {
            charisma: AbilityScore::new(rng),
            constitution: AbilityScore::new(rng),
            dexterity: AbilityScore::new(rng),
            intelligence: AbilityScore::new(rng),
            strength: AbilityScore::new(rng),
            wisdom: AbilityScore::new(rng),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_ability_score_new() {
        let mut rng = Pcg64::from_entropy();
        let score = AbilityScore::new(&mut rng);
        assert!(score.0 >= 3 && score.0 <= 18);
    }

    #[test]
    fn test_ability_score_avg() {
        let mut rng = Pcg64::from_entropy();
        let average = (0..100).fold(0, |acc, _| acc + AbilityScore::new(&mut rng).0) as f64 / 100.0;
        // Comparison based on http://rumkin.com/reference/dnd/diestats.php
        assert!(12.24 - 2.847 < average && average < 12.24 + 2.847);
    }

    #[test]
    fn test_ability_score_modifier() {
        assert_eq!(AbilityScore(1).modifier(), -5);
        assert_eq!(AbilityScore(2).modifier(), -4);
        assert_eq!(AbilityScore(3).modifier(), -4);
        assert_eq!(AbilityScore(4).modifier(), -3);
        assert_eq!(AbilityScore(5).modifier(), -3);
        assert_eq!(AbilityScore(6).modifier(), -2);
        assert_eq!(AbilityScore(7).modifier(), -2);
        assert_eq!(AbilityScore(8).modifier(), -1);
        assert_eq!(AbilityScore(9).modifier(), -1);
        assert_eq!(AbilityScore(10).modifier(), 0);
        assert_eq!(AbilityScore(11).modifier(), 0);
        assert_eq!(AbilityScore(12).modifier(), 1);
        assert_eq!(AbilityScore(13).modifier(), 1);
        assert_eq!(AbilityScore(14).modifier(), 2);
        assert_eq!(AbilityScore(15).modifier(), 2);
        assert_eq!(AbilityScore(16).modifier(), 3);
        assert_eq!(AbilityScore(17).modifier(), 3);
        assert_eq!(AbilityScore(18).modifier(), 4);
        assert_eq!(AbilityScore(19).modifier(), 4);
        assert_eq!(AbilityScore(20).modifier(), 5);
    }

    #[test]
    fn test_ability_scores() {
        let mut rng = Pcg64::from_entropy();
        let AbilityScores {
            charisma,
            constitution,
            dexterity,
            intelligence,
            strength,
            wisdom,
        } = AbilityScores::new(&mut rng);
        assert!(charisma.0 >= 3 && charisma.0 <= 18);
        assert!(constitution.0 >= 3 && constitution.0 <= 18);
        assert!(dexterity.0 >= 3 && dexterity.0 <= 18);
        assert!(intelligence.0 >= 3 && intelligence.0 <= 18);
        assert!(strength.0 >= 3 && strength.0 <= 18);
        assert!(wisdom.0 >= 3 && wisdom.0 <= 18);
    }
}
