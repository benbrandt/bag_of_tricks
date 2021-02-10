mod ability;
mod alignment;
mod attack;
mod background;
mod backstory;
mod characteristics;
mod equipment;
mod features;
mod languages;
mod names;
mod proficiencies;
mod race;

use std::{fmt, writeln};

use alignment::{Alignment, AlignmentInfluences, Attitude, Morality};
use background::{Background, BackgroundOptions, Personality};
use backstory::Backstory;
use equipment::{currency::Coin, Equipment, EquipmentOption, StartingEquipment};
use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use ability::{AbilityScores, Skill};
use attack::{DamageType, Resistances};
use characteristics::CharacteristicDetails;
use features::{Feature, Features};
use languages::{Language, Languages};
use proficiencies::{Proficiencies, Proficiency, ProficiencyOption};
use race::{Race, RaceOptions};

/// Character information. Mostly stores random choices made for this character.
#[derive(Deserialize, Serialize)]
pub struct Character {
    /// Ability scores for the character.
    abilities: AbilityScores,
    /// The character's alignment.
    alignment: Option<Alignment>,
    /// The character's background choice.
    background: Box<dyn Background>,
    /// Characteristics of the character.
    characteristics: CharacteristicDetails,
    /// Equipment randomly chosen for the character.
    chosen_equipment: Vec<Equipment>,
    /// Languages randomly chosen for the character.
    chosen_languages: Vec<Language>,
    /// Proficiencies randomly chosen for the character.
    chosen_proficiencies: Vec<Proficiency>,
    /// Current level of the character.
    level: u8,
    /// Character's name.
    name: String,
    /// Personality traits of the chracacter.
    personality: Personality,
    /// Race randomly chosen for the character.
    race: Box<dyn Race>,
}

impl Character {
    /// Generate a new random character
    ///
    /// The methodolgy is to gather together as many static inputs as possible (based on some initial random choices),
    /// and then have those feed into later choices.
    ///
    /// Steps are as follows:
    /// 1. Randomly choose a Race (which also generates a name and some physical characteristics)
    /// 2. Roll ability scores (and apply the racial ability increases)
    /// 3. Randomly choose a background and personality traits.
    /// 4. Choose alignment (weighted based on inputs from race and background)
    /// 5. Randomly choose any additional languages
    /// 6. Randomly choose proficiencies, weighted towards optimal ones based on what is known about the character so far
    pub fn gen(rng: &mut impl Rng) -> Self {
        let (race, name, characteristics) = RaceOptions::gen(rng);
        let mut abilities = AbilityScores::gen(rng);
        abilities.increase(race.abilities());
        let (background, personality) = BackgroundOptions::gen(rng);
        let mut character = Self {
            abilities,
            alignment: None,
            background,
            characteristics,
            chosen_equipment: vec![],
            chosen_languages: vec![],
            chosen_proficiencies: vec![],
            level: 1,
            name,
            personality,
            race,
        };
        character.gen_alignment(rng);
        character.gen_languages(rng);
        character.gen_proficiences(rng);
        character.gen_equipment(rng);
        character
    }

    /// Generate character's alignment, feeding in any inputs we have for attitude and morality.
    fn gen_alignment(&mut self, rng: &mut impl Rng) {
        self.alignment = Some(Alignment::gen(rng, &self.attitude(), &self.morality()));
    }

    /// Generate any additional equipment.
    fn gen_equipment(&mut self, rng: &mut impl Rng) {
        let mut options = self.addl_equipment();
        options.sort();
        for option in options {
            self.chosen_equipment.push(option.gen(rng, &self));
        }
    }

    /// Generate any additional languages, ensuring no overlap with current languages.
    fn gen_languages(&mut self, rng: &mut impl Rng) {
        let amount = self.addl_languages();
        let current = self.languages();
        self.chosen_languages = Language::iter()
            .filter(|l| !current.contains(l))
            .choose_multiple(rng, amount);
    }

