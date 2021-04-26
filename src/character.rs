mod ability;
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

use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use ability::{AbilityScores, Skill};
use attack::{DamageType, Resistances};
use background::{Background, BackgroundOption, Personality};
use backstory::Backstory;
use characteristics::{CharacteristicDetails, Speed};
use equipment::{currency::Coin, Equipment, EquipmentOption, StartingEquipment};
use features::{Feature, Features};
use languages::Language;
use proficiencies::Proficiency;
use race::{Race, RaceOptions};

use self::characteristics::Appearance;

use super::alignment::{Alignment, AlignmentInfluences, Attitude, Morality};

/// Character information. Mostly stores random choices made for this character.
#[derive(Default, Deserialize, Serialize)]
pub struct Character {
    /// Ability scores for the character.
    abilities: AbilityScores,
    /// The character's alignment.
    alignment: Option<Alignment>,
    /// The character's background choice.
    background: Option<Box<dyn Background>>,
    /// Characteristics of the character.
    characteristics: Option<CharacteristicDetails>,
    /// Equipment randomly chosen for the character.
    chosen_equipment: Vec<Equipment>,
    /// Languages randomly chosen for the character.
    languages: Vec<Language>,
    /// Current level of the character.
    level: u8,
    /// Character's name.
    name: String,
    /// Personality traits of the chracacter.
    personality: Option<Personality>,
    /// Proficiencies for the character.
    proficiencies: Vec<Proficiency>,
    /// Race randomly chosen for the character.
    race: Option<Box<dyn Race>>,
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
    /// 3. Randomly choose a background and personality traits (weighted by highest skill modifiers)
    /// 4. Choose alignment (weighted based on inputs from race and background)
    /// 5. Randomly choose any additional languages
    /// 6. Randomly choose proficiencies, weighted towards optimal ones based on what is known about the character so far
    pub fn gen(rng: &mut impl Rng) -> Self {
        let mut character = Self {
            level: 1,
            ..Self::default()
        };
        let (race, name, characteristics) = RaceOptions::gen(rng);
        let mut abilities = AbilityScores::gen(rng);
        abilities.increase(race.abilities());
        character.abilities = abilities;
        let (background, personality) = BackgroundOption::gen(rng, &character);
        character.race = Some(race);
        character.name = name;
        character.characteristics = Some(characteristics);
        character.background = Some(background);
        character.personality = Some(personality);
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
        let mut languages = vec![];

        if let Some(race) = self.race.as_ref() {
            languages.extend(race.languages());
        }
        if let Some(background) = self.background.as_ref() {
            languages.extend(background.languages());
        }

        let addl_languages = self
            .race
            .as_ref()
            .map(|r| r.addl_languages())
            .unwrap_or_default()
            + self
                .background
                .as_ref()
                .map(|b| b.addl_languages())
                .unwrap_or_default();

        // Handle any dupes across these options
        for l in languages {
            if self.languages.contains(&l) {
                self.languages.extend(Language::gen(rng, self, 1));
            } else {
                self.languages.push(l);
            }
        }

        self.languages
            .extend(Language::gen(rng, self, addl_languages));
    }

