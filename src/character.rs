mod ability;
mod alignment;
mod attack;
mod characteristics;
mod equipment;
mod features;
mod languages;
mod names;
mod proficiencies;
mod race;

use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use ability::AbilityScores;
use attack::{DamageType, Resistances};
use characteristics::CharacteristicDetails;
use features::{Feature, Features};
use languages::{Language, Languages};
use proficiencies::{Proficiencies, Proficiency};
use race::{Race, RaceOptions};

/// Character information
#[derive(Deserialize, Serialize)]
pub struct Character {
    abilities: AbilityScores,
    addl_languages: Vec<Language>,
    characteristics: CharacteristicDetails,
    level: u8,
    name: String,
    race: Box<dyn Race>,
}

impl Character {
    /// Generate a new random character
    pub fn gen(rng: &mut impl Rng) -> Self {
        let (race, name, characteristics) = RaceOptions::gen(rng);
        let mut abilities = AbilityScores::new(rng);
        abilities.extend(race.abilities());
        let mut character = Self {
            abilities,
            addl_languages: vec![],
            characteristics,
            level: 1,
            name,
            race,
        };
        character.gen_languages(rng);
        character
    }

    fn gen_languages(&mut self, rng: &mut impl Rng) {
        let amount = self.addl_languages();
        let current = self.languages();
        self.addl_languages = Language::iter()
            .filter(|l| !current.contains(l))
            .choose_multiple(rng, amount);
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

    fn speed(&self) -> u8 {
        self.characteristics.base_speed
    }
}

impl Features for Character {
    fn features(&self) -> Vec<Feature> {
        self.race.features()
    }
}

impl Languages for Character {
    fn languages(&self) -> Vec<Language> {
        let mut languages = self.race.languages();
        languages.extend(self.addl_languages.clone());
        languages
    }

    fn addl_languages(&self) -> usize {
        self.race.addl_languages()
    }
}

impl Proficiencies for Character {
    fn proficiencies(&self) -> Vec<Proficiency> {
        self.race.proficiencies()
    }
}

impl Resistances for Character {
    fn resistances(&self) -> Vec<DamageType> {
        self.race.resistances()
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
        writeln!(
            f,
            "PROFICIENCIES: {}",
            self.proficiencies()
                .iter()
                .map(|r| format!("{:?}", r))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(f, "CHARACTERISTICS:")?;
        writeln!(f, "{}", self.characteristics)?;
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
        let character = Character::gen(&mut rng);
        // Struct Snapshot
        insta::assert_yaml_snapshot!(character);
        // fmt::Display Snapshot
        insta::assert_snapshot!(format!("{}", character));
    }
}
