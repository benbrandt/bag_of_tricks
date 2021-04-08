mod acolyte;
mod charlatan;
mod criminal;
mod entertainer;
mod folk_hero;
mod guild_artisan;
mod hermit;
mod noble;
mod outlander;
mod sage;
mod sailor;
mod soldier;
mod urchin;

use std::fmt;

use entertainer::Entertainer;
use folk_hero::FolkHero;
use rand::{
    prelude::{Distribution, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::citation::Citations;

use self::{
    acolyte::Acolyte, charlatan::Charlatan, criminal::Criminal, guild_artisan::GuildArtisan,
    hermit::Hermit, noble::Noble, outlander::Outlander, sage::Sage, sailor::Sailor,
    soldier::Soldier, urchin::Urchin,
};

use super::{
    ability::{weighted_modifiers_dist, AbilityScores, Skill},
    alignment::{AlignmentInfluences, Attitude, Morality},
    backstory::Backstory,
    equipment::StartingEquipment,
    features::Features,
    languages::Languages,
    proficiencies::Proficiencies,
};

/// Types of alignment influence from personality traits
#[derive(Clone, Copy, Deserialize, Display, Serialize)]
pub(crate) enum Influence {
    Any,
    Chaotic,
    Evil,
    Good,
    Lawful,
    Neutral,
}

impl AlignmentInfluences for Influence {
    /// Return attitude influence from personality.
    /// Doubled because personality should be a major indicator of alignment.
    fn attitude(&self) -> Vec<Attitude> {
        match self {
            Self::Chaotic => vec![Attitude::Chaotic; 2],
            Self::Lawful => vec![Attitude::Lawful; 2],
            Self::Neutral => vec![Attitude::Neutral; 2],
            _ => vec![],
        }
    }

    /// Return morality influence from personality.
    /// Doubled because personality should be a major indicator of alignment.
    fn morality(&self) -> Vec<Morality> {
        match self {
            Self::Good => vec![Morality::Good; 2],
            Self::Evil => vec![Morality::Evil; 2],
            Self::Neutral => vec![Morality::Neutral; 2],
            _ => vec![],
        }
    }
}

/// Description of a character's personality
#[derive(Deserialize, Serialize)]
pub(crate) struct Personality {
    bond: String,
    flaw: String,
    ideal: (String, Influence),
    traits: Vec<String>,
}

/// Return attitude/morality influences from character's ideal
impl AlignmentInfluences for Personality {
    fn attitude(&self) -> Vec<Attitude> {
        self.ideal.1.attitude()
    }

    fn morality(&self) -> Vec<Morality> {
        self.ideal.1.morality()
    }
}

impl fmt::Display for Personality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PERSONALITY TRAITS:")?;
        for t in &self.traits {
            writeln!(f, "{}", t)?;
        }
        writeln!(f, "IDEAL: {} ({})", self.ideal.0, self.ideal.1)?;
        writeln!(f, "BOND: {}", self.bond)?;
        writeln!(f, "FLAW: {}", self.flaw)
    }
}

/// Trait to store associated constants for each background's personality tables.
pub(crate) trait PersonalityOptions {
    /// List of 6 bonds to choose from
    const BONDS: [&'static str; 6];
    /// List of 6 flaws to choose from
    const FLAWS: [&'static str; 6];
    /// List of 6 ideals to choose from, and their corresponding alignment influence
    const IDEALS: [(&'static str, Influence); 6];
    /// List of 8 traits to choose from
    const TRAITS: [&'static str; 8];

    /// Generate personality descriptions from the associated constants
    fn gen_personality(rng: &mut impl Rng) -> Personality {
        let ideal = *Self::IDEALS.choose(rng).unwrap();
        Personality {
            bond: String::from(*Self::BONDS.choose(rng).unwrap()),
            flaw: String::from(*Self::FLAWS.choose(rng).unwrap()),
            ideal: (String::from(ideal.0), ideal.1),
            traits: Self::TRAITS
                .choose_multiple(rng, 2)
                .map(|s| String::from(*s))
                .collect(),
        }
    }
}

/// Trait for backgrounds to build from
#[typetag::serde(tag = "type")]
pub(crate) trait Background:
    Backstory + Citations + Features + Languages + Proficiencies + StartingEquipment + fmt::Display
{
    /// Generate a new instance of the background
    fn gen(rng: &mut impl Rng) -> (Box<dyn Background>, Personality)
    where
        Self: Sized;

    /// Return list of skills background gives proficiency in
    fn skills() -> Vec<Skill>
    where
        Self: Sized;
}

/// List of currently supported background options
#[derive(EnumIter)]
pub(crate) enum BackgroundOption {
    Acolyte,
    Charlatan,
    Criminal,
    Entertainer,
    FolkHero,
    GuildArtisan,
    Hermit,
    Noble,
    Outlander,
    Sage,
    Sailor,
    Soldier,
    Urchin,
}

impl BackgroundOption {
    /// Choose a random background option, weighted by proficiency bonuses, and map to corresponding generator
    pub(crate) fn gen(
        rng: &mut impl Rng,
        abilities: &AbilityScores,
    ) -> (Box<dyn Background>, Personality) {
        let options: Vec<BackgroundOption> = Self::iter().collect();
        let modifiers = Self::iter().map(|o| o.skill_modifier(abilities));
        let dist = weighted_modifiers_dist(modifiers);
        match options.get(dist.sample(rng)).unwrap() {
            Self::Acolyte => Acolyte::gen(rng),
            Self::Charlatan => Charlatan::gen(rng),
            Self::Criminal => Criminal::gen(rng),
            Self::Entertainer => Entertainer::gen(rng),
            Self::FolkHero => FolkHero::gen(rng),
            Self::GuildArtisan => GuildArtisan::gen(rng),
            Self::Hermit => Hermit::gen(rng),
            Self::Noble => Noble::gen(rng),
            Self::Outlander => Outlander::gen(rng),
            Self::Sage => Sage::gen(rng),
            Self::Sailor => Sailor::gen(rng),
            Self::Soldier => Soldier::gen(rng),
            Self::Urchin => Urchin::gen(rng),
        }
    }

    /// Max skill modifier of background for weighting
    fn skill_modifier(&self, abilities: &AbilityScores) -> i16 {
        let skills = match self {
            Self::Acolyte => Acolyte::skills(),
            Self::Charlatan => Charlatan::skills(),
            Self::Criminal => Criminal::skills(),
            Self::Entertainer => Entertainer::skills(),
            Self::FolkHero => FolkHero::skills(),
            Self::GuildArtisan => GuildArtisan::skills(),
            Self::Hermit => Hermit::skills(),
            Self::Noble => Noble::skills(),
            Self::Outlander => Outlander::skills(),
            Self::Sage => Sage::skills(),
            Self::Sailor => Sailor::skills(),
            Self::Soldier => Soldier::skills(),
            Self::Urchin => Urchin::skills(),
        };
        skills
            .iter()
            .map(|s| abilities.modifier(s.ability_score_type()))
            .max()
            .unwrap_or(0)
    }
}
