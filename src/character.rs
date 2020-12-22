/// Value of a base ability score.
struct AbilityScore(i32);

impl AbilityScore {
    /// Return modifier based on ability score.
    const fn modifier(&self) -> i32 {
        // Lower value to closest even number, subtract by 10 and divide by two
        (self.0 - self.0 % 2 - 10) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
