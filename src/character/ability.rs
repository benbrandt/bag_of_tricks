use rand::Rng;
use std::fmt;
use strum_macros::EnumIter;

use crate::dice_roller::{roll_dice, Die};

#[derive(EnumIter, PartialEq)]
pub(crate) enum AbilityScoreType {
    Charisma,
    Constitution,
    Dexterity,
    Intelligence,
    Strength,
    Wisdom,
}

/// Value of a base ability score.
#[derive(Debug)]
pub(crate) struct AbilityScore {
    pub(crate) base: i8,
    increase: i8,
}

impl AbilityScore {
    /// Generate a new ability score based on dice rolls
    fn new(rng: &mut impl Rng, increase: i8) -> Self {
        // Roll 4 d6's
        let mut rolls = roll_dice(rng, Die::D6, 4);
        // Reverse sort, highest to lowest
        rolls.sort_by(|a, b| b.roll.cmp(&a.roll));
        // Sum top 3
        let base = rolls.drain(0..3).fold(0, |acc, d| acc + d.roll);
        Self { base, increase }
    }

    /// Return score value (base + increase)
    const fn score(&self) -> i8 {
        self.base + self.increase
    }

    /// Return modifier based on ability score.
    const fn modifier(&self) -> i8 {
        let score = self.score();
        // Lower value to closest even number, subtract by 10 and divide by two
        (score - score % 2 - 10) / 2
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

/// Increases to ablity scores
#[derive(Debug)]
pub(crate) struct AbilityScoreIncreases {
    pub(crate) charisma: i8,
    pub(crate) constitution: i8,
    pub(crate) dexterity: i8,
    pub(crate) intelligence: i8,
    pub(crate) strength: i8,
    pub(crate) wisdom: i8,
}

impl Default for AbilityScoreIncreases {
    fn default() -> Self {
        Self {
            charisma: 0,
            constitution: 0,
            dexterity: 0,
            intelligence: 0,
            strength: 0,
            wisdom: 0,
        }
    }
}

impl AbilityScores {
    /// Generate a set of ability scores for a character
    pub(crate) fn new(rng: &mut impl Rng, increases: &AbilityScoreIncreases) -> Self {
        Self {
            charisma: AbilityScore::new(rng, increases.charisma),
            constitution: AbilityScore::new(rng, increases.constitution),
            dexterity: AbilityScore::new(rng, increases.dexterity),
            intelligence: AbilityScore::new(rng, increases.intelligence),
            strength: AbilityScore::new(rng, increases.strength),
            wisdom: AbilityScore::new(rng, increases.wisdom),
        }
    }
}

impl fmt::Display for AbilityScores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            charisma,
            constitution,
            dexterity,
            intelligence,
            strength,
            wisdom,
        } = self;
        for (abbr, ability) in &[
            ("STR", strength),
            ("DEX", dexterity),
            ("CON", constitution),
            ("INT", intelligence),
            ("WIS", wisdom),
            ("CHA", charisma),
        ] {
            writeln!(
                f,
                "{} {:+3} ({})",
                abbr,
                ability.modifier(),
                ability.score()
            )?;
        }
        write!(f, "")
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
        let score = AbilityScore::new(&mut rng, 0);
        assert!(score.base >= 3 && score.base <= 18);
    }

    #[test]
    fn test_ability_score_avg() {
        let mut rng = Pcg64::from_entropy();
        let average = (0..100).fold(0 as f64, |acc, _| {
            acc + AbilityScore::new(&mut rng, 0).base as f64
        }) / 100.0;
        // Comparison based on http://rumkin.com/reference/dnd/diestats.php
        assert!(12.24 - 2.847 < average && average < 12.24 + 2.847);
    }

    #[test]
    fn test_ability_score_modifier() {
        let modifier = AbilityScore {
            base: 1,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, -5);
        let modifier = AbilityScore {
            base: 2,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, -4);
        let modifier = AbilityScore {
            base: 3,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, -4);
        let modifier = AbilityScore {
            base: 4,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, -3);
        let modifier = AbilityScore {
            base: 5,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, -3);
        let modifier = AbilityScore {
            base: 6,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, -2);
        let modifier = AbilityScore {
            base: 7,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, -2);
        let modifier = AbilityScore {
            base: 8,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, -1);
        let modifier = AbilityScore {
            base: 9,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, -1);
        let modifier = AbilityScore {
            base: 10,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, 0);
        let modifier = AbilityScore {
            base: 11,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, 0);
        let modifier = AbilityScore {
            base: 12,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, 1);
        let modifier = AbilityScore {
            base: 13,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, 1);
        let modifier = AbilityScore {
            base: 14,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, 2);
        let modifier = AbilityScore {
            base: 15,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, 2);
        let modifier = AbilityScore {
            base: 16,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, 3);
        let modifier = AbilityScore {
            base: 17,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, 3);
        let modifier = AbilityScore {
            base: 18,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, 4);
        let modifier = AbilityScore {
            base: 19,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, 4);
        let modifier = AbilityScore {
            base: 20,
            increase: 0,
        }
        .modifier();
        assert_eq!(modifier, 5);
    }

    #[test]
    fn test_ability_score_increase_modifier() {
        let modifier = AbilityScore {
            base: 1,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, -4);
        let modifier = AbilityScore {
            base: 2,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, -4);
        let modifier = AbilityScore {
            base: 3,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, -3);
        let modifier = AbilityScore {
            base: 4,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, -3);
        let modifier = AbilityScore {
            base: 5,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, -2);
        let modifier = AbilityScore {
            base: 6,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, -2);
        let modifier = AbilityScore {
            base: 7,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, -1);
        let modifier = AbilityScore {
            base: 8,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, -1);
        let modifier = AbilityScore {
            base: 9,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 0);
        let modifier = AbilityScore {
            base: 10,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 0);
        let modifier = AbilityScore {
            base: 11,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 1);
        let modifier = AbilityScore {
            base: 12,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 1);
        let modifier = AbilityScore {
            base: 13,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 2);
        let modifier = AbilityScore {
            base: 14,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 2);
        let modifier = AbilityScore {
            base: 15,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 3);
        let modifier = AbilityScore {
            base: 16,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 3);
        let modifier = AbilityScore {
            base: 17,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 4);
        let modifier = AbilityScore {
            base: 18,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 4);
        let modifier = AbilityScore {
            base: 19,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 5);
        let modifier = AbilityScore {
            base: 20,
            increase: 1,
        }
        .modifier();
        assert_eq!(modifier, 5);
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
        } = AbilityScores::new(&mut rng, &AbilityScoreIncreases::default());
        assert!(charisma.base >= 3 && charisma.base <= 18);
        assert!(constitution.base >= 3 && constitution.base <= 18);
        assert!(dexterity.base >= 3 && dexterity.base <= 18);
        assert!(intelligence.base >= 3 && intelligence.base <= 18);
        assert!(strength.base >= 3 && strength.base <= 18);
        assert!(wisdom.base >= 3 && wisdom.base <= 18);
    }
}
