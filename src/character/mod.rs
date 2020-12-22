mod ability;

use ability::{AbilityScoreIncreases, AbilityScores};
use rand::Rng;
use std::fmt;

/// Character information
#[derive(Debug)]
pub struct Character {
    abilities: AbilityScores,
}

impl Character {
    /// Generate a new random character
    pub fn new(rng: &mut impl Rng) -> Self {
        Self {
            abilities: AbilityScores::new(rng, AbilityScoreIncreases::default()),
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abilities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_character_new() {
        let mut rng = Pcg64::from_entropy();
        let Character {
            abilities:
                AbilityScores {
                    charisma,
                    constitution,
                    dexterity,
                    intelligence,
                    strength,
                    wisdom,
                },
        } = Character::new(&mut rng);
        assert!(charisma.base >= 3 && charisma.base <= 18);
        assert!(constitution.base >= 3 && constitution.base <= 18);
        assert!(dexterity.base >= 3 && dexterity.base <= 18);
        assert!(intelligence.base >= 3 && intelligence.base <= 18);
        assert!(strength.base >= 3 && strength.base <= 18);
        assert!(wisdom.base >= 3 && wisdom.base <= 18);
    }
}
