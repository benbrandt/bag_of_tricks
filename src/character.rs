mod ability;
mod race;

use rand::Rng;
use std::fmt;

use ability::AbilityScores;
use race::{gen_race_option, Race};

/// Character information
pub struct Character {
    abilities: AbilityScores,
    race: Box<dyn Race>,
}

impl Character {
    /// Generate a new random character
    pub fn new(rng: &mut impl Rng) -> Self {
        let race = gen_race_option(rng);
        Self {
            abilities: AbilityScores::new(rng, &race.increases()),
            race,
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RACE: {} ({})\n", self.race, self.race.citations())?;
        write!(f, "{}", self.abilities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    /// Verify that our snapshot remains the same.
    #[test]
    fn test_character_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let character = Character::new(&mut rng);
        assert_eq!(
            format!("{}", character),
            "\
RACE: Mountain Dwarf (PHB p18,20)

STR  +3 (17)
DEX  -2 (7)
CON  +2 (15)
INT  -1 (9)
WIS  +3 (16)
CHA  +1 (13)
"
        );
    }
}