    /// Generate additional proficiencies, each looking at the current character sheet.
    ///
    /// They are generated from the most limited sets of options to greatest, to avoid overlapping choices.
    fn gen_proficiences(&mut self, rng: &mut impl Rng) {
        let mut options = self.addl_proficiencies();
        // Sort so that the options with the least amount are chosen first.
        options.sort();
        for option in options {
            self.chosen_proficiencies.push(option.gen(rng, &self));
        }
    }

    /// Return the character's proficiency bonus based on their level.
    fn proficiency_bonus(&self) -> i16 {
        match self.level {
            0..=4 => 2,
            5..=8 => 3,
            9..=12 => 4,
            13..=16 => 5,
            17..=u8::MAX => 6,
        }
    }

    /// Return the walking speed of the character.
    fn speed(&self) -> u8 {
        self.characteristics.base_speed
    }
}

/// Combine all attitude and morality influences for the character (race and personality)
impl AlignmentInfluences for Character {
    fn attitude(&self) -> Vec<Attitude> {
        let mut attitude = self.race.attitude();
        attitude.extend(self.personality.attitude());
        attitude
    }

    fn morality(&self) -> Vec<Morality> {
        let mut morality = self.race.morality();
        morality.extend(self.personality.morality());
        morality
    }
}

/// Combine all backstory items for the character.
impl Backstory for Character {
    fn backstory(&self) -> Vec<String> {
        self.background.backstory()
    }
}

/// Combine all features and traits for the characters (race and background)
impl Features for Character {
    fn features(&self) -> Vec<Feature> {
        let mut features = self.race.features();
        features.extend(self.background.features());
        features
    }
}

/// Combine all languages for the character, both statically assigned and randomly chosen.
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

/// Combine all proficiencies for the character, both statically assigned and randomly chosen.
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

/// Combine all resistances the character has.
impl Resistances for Character {
    fn resistances(&self) -> Vec<DamageType> {
        self.race.resistances()
    }
}

impl StartingEquipment for Character {
    fn coins(&self) -> (Coin, u8) {
        self.background.coins()
    }

    fn equipment(&self) -> Vec<Equipment> {
        let mut equipment = self.background.equipment();
        equipment.extend(self.chosen_equipment.clone());
        equipment
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        self.background.addl_equipment()
    }
}

/// Render a text version of the character. Useful for CLI or other output.
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
        writeln!(f, "ALIGNMENT: {}", self.alignment.as_ref().unwrap())?;
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
                    Proficiency::Armor(_)
                    | Proficiency::Tool(_)
                    | Proficiency::Weapon(_)
                    | Proficiency::Vehicle(_) => Some(format!("{:?}", p)),
                })
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(f)?;
        writeln!(f, "CHARACTERISTICS:")?;
        writeln!(f, "{}", self.characteristics)?;
        writeln!(f, "{}", self.personality)?;
        writeln!(f, "EQUIPMENT")?;
        writeln!(
            f,
            "{}",
            self.equipment()
                .iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(f, "COINS: {}{}", self.coins().1, self.coins().0)?;
        writeln!(f)?;
        writeln!(f, "FEATURES AND TRAITS:")?;
        for feature in self.features() {
            writeln!(f, "- {}", feature)?;
        }
        writeln!(f)?;
        writeln!(f, "BACKSTORY:")?;
        for backstory in self.backstory() {
            writeln!(f, "{}", backstory)?;
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
    fn test_character() {
        let mut rng = Pcg64::seed_from_u64(1);
        let character = Character::gen(&mut rng);
        insta::assert_yaml_snapshot!(character);
    }

    /// Verify that our snapshot remains the same.
    #[test]
    fn test_character_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let character = Character::gen(&mut rng);
        // fmt::Display Snapshot
        insta::assert_snapshot!(format!("{}", character));
    }
}