    /// Generate additional proficiencies, each looking at the current character sheet.
    ///
    /// They are generated from the most limited sets of options to greatest, to avoid overlapping choices.
    fn gen_proficiences(&mut self, rng: &mut impl Rng) {
        // Static choices
        let mut proficiencies = vec![];
        let mut addl_proficiencies = vec![];
        if let Some(race) = self.race.as_ref() {
            proficiencies.extend(race.proficiencies());
            addl_proficiencies.extend(race.addl_proficiencies());
        }
        if let Some(background) = self.background.as_ref() {
            proficiencies.extend(background.proficiencies());
            addl_proficiencies.extend(background.addl_proficiencies());
        }
        // Handle any dupes across these options
        for p in proficiencies {
            if self.proficiencies.contains(&p) {
                self.proficiencies.extend(p.gen_replacement(rng, self));
            } else {
                self.proficiencies.push(p);
            }
        }

        // Sort so that the options with the least amount are chosen first.
        addl_proficiencies.sort();
        for option in addl_proficiencies {
            self.proficiencies.extend(option.gen(rng, &self));
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

    /// Return the speeds of the character.
    fn speeds(&self) -> Vec<Speed> {
        self.characteristics
            .as_ref()
            .map(|c| c.base_speeds.clone())
            .unwrap_or_default()
    }
}

/// Combine all attitude and morality influences for the character (race and personality)
impl AlignmentInfluences for Character {
    fn attitude(&self) -> Vec<Attitude> {
        let mut attitude = vec![];
        if let Some(race) = self.race.as_ref() {
            attitude.extend(race.attitude());
        }
        if let Some(personality) = self.personality.as_ref() {
            attitude.extend(personality.attitude());
        }
        attitude
    }

    fn morality(&self) -> Vec<Morality> {
        let mut morality = vec![];
        if let Some(race) = self.race.as_ref() {
            morality.extend(race.morality());
        }
        if let Some(personality) = self.personality.as_ref() {
            morality.extend(personality.morality());
        }
        morality
    }
}

impl Appearance for Character {
    fn appearance(&self) -> Vec<String> {
        let mut appearance = vec![];
        if let Some(race) = self.race.as_ref() {
            appearance.extend(race.appearance());
        }
        appearance
    }
}

/// Combine all backstory items for the character.
impl Backstory for Character {
    fn backstory(&self) -> Vec<String> {
        let mut backstory = vec![];
        if let Some(race) = self.race.as_ref() {
            backstory.extend(race.backstory());
        }
        if let Some(background) = self.background.as_ref() {
            backstory.extend(background.backstory());
        }
        backstory
    }
}

/// Combine all features and traits for the characters (race and background)
impl Features for Character {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![];
        if let Some(race) = self.race.as_ref() {
            features.extend(race.features());
        }
        if let Some(background) = self.background.as_ref() {
            features.extend(background.features());
        }
        features
    }
}

/// Combine all resistances the character has.
impl Resistances for Character {
    fn immunities(&self) -> Vec<DamageType> {
        self.race
            .as_ref()
            .map(|r| r.immunities())
            .unwrap_or_default()
    }

    fn resistances(&self) -> Vec<DamageType> {
        self.race
            .as_ref()
            .map(|r| r.resistances())
            .unwrap_or_default()
    }
}

impl StartingEquipment for Character {
    fn coins(&self) -> (Coin, u8) {
        self.background
            .as_ref()
            .map_or((Coin::Gold, 0), |r| r.coins())
    }

    fn equipment(&self) -> Vec<Equipment> {
        let mut equipment = vec![];
        if let Some(background) = self.background.as_ref() {
            equipment.extend(background.equipment());
        }
        equipment.extend(self.chosen_equipment.clone());
        equipment
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        let mut addl_equipment = vec![];
        if let Some(background) = self.background.as_ref() {
            addl_equipment.extend(background.addl_equipment());
        }
        // Choose a trinket
        addl_equipment.push(EquipmentOption::Trinket);
        addl_equipment
    }
}

/// Render a text version of the character. Useful for CLI or other output.
impl fmt::Display for Character {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CHARACTER NAME: {}", self.name)?;
        if let Some(race) = self.race.as_ref() {
            writeln!(f, "RACE: {} ({})", race, race.citations())?;
        }
        if let Some(background) = self.background.as_ref() {
            writeln!(f, "BACKGROUND: {} ({})", background, background.citations())?;
        }
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
        writeln!(f, "SPEED:")?;
        for speed in self.speeds() {
            writeln!(f, "{}", speed)?;
        }
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
            "IMMUNITIES: {}",
            self.immunities()
                .iter()
                .map(|r| format!("{}", r))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(
            f,
            "LANGUAGES: {}",
            self.languages
                .iter()
                .map(|r| format!("{}", r))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(
            f,
            "PROFICIENCIES: {}",
            self.proficiencies
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
        if let Some(characteristics) = self.characteristics.as_ref() {
            writeln!(f, "{}", characteristics)?;
        }
        if let Some(personality) = self.personality.as_ref() {
            writeln!(f, "{}", personality)?;
        }
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
        writeln!(f, "APPEARANCE:")?;
        for appearance in self.appearance() {
            writeln!(f, "{}", appearance)?;
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
