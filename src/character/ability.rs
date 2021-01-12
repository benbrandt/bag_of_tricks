use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::dice_roller::{roll_dice, Die};

/// Return modifier based on ability score.
const fn modifier(score: i8) -> i8 {
    // Lower value to closest even number, subtract by 10 and divide by two
    (score - score % 2 - 10) / 2
}

#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum AbilityScoreType {
    #[strum(serialize = "STR")]
    Strength,
    #[strum(serialize = "DEX")]
    Dexterity,
    #[strum(serialize = "CON")]
    Constitution,
    #[strum(serialize = "INT")]
    Intelligence,
    #[strum(serialize = "WIS")]
    Wisdom,
    #[strum(serialize = "CHA")]
    Charisma,
}

/// Value of a base ability score or increase
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct AbilityScore(pub(crate) AbilityScoreType, pub(crate) i8);

impl AbilityScore {
    /// Generate a new ability score based on dice rolls
    fn new(rng: &mut impl Rng, score_type: AbilityScoreType) -> Self {
        // Roll 4 d6's
        let mut rolls = roll_dice(rng, Die::D6, 4);
        // Reverse sort, highest to lowest
        rolls.sort_by(|a, b| b.roll.cmp(&a.roll));
        // Sum top 3
        let score = rolls.drain(0..3).fold(0, |acc, d| acc + d.roll);
        Self(score_type, score)
    }
}

/// Full set of ability scores a character could have
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AbilityScores(pub(crate) Vec<AbilityScore>);

impl AbilityScores {
    /// Generate a set of ability scores for a character
    pub(crate) fn new(rng: &mut impl Rng) -> Self {
        Self(
            AbilityScoreType::iter()
                .map(|t| AbilityScore::new(rng, t))
                .collect(),
        )
    }

    pub(crate) fn extend(&mut self, addl_scores: Self) {
        self.0.extend(addl_scores.0)
    }

    pub(crate) fn scores(&self) -> HashMap<AbilityScoreType, i8> {
        let mut scores = HashMap::new();
        for AbilityScore(score_type, val) in self.0.clone() {
            *scores.entry(score_type).or_insert(0) += val;
        }
        scores
    }
}

impl fmt::Display for AbilityScores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scores = self.scores();
        for score_type in AbilityScoreType::iter() {
            let score = *scores.get(&score_type).unwrap_or(&0);
            writeln!(f, "{} {:+3} ({})", score_type, modifier(score), score)?;
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
        let score = AbilityScore::new(&mut rng, AbilityScoreType::Charisma);
        assert!(score.1 >= 3 && score.1 <= 18);
    }

    #[test]
    fn test_ability_score_avg() {
        let mut rng = Pcg64::from_entropy();
        let average = (0..100).fold(0 as f64, |acc, _| {
            acc + AbilityScore::new(&mut rng, AbilityScoreType::Constitution).1 as f64
        }) / 100.0;
        // Comparison based on http://rumkin.com/reference/dnd/diestats.php
        assert!(12.24 - 2.847 < average && average < 12.24 + 2.847);
    }

    #[test]
    fn test_ability_score_modifier() {
        assert_eq!(modifier(1), -5);
        assert_eq!(modifier(2), -4);
        assert_eq!(modifier(3), -4);
        assert_eq!(modifier(4), -3);
        assert_eq!(modifier(5), -3);
        assert_eq!(modifier(6), -2);
        assert_eq!(modifier(7), -2);
        assert_eq!(modifier(8), -1);
        assert_eq!(modifier(9), -1);
        assert_eq!(modifier(10), 0);
        assert_eq!(modifier(11), 0);
        assert_eq!(modifier(12), 1);
        assert_eq!(modifier(13), 1);
        assert_eq!(modifier(14), 2);
        assert_eq!(modifier(15), 2);
        assert_eq!(modifier(16), 3);
        assert_eq!(modifier(17), 3);
        assert_eq!(modifier(18), 4);
        assert_eq!(modifier(19), 4);
        assert_eq!(modifier(20), 5);
    }

    #[test]
    fn test_ability_scores() {
        let mut rng = Pcg64::from_entropy();
        let scores = AbilityScores::new(&mut rng).scores();
        for score_type in AbilityScoreType::iter() {
            let score = *scores.get(&score_type).unwrap();
            assert!(score >= 3 && score <= 18);
        }
    }
}
