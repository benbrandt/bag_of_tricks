#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::fmt;

use alignment::{Alignment, AlignmentInfluences, Attitude, Morality};
use attack::{DamageType, Resistances};
use background::BackgroundOption;
use backstory::Backstory;
use characteristics::{Appearance, CharacteristicDetails, Speed};
use class::ClassOption;
use deities::{Deity, Pantheon, Pantheons};
use features::{Feature, Features};
use gear::currency::Coin;
use languages::Language;
use personality::Personality;
use race::RaceOption;
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScores, Skill},
    equipment::{Equipment, EquipmentOption},
    proficiencies::Proficiency,
};
use strum::IntoEnumIterator;
use trinkets::{TrinketOption, Trinkets};

/// Character information. Mostly stores random choices made for this character.
#[derive(Default, Deserialize, Serialize)]
pub struct Character<'a> {
    /// Ability scores for the character.
    abilities: AbilityScores,
    /// The character's alignment.
    alignment: Option<Alignment>,
    /// The character's background choice.
    background: Option<BackgroundOption>,
    /// Characteristics of the character.
    characteristics: Option<CharacteristicDetails>,
    /// Character's class choice
    class: Option<ClassOption>,
    /// Currency
    coins: (Coin, u8),
    /// Character's chosen deity
    #[serde(borrow)]
    deity: Option<Deity<'a>>,
    /// Equipment randomly chosen for the character.
    equipment: Vec<Equipment>,
    /// Languages randomly chosen for the character.
    languages: Vec<Language>,
    /// Current level of the character.
    level: u8,
    /// Character's name.
    name: String,
    /// Pantheon of Deities this character believes in
    pantheon: Option<Pantheon>,
    /// Personality traits of the chracacter.
    personality: Option<Personality>,
    /// Proficiencies for the character.
    proficiencies: Vec<Proficiency>,
    /// Race randomly chosen for the character.
    race: Option<RaceOption>,
}

