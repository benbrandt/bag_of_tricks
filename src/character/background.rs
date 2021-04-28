mod acolyte;
mod charlatan;
mod city_watch;
mod clan_crafter;
mod cloistered_scholar;
mod courtier;
mod criminal;
mod entertainer;
mod faction_agent;
mod far_traveler;
mod folk_hero;
mod guild_artisan;
mod hermit;
mod inheritor;
mod knight_of_the_order;
mod noble;
mod outlander;
mod sage;
mod sailor;
mod soldier;
mod urchin;

use std::fmt;

use entertainer::Entertainer;
use folk_hero::FolkHero;
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    alignment::{AlignmentInfluences, Attitude, Morality},
    citation::Citations,
};

use self::{
    acolyte::Acolyte, charlatan::Charlatan, city_watch::CityWatch, clan_crafter::ClanCrafter,
    cloistered_scholar::CloisteredScholar, courtier::Courtier, criminal::Criminal,
    faction_agent::FactionAgent, far_traveler::FarTraveler, guild_artisan::GuildArtisan,
    hermit::Hermit, inheritor::Inheritor, knight_of_the_order::KnightOfTheOrder, noble::Noble,
    outlander::Outlander, sage::Sage, sailor::Sailor, soldier::Soldier, urchin::Urchin,
};

use super::{
    ability::{modifier_shift, modifier_weight, Skill},
    backstory::Backstory,
    equipment::StartingEquipment,
    features::Features,
    languages::Languages,
    proficiencies::Proficiencies,
    Character,
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
    /// List of bonds to choose from
    const BONDS: &'static [&'static str];
    /// List of flaws to choose from
    const FLAWS: &'static [&'static str];
    /// List of ideals to choose from, and their corresponding alignment influence
    const IDEALS: &'static [(&'static str, Influence)];
    /// List of traits to choose from
    const TRAITS: &'static [&'static str];

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

pub(crate) fn skill_weight(skills: &[Skill], character: &Character) -> i16 {
    skills
        .iter()
        .map(|s| character.abilities.modifier(s.ability_score_type()))
        .max()
        .unwrap_or(0)
}

/// Trait for backgrounds to build from
#[typetag::serde(tag = "type")]
pub(crate) trait Background:
    Backstory + Citations + Features + Languages + Proficiencies + StartingEquipment + fmt::Display
{
    /// Generate a new instance of the background
    fn gen(rng: &mut impl Rng, character: &Character) -> (Box<dyn Background>, Personality)
    where
        Self: Sized;

    /// Return list of skills background gives proficiency in
    fn skills() -> Vec<Skill>
    where
        Self: Sized;

    /// Max skill modifier of background for weighting
    fn weight(character: &Character) -> i16
    where
        Self: Sized,
    {
        skill_weight(&Self::skills(), character)
    }
}

/// List of currently supported background options
#[derive(EnumIter)]
pub(crate) enum BackgroundOption {
    Acolyte,
    Charlatan,
    CityWatch,
    ClanCrafter,
    CloisteredScholar,
    Courtier,
    Criminal,
    Entertainer,
    FactionAgent,
    FarTraveler,
    FolkHero,
    GuildArtisan,
    Hermit,
    Inheritor,
    KnightOfTheOrder,
    // MercenaryVeteran 152
    Noble,
    Outlander,
    Sage,
    Sailor,
    Soldier,
    // UrbanBountyHunter 153
    Urchin,
    // UthgardtTribeMember 153, 154
    // WaterdhavianNoble 154
}

impl BackgroundOption {
    /// Choose a random background option, weighted by proficiency bonuses, and map to corresponding generator
    pub(crate) fn gen(
        rng: &mut impl Rng,
        character: &Character,
    ) -> (Box<dyn Background>, Personality) {
        let options: Vec<BackgroundOption> = Self::iter().collect();
        let shift = modifier_shift(Self::iter().map(|o| o.weight(character)));
        let option = options
            .choose_weighted(rng, |o| modifier_weight(o.weight(character), shift))
            .unwrap();
        match option {
            Self::Acolyte => Acolyte::gen(rng, character),
            Self::Charlatan => Charlatan::gen(rng, character),
            Self::CityWatch => CityWatch::gen(rng, character),
            Self::ClanCrafter => ClanCrafter::gen(rng, character),
            Self::CloisteredScholar => CloisteredScholar::gen(rng, character),
            Self::Courtier => Courtier::gen(rng, character),
            Self::Criminal => Criminal::gen(rng, character),
            Self::Entertainer => Entertainer::gen(rng, character),
            Self::FactionAgent => FactionAgent::gen(rng, character),
            Self::FarTraveler => FarTraveler::gen(rng, character),
            Self::FolkHero => FolkHero::gen(rng, character),
            Self::GuildArtisan => GuildArtisan::gen(rng, character),
            Self::Hermit => Hermit::gen(rng, character),
            Self::Inheritor => Inheritor::gen(rng, character),
            Self::KnightOfTheOrder => KnightOfTheOrder::gen(rng, character),
            Self::Noble => Noble::gen(rng, character),
            Self::Outlander => Outlander::gen(rng, character),
            Self::Sage => Sage::gen(rng, character),
            Self::Sailor => Sailor::gen(rng, character),
            Self::Soldier => Soldier::gen(rng, character),
            Self::Urchin => Urchin::gen(rng, character),
        }
    }

    /// Max skill modifier of background for weighting
    fn weight(&self, character: &Character) -> i16 {
        match self {
            Self::Acolyte => Acolyte::weight(character),
            Self::Charlatan => Charlatan::weight(character),
            Self::CityWatch => CityWatch::weight(character),
            Self::ClanCrafter => ClanCrafter::weight(character),
            Self::CloisteredScholar => CloisteredScholar::weight(character),
            Self::Courtier => Courtier::weight(character),
            Self::Criminal => Criminal::weight(character),
            Self::Entertainer => Entertainer::weight(character),
            Self::FactionAgent => FactionAgent::weight(character),
            Self::FarTraveler => FarTraveler::weight(character),
            Self::FolkHero => FolkHero::weight(character),
            Self::GuildArtisan => GuildArtisan::weight(character),
            Self::Hermit => Hermit::weight(character),
            Self::Inheritor => Inheritor::weight(character),
            Self::KnightOfTheOrder => KnightOfTheOrder::weight(character),
            Self::Noble => Noble::weight(character),
            Self::Outlander => Outlander::weight(character),
            Self::Sage => Sage::weight(character),
            Self::Sailor => Sailor::weight(character),
            Self::Soldier => Soldier::weight(character),
            Self::Urchin => Urchin::weight(character),
        }
    }
}
