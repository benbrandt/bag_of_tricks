use std::{collections::BTreeMap, f64::consts::E, fmt};

use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::dice_roller::{Die, RollCmd};

use super::{
    proficiencies::{Proficiencies, Proficiency},
    Character,
};

/// Return modifier based on ability score.
fn modifier(score: u8) -> i16 {
    // Lower value to closest even number, reduce by 10, and divide by two
    (i16::from(score) - i16::from(score) % 2 - 10) / 2
}

// Used for weighting ability scores. Get value to shift all other values by to have lowest be 1
pub(crate) fn modifier_shift<L>(modifiers: L) -> i16
where
    L: Iterator<Item = i16> + Clone,
{
    let min = modifiers.min().unwrap_or(0);
    if min <= 0 {
        min.abs()
    } else {
        -min
    }
}

pub(crate) fn modifier_weight(modifier: i16, shift: i16) -> f64 {
    E.powi(i32::from(modifier + shift))
}

/// All possible ability score types to choose from
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
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
    /// Generate an ability score by rolling 4d6 and keeping the highest 3
    fn gen(rng: &mut impl Rng) -> u8 {
        // Roll 4 d6's
        let mut rolls = RollCmd(4, Die::D6).roll(rng).0;
        // Reverse sort, highest to lowest
        rolls.sort_by(|a, b| b.roll.cmp(&a.roll));
        // Sum top 3
        let score = rolls.drain(0..3).fold(0, |acc, d| acc + d.roll);
        score
    }
}

/// Full set of ability scores a character could have
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AbilityScores(pub(crate) BTreeMap<AbilityScoreType, u8>);

impl AbilityScores {
    /// Generate a set of ability scores for a character
    pub(crate) fn gen(rng: &mut impl Rng) -> Self {
        let mut scores = BTreeMap::new();
        for a in AbilityScoreType::iter() {
            scores.insert(a, AbilityScore::gen(rng));
        }
        Self(scores)
    }

    /// Add list of ability score increases to the totals
    pub(crate) fn increase(&mut self, addl_scores: Vec<AbilityScore>) {
        for AbilityScore(score_type, val) in addl_scores {
            *self.0.entry(score_type).or_insert(0) += val;
        }
    }

    /// Get modifier for a given ability score type
    pub(crate) fn modifier(&self, ability: AbilityScoreType) -> i16 {
        modifier(*self.0.get(&ability).unwrap_or(&0))
    }
}

impl fmt::Display for AbilityScores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for score_type in AbilityScoreType::iter() {
            let score = *self.0.get(&score_type).unwrap_or(&0);
            writeln!(f, "{} {:+3} ({})", score_type, modifier(score), score)?;
        }
        write!(f, "")
    }
}

/// All skill types available
#[allow(dead_code)]
#[derive(
    Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub(crate) enum Skill {
    Acrobatics,
    #[strum(serialize = "Animal Handling")]
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
    #[strum(serialize = "Sleight of Hand")]
    SleightOfHand,
    Stealth,
    Survival,
}

impl Skill {
    /// Return corresponding ability score type for a given skill
    pub(crate) fn ability_score_type(self) -> AbilityScoreType {
        match self {
            Skill::Athletics => AbilityScoreType::Strength,
            Skill::Acrobatics | Skill::SleightOfHand | Skill::Stealth => {
                AbilityScoreType::Dexterity
            }
            Skill::Arcana
            | Skill::History
            | Skill::Investigation
            | Skill::Nature
            | Skill::Religion => AbilityScoreType::Intelligence,
            Skill::AnimalHandling
            | Skill::Insight
            | Skill::Medicine
            | Skill::Perception
            | Skill::Survival => AbilityScoreType::Wisdom,
            Skill::Deception | Skill::Intimidation | Skill::Performance | Skill::Persuasion => {
                AbilityScoreType::Charisma
            }
        }
    }

    /// Return the modifier for a skill, adding proficiency bonus if applicable
    pub(crate) fn modifier(self, character: &Character) -> i16 {
        character.abilities.modifier(self.ability_score_type())
            + if self.proficient(character) {
                character.proficiency_bonus()
            } else {
                0
            }
    }

    /// Check if the character is proficient in this skill
    pub(crate) fn proficient(self, character: &Character) -> bool {
        character
            .proficiencies()
            .contains(&Proficiency::Skill(self))
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
        let score = AbilityScore::gen(&mut rng);
        assert!((3..=18).contains(&score));
    }

    #[test]
    fn test_ability_score_avg() {
        let mut rng = Pcg64::from_entropy();
        let average = (0..100).fold(f64::from(0), |acc, _| {
            acc + f64::from(AbilityScore::gen(&mut rng))
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
        let scores = AbilityScores::gen(&mut rng).0;
        for score_type in AbilityScoreType::iter() {
            let score = *scores.get(&score_type).unwrap();
            assert!((3..=18).contains(&score));
        }
    }

    #[test]
    fn test_ability_scores_increase() {
        let mut rng = Pcg64::seed_from_u64(1);
        let mut scores = AbilityScores::gen(&mut rng);
        let more_scores = AbilityScores::gen(&mut rng);
        scores.increase(
            more_scores
                .0
                .into_iter()
                .map(|(t, v)| AbilityScore(t, v))
                .collect(),
        );
        insta::assert_yaml_snapshot!(scores);
    }

    #[test]
    fn test_ability_scores_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let scores = AbilityScores::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", scores));
    }

    #[test]
    fn test_modifier_shift() {
        let mods = vec![-5, 1].into_iter();
        assert_eq!(modifier_shift(mods), 5);
        let mods = vec![4, 5].into_iter();
        assert_eq!(modifier_shift(mods), -4);
        let mods = vec![-1, 0, 1].into_iter();
        assert_eq!(modifier_shift(mods), 1);
        let mods = vec![0, 1].into_iter();
        assert_eq!(modifier_shift(mods), 0);
        let mods = vec![1, 1].into_iter();
        assert_eq!(modifier_shift(mods), -1);
        let mods = vec![0, 0].into_iter();
        assert_eq!(modifier_shift(mods), 0);
    }
}