impl<'a> Character<'a> {
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
        let race = RaceOption::gen(rng);
        let characteristics = race.gen_characteristics(rng);
        let name = race.gen_name(rng, &characteristics);
        let mut abilities = AbilityScores::gen(rng);
        abilities.increase(race.abilities());
        character.abilities = abilities;
        let background = BackgroundOption::gen(
            rng,
            &character.abilities,
            &character.proficiencies,
            character.proficiency_bonus(),
        );
        let class = ClassOption::gen(rng, &character.abilities);
        character.race = Some(race);
        character.name = name;
        character.characteristics = Some(characteristics);
        character.background = Some(background);
        println!("{}", class);
        character.class = Some(class);
        character.gen_personality(rng);
        character.gen_languages(rng);
        character.gen_deity(rng);
        character.gen_alignment(rng);
        character.gen_proficiences(rng);
        character.gen_equipment(rng);
        character
    }

    /// Generate character's alignment, feeding in any inputs we have for attitude and morality.
    fn gen_alignment(&mut self, rng: &mut impl Rng) {
        self.alignment = Some(Alignment::gen(rng, &self.attitude(), &self.morality()));
    }

    /// Generate a character's pantheon and Deity
    fn gen_deity(&mut self, rng: &mut impl Rng) {
        let mut addl_pantheons = vec![];
        let mut required = vec![];
        let domain = None;
        if let Some(race) = self.race.as_ref() {
            addl_pantheons.extend(race.addl_pantheons());
            required.push(race.deity_required());
        }
        if let Some(class) = self.class.as_ref() {
            addl_pantheons.extend(class.addl_pantheons());
            required.push(class.deity_required());
        }
        if let Some(background) = self.background.as_ref() {
            addl_pantheons.extend(background.addl_pantheons());
            required.push(background.deity_required());
        }
        addl_pantheons.extend(self.languages.iter().flat_map(|l| l.addl_pantheons()));
        let pantheon = Pantheon::choose(rng, addl_pantheons, domain, required.contains(&true));
        self.pantheon = Some(pantheon);
        self.deity = pantheon.choose_deity(rng, &self.attitude(), &self.morality(), domain)
    }

    /// Generate any additional equipment.
    fn gen_equipment(&mut self, rng: &mut impl Rng) {
        // Choose a trinket
        let mut addl_equipment = vec![EquipmentOption::Trinket(None, None, true)];

        if let Some(class) = self.class.as_ref() {
            self.coins = class.coins();
            self.equipment.extend(class.equipment());
            addl_equipment.extend(class.addl_equipment());
        }
        if let Some(background) = self.background.as_ref() {
            self.coins = background.coins();
            self.equipment.extend(background.equipment());
            addl_equipment.extend(background.addl_equipment());
        }

        addl_equipment.sort();
        for option in addl_equipment {
            self.equipment.extend(option.gen(
                rng,
                &self.abilities,
                &self.equipment,
                &self.proficiencies,
                &self.characteristics.as_ref().map(|c| &c.size),
                &self.trinket_options(),
            ));
        }
        self.equipment.sort();
    }

    /// Generate any additional languages, ensuring no overlap with current languages.
    fn gen_languages(&mut self, rng: &mut impl Rng) {
        let mut languages = vec![];
        let mut addl_languages = vec![];

        if let Some(race) = self.race.as_ref() {
            languages.extend(race.languages());
            addl_languages.push(race.addl_languages());
        }
        if let Some(class) = self.class.as_ref() {
            languages.extend(class.languages());
            addl_languages.push(class.addl_languages());
        }
        if let Some(background) = self.background.as_ref() {
            languages.extend(background.languages());
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

    fn add_or_replace_proficiencies(
        &mut self,
        rng: &mut impl Rng,
        proficiencies: Vec<Proficiency>,
    ) {
        for p in proficiencies {
            if self.proficiencies.contains(&p) {
                let replacements = p.gen_replacement(
                    rng,
                    &self.abilities,
                    &self.proficiencies,
                    self.proficiency_bonus(),
                );
                self.add_or_replace_proficiencies(rng, replacements);
            } else {
                self.proficiencies.push(p);
            }
        }
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
        if let Some(class) = self.class.as_ref() {
            proficiencies.extend(class.proficiencies());
            addl_proficiencies.extend(class.addl_proficiencies());
        }
        // Handle any dupes across these options
        self.add_or_replace_proficiencies(rng, proficiencies);

        // Sort so that the options with the least amount are chosen first.
        addl_proficiencies.sort();
        for option in addl_proficiencies {
            let choices = option.gen(
                rng,
                &self.abilities,
                &self.proficiencies,
                self.proficiency_bonus(),
            );
            self.add_or_replace_proficiencies(rng, choices);
        }
        self.proficiencies.sort();
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
impl<'a> AlignmentInfluences for Character<'a> {
    fn attitude(&self) -> Vec<Attitude> {
        let mut attitude = vec![];
        if let Some(race) = self.race.as_ref() {
            attitude.extend(race.attitude());
        }
        if let Some(personality) = self.personality.as_ref() {
            attitude.extend(personality.attitude());
        }
        if let Some(deity) = self.deity.as_ref() {
            attitude.extend(deity.alignment.attitude());
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
        if let Some(deity) = self.deity.as_ref() {
            morality.extend(deity.alignment.morality());
        }
        morality
    }
}

impl<'a> Appearance for Character<'a> {
    fn appearance(&self) -> Vec<String> {
        let mut appearance = vec![];
        if let Some(race) = self.race.as_ref() {
            appearance.extend(race.appearance());
        }
        appearance
    }
}

/// Combine all backstory items for the character.
impl<'a> Backstory for Character<'a> {
    fn backstory(&self) -> Vec<String> {
        let mut backstory = vec![];
        if let Some(race) = self.race.as_ref() {
            backstory.extend(race.backstory());
        }
        if let Some(class) = self.class.as_ref() {
            backstory.extend(class.backstory());
        }
        if let Some(background) = self.background.as_ref() {
            backstory.extend(background.backstory());
        }
        backstory
    }
}

/// Combine all features and traits for the characters (race and background)
impl<'a> Features for Character<'a> {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![];
        if let Some(race) = self.race.as_ref() {
            features.extend(race.features());
        }
        if let Some(class) = self.class.as_ref() {
            features.extend(class.features());
        }
        if let Some(background) = self.background.as_ref() {
            features.extend(background.features());
        }
        features
    }
}

/// Combine all resistances the character has.
impl<'a> Resistances for Character<'a> {
    fn immunities(&self) -> Vec<DamageType> {
        self.race
            .as_ref()
            .map(RaceOption::immunities)
            .unwrap_or_default()
    }

    fn resistances(&self) -> Vec<DamageType> {
        self.race
            .as_ref()
            .map(RaceOption::resistances)
            .unwrap_or_default()
    }
}

impl<'a> Trinkets for Character<'a> {
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
impl<'a> fmt::Display for Character<'a> {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CHARACTER NAME: {}", self.name)?;
        if let Some(race) = self.race.as_ref() {
            writeln!(f, "RACE: {} ({})", race, race.citations())?;
        }
        if let Some(class) = self.class.as_ref() {
            writeln!(f, "CLASS: {} ({})", class, class.citations())?;
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
        writeln!(f)?;
        writeln!(f, "CHARACTERISTICS:")?;
        if let Some(characteristics) = self.characteristics.as_ref() {
            writeln!(f, "{}", characteristics)?;
        }
        if let Some(personality) = self.personality.as_ref() {
            writeln!(f, "{}", personality)?;
        }
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
        writeln!(f)?;
        if let Some(pantheon) = self.pantheon.as_ref() {
            writeln!(f, "PANTHEON: {}", pantheon)?;
        }
        if let Some(deity) = self.deity.as_ref() {
            writeln!(f, "{}", deity)?;
        }
        writeln!(
            f,
            "PROFICIENCIES: {}",
            self.proficiencies
                .iter()
                .filter_map(|p| match p {
                    Proficiency::Skill(_) => None,
                    Proficiency::Armor(_)
                    | Proficiency::SavingThrow(_)
                    | Proficiency::Tool(_)
                    | Proficiency::Weapon(_)
                    | Proficiency::Vehicle(_) => Some(format!("{:?}", p)),
                })
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(f)?;
        writeln!(f, "EQUIPMENT")?;
        for equipment in &self.equipment {
            writeln!(f, "{}", equipment)?;
        }
        writeln!(f)?;
        writeln!(f, "COINS: {}{}", self.coins.1, self.coins.0)?;
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
