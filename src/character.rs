mod ability;
mod features;
mod race;

use features::Feature;
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

    fn features(&self) -> Vec<Feature> {
        self.race.features()
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RACE: {} ({})\n", self.race, self.race.citations())?;
        writeln!(f, "{}", self.abilities)?;
        writeln!(f, "FEATURES AND TRAITS:")?;
        for feature in self.features() {
            writeln!(f, "{}", feature)?;
        }
        write!(f, "")
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
        // Struct Snapshot
        insta::assert_yaml_snapshot!(character);
        // fmt::Display Snapshot
        insta::assert_snapshot!(format!("{}", character));
    }
}
