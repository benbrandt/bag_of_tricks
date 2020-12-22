mod ability;

use ability::AbilityScores;
use rand::Rng;

/// Character information
#[derive(Debug)]
pub struct Character {
    abilities: AbilityScores,
}

impl Character {
    /// Generate a new random character
    pub fn new(rng: &mut impl Rng) -> Self {
        Self {
            abilities: AbilityScores::new(rng),
        }
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
        assert!(charisma.0 >= 3 && charisma.0 <= 18);
        assert!(constitution.0 >= 3 && constitution.0 <= 18);
        assert!(dexterity.0 >= 3 && dexterity.0 <= 18);
        assert!(intelligence.0 >= 3 && intelligence.0 <= 18);
        assert!(strength.0 >= 3 && strength.0 <= 18);
        assert!(wisdom.0 >= 3 && wisdom.0 <= 18);
    }
}
