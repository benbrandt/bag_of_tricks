mod ability;
mod race;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

use ability::AbilityScores;
use race::{gen_race_option, Race};

/// Character information
#[derive(Deserialize, Serialize)]
pub struct Character {
    abilities: AbilityScores,
    race: Box<dyn Race>,
}

impl Character {
    /// Generate a new random character
    pub fn new(rng: &mut impl Rng) -> Self {
        let race = gen_race_option(rng);
        let mut abilities = AbilityScores::new(rng);
        abilities.extend(race.abilities());
        Self { abilities, race }
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
        insta::assert_yaml_snapshot!(character);
        assert_eq!(
            format!("{}", character),
            "\
RACE: Mountain Dwarf (PHB p18,20)

STR  +2 (15)
DEX  +1 (13)
CON  -1 (9)
INT  -1 (9)
WIS  +2 (15)
CHA  +3 (16)
"
        );
    }
}
