use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::dice_roller::{Die, RollCmd};

/// Return modifier based on ability score.
fn modifier(score: u8) -> i16 {
    // Lower value to closest even number, subtract by 10 and divide by two
    (i16::from(score) - i16::from(score) % 2 - 10) / 2
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
pub(crate) struct AbilityScore(pub(crate) AbilityScoreType, pub(crate) u8);

impl AbilityScore {
    /// Generate a new ability score based on dice rolls
    fn new(rng: &mut impl Rng, score_type: AbilityScoreType) -> Self {
        // Roll 4 d6's
        let mut rolls = RollCmd(4, Die::D6).roll(rng).0;
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

    pub(crate) fn modifier(&self, ability: AbilityScoreType) -> i16 {
        modifier(*self.scores().get(&ability).unwrap_or(&0))
    }

    pub(crate) fn scores(&self) -> HashMap<AbilityScoreType, u8> {
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

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Display, EnumIter, Serialize)]
pub(crate) enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
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
        let average = (0..100).fold(f64::from(0), |acc, _| {
            acc + f64::from(AbilityScore::new(&mut rng, AbilityScoreType::Constitution).1)
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
            assert!((3..=18).contains(&score));
        }
    }

    #[test]
    fn test_ability_scores_extend() {
        let mut rng = Pcg64::seed_from_u64(1);
        let mut scores = AbilityScores::new(&mut rng);
        scores.extend(AbilityScores::new(&mut rng));
        insta::assert_yaml_snapshot!(scores);
    }

    #[test]
    fn test_ability_scores_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let scores = AbilityScores::new(&mut rng);
        insta::assert_snapshot!(format!("{}", scores));
    }
}
