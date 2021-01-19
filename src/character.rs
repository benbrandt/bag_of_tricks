mod ability;
mod attack;
mod characteristics;
mod features;
mod languages;
mod names;
mod race;

use attack::{Attack, DamageType};
use features::Feature;
use languages::Language;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{fmt, writeln};

use ability::AbilityScores;
use characteristics::CharacteristicDetails;
use race::{Race, RaceOptions};

/// Character information
#[derive(Deserialize, Serialize)]
pub struct Character {
    abilities: AbilityScores,
    characteristics: CharacteristicDetails,
    level: u8,
    name: String,
    race: Box<dyn Race>,
}

impl Character {
    /// Generate a new random character
    pub fn new(rng: &mut impl Rng) -> Self {
        let (race, name, characteristics) = RaceOptions::gen(rng);
        let mut abilities = AbilityScores::new(rng);
        abilities.extend(race.abilities());
        Self {
            abilities,
            characteristics,
            level: 1,
            name,
            race,
        }
    }

    fn attacks(&self) -> Vec<Attack> {
        self.race.attacks(self)
    }

    fn features(&self) -> Vec<Feature> {
        self.race.features()
    }

    fn languages(&self) -> Vec<Language> {
        self.race.languages()
    }

    fn proficiency_bonus(&self) -> i16 {
        match self.level {
            0..=4 => 2,
            5..=8 => 3,
            9..=12 => 4,
            13..=16 => 5,
            17..=u8::MAX => 6,
        }
    }

    fn resistances(&self) -> Vec<DamageType> {
        self.race.resistances()
    }

    fn speed(&self) -> u8 {
        self.characteristics.base_speed
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CHARACTER NAME: {}", self.name)?;
        writeln!(f, "RACE: {} ({})", self.race, self.race.citations())?;
        writeln!(f, "LEVEL: {}", self.level)?;
        writeln!(f)?;
        writeln!(f, "{}", self.abilities)?;
        writeln!(f, "SPEED: {}", self.speed())?;
        writeln!(f, "PROFICIENCY BONUS: {:+}", self.proficiency_bonus())?;
        writeln!(
            f,
            "RESISTANCES: {}",
            self.resistances()
                .iter()
                .map(|r| format!("{}", r))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(
            f,
            "LANGUAGES: {}",
            self.languages()
                .iter()
                .map(|r| format!("{}", r))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(f, "CHARACTERISTICS:")?;
        writeln!(f, "{}", self.characteristics)?;
        writeln!(f, "ATTACKS:")?;
        writeln!(
            f,
            "{:20} {:20} {:20} {:20}",
            "Attack", "Range", "Hit/DC", "Damage"
        )?;
        for attack in self.attacks() {
            writeln!(f, "{}", attack)?;
        }
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
