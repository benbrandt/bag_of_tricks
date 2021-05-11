#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod ability;
mod background;
mod equipment;
mod features;
mod proficiencies;
mod race;

use std::{fmt, writeln};

use ability::{AbilityScores, Skill};
use alignment::{Alignment, AlignmentInfluences, Attitude, Morality};
use attack::{DamageType, Resistances};
use background::{Background, BackgroundOption};
use backstory::Backstory;
use characteristics::{Appearance, CharacteristicDetails, Speed};
use equipment::{currency::Coin, Equipment, EquipmentOption};
use features::{Feature, Features};
use languages::Language;
use personality::Personality;
use proficiencies::Proficiency;
use race::{Race, RaceOptions};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use trinkets::{TrinketOption, Trinkets};

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
    /// Currency
    coins: (Coin, u8),
    /// Equipment randomly chosen for the character.
    equipment: Vec<Equipment>,
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
        let background = BackgroundOption::gen(
            rng,
            &character.abilities,
            &character.proficiencies,
            character.proficiency_bonus(),
        );
        character.race = Some(race);
        character.name = name;
        character.characteristics = Some(characteristics);
        character.background = Some(background);
        character.gen_personality(rng);
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
        // Choose a trinket
        let mut addl_equipment = vec![EquipmentOption::Trinket(None, None, true)];

        if let Some(background) = self.background.as_ref() {
            self.coins = background.coins();
            self.equipment.extend(background.equipment());
            addl_equipment.extend(background.addl_equipment());
        }

        addl_equipment.sort();
        for option in addl_equipment {
            self.equipment.push(option.gen(
                rng,
                &self.equipment,
                &self.proficiencies,
                &self.trinket_options(),
            ));
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

        let mut addl_languages = vec![];
        if let Some(race) = self.race.as_ref() {
            addl_languages.push(race.addl_languages());
        }
        if let Some(background) = self.background.as_ref() {
            addl_languages.push(background.addl_languages());
        }

        // Handle any dupes across these options
        for l in languages {
            if self.languages.contains(&l) {
                self.languages
                    .extend(Language::gen(rng, &self.languages, (1, None)));
            } else {
                self.languages.push(l);
            }
        }

        for a in addl_languages {
            self.languages
                .extend(Language::gen(rng, &self.languages, a));
        }
    }

    /// Generate personality descriptions from the associated constants
    fn gen_personality(&mut self, rng: &mut impl Rng) {
        let mut bonds = vec![];
        let mut flaws = vec![];
        let mut ideals = vec![];
        let mut traits = vec![];
        if let Some(race) = self.race.as_ref() {
            bonds.extend(race.bonds());
            flaws.extend(race.flaws());
            ideals.extend(race.ideals());
            traits.extend(race.traits());
        }
        if let Some(background) = self.background.as_ref() {
            bonds.extend(background.bonds());
            flaws.extend(background.flaws());
            ideals.extend(background.ideals());
            traits.extend(background.traits());
        }
        self.personality = Some(Personality {
            bond: bonds.choose(rng).unwrap().clone(),
            flaw: flaws.choose(rng).unwrap().clone(),
            ideal: ideals.choose(rng).unwrap().clone(),
            traits: traits.choose_multiple(rng, 2).cloned().collect(),
        });
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
                self.proficiencies.extend(p.gen_replacement(
                    rng,
                    &self.abilities,
                    &self.proficiencies,
                    self.proficiency_bonus(),
                ));
            } else {
                self.proficiencies.push(p);
            }
        }

        // Sort so that the options with the least amount are chosen first.
        addl_proficiencies.sort();
        for option in addl_proficiencies {
            self.proficiencies.extend(option.gen(
                rng,
                &self.abilities,
                &self.proficiencies,
                self.proficiency_bonus(),
            ));
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

impl Trinkets for Character {
    fn trinket_options(&self) -> Vec<TrinketOption> {
        let mut options = vec![TrinketOption::Default];
        if let Some(race) = self.race.as_ref() {
            options.extend(race.trinket_options());
        }
        options.sort();
        options.dedup();
        options
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
                if skill.proficient(&self.proficiencies,) {
                    " X"
                } else {
                    ""
                },
                skill.ability_score_type(),
                skill,
                skill.modifier(
                    &self.abilities,
                    &self.proficiencies,
                    self.proficiency_bonus(),
                ),
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
            self.equipment
                .iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(f, "COINS: {}{}", self.coins.1, self.coins.0)?;
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
    // #[ignore = "linux vs mac differences"]
    fn test_character_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let character = Character::gen(&mut rng);
        insta::assert_display_snapshot!(character);
    }
}
