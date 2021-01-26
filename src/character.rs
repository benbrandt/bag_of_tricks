mod ability;
mod alignment;
mod attack;
mod background;
mod characteristics;
mod equipment;
mod features;
mod languages;
mod names;
mod proficiencies;
mod race;

use std::{fmt, writeln};

use background::{Background, BackgroundOptions, Personality};
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use ability::{AbilityScores, Skill};
use attack::{DamageType, Resistances};
use characteristics::CharacteristicDetails;
use features::{Feature, Features};
use languages::{Language, Languages};
use proficiencies::{Proficiencies, Proficiency};
use race::{Race, RaceOptions};

use self::proficiencies::ProficiencyOption;

/// Character information
#[derive(Deserialize, Serialize)]
pub struct Character {
    abilities: AbilityScores,
    background: Box<dyn Background>,
    characteristics: CharacteristicDetails,
    chosen_languages: Vec<Language>,
    chosen_proficiencies: Vec<Proficiency>,
    level: u8,
    name: String,
    personality: Personality,
    race: Box<dyn Race>,
}

impl Character {
    /// Generate a new random character
    pub fn gen(rng: &mut impl Rng) -> Self {
        let (race, name, characteristics) = RaceOptions::gen(rng);
        let mut abilities = AbilityScores::gen(rng);
        abilities.increase(race.abilities());
        let (background, personality) = BackgroundOptions::gen(rng);
        let mut character = Self {
            abilities,
            background,
            characteristics,
            chosen_languages: vec![],
            chosen_proficiencies: vec![],
            level: 1,
            name,
            personality,
            race,
        };
        character.gen_languages(rng);
        character.gen_proficiences(rng);
        character
    }

    fn gen_languages(&mut self, rng: &mut impl Rng) {
        let amount = self.addl_languages();
        let current = self.languages();
        self.chosen_languages = Language::iter()
            .filter(|l| !current.contains(l))
            .choose_multiple(rng, amount);
    }

    fn gen_proficiences(&mut self, rng: &mut impl Rng) {
        let mut options = self.addl_proficiencies();
        // Sort so that the options with the least amount are chosen first.
        options.sort();
        for option in options {
            self.chosen_proficiencies.push(option.gen(rng, &self));
        }
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
        let mut features = self.race.features();
        features.extend(self.background.features());
        features
    }
}

impl Languages for Character {
    fn languages(&self) -> Vec<Language> {
        let mut languages = self.race.languages();
        languages.extend(self.background.languages());
        languages.extend(self.chosen_languages.clone());
        languages
    }

    fn addl_languages(&self) -> usize {
        self.race.addl_languages() + self.background.addl_languages()
    }
}

impl Proficiencies for Character {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = self.race.proficiencies();
        proficiencies.extend(self.background.proficiencies());
        proficiencies.extend(self.chosen_proficiencies.clone());
        proficiencies
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        let mut addl_proficiencies = self.race.addl_proficiencies();
        addl_proficiencies.extend(self.background.addl_proficiencies());
        addl_proficiencies
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
        writeln!(
            f,
            "BACKGROUND: {} ({})",
            self.background,
            self.background.citations()
        )?;
        writeln!(f, "LEVEL: {}", self.level)?;
        writeln!(f)?;
        writeln!(f, "{}", self.abilities)?;
        writeln!(f, "SKILLS:")?;
        writeln!(f, "PROF  MOD  SKILL            BONUS:")?;
        for skill in Skill::iter() {
            writeln!(
                f,
                "{:4}  {}  {:15}  {:+}",
                if skill.proficient(self) { " X" } else { "" },
                skill.ability_score_type(),
                skill,
                skill.modifier(self),
            )?;
        }
        writeln!(f)?;
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
                .filter_map(|p| match p {
                    Proficiency::Skill(_) => None,
                    Proficiency::Armor(_) | Proficiency::Tool(_) | Proficiency::Weapon(_) =>
                        Some(format!("{:?}", p)),
                })
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(f, "CHARACTERISTICS:")?;
        writeln!(f, "{}", self.characteristics)?;
        writeln!(f, "{}", self.personality)?;
        writeln!(f, "EQUIPMENT")?;
        writeln!(f, "{}", self.background.equipment())?;
        writeln!(f, "FEATURES AND TRAITS:")?;
        for feature in self.features() {
            writeln!(f, "- {}", feature)?;
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
