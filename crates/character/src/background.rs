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
mod mercenary_veteran;
mod noble;
mod outlander;
mod sage;
mod sailor;
mod soldier;
mod urban_bounty_hunter;
mod urchin;
mod uthgardt_tribe_member;
mod waterdhavian_noble;

use std::fmt;

use citation::Citations;
use itertools::Itertools;
use languages::Languages;
use personality::PersonalityOptions;
use rand::{prelude::SliceRandom, Rng};
use strum::{EnumIter, IntoEnumIterator};

use self::{
    acolyte::Acolyte, charlatan::Charlatan, city_watch::CityWatch, clan_crafter::ClanCrafter,
    cloistered_scholar::CloisteredScholar, courtier::Courtier, criminal::Criminal,
    entertainer::Entertainer, faction_agent::FactionAgent, far_traveler::FarTraveler,
    folk_hero::FolkHero, guild_artisan::GuildArtisan, hermit::Hermit, inheritor::Inheritor,
    knight_of_the_order::KnightOfTheOrder, mercenary_veteran::MercenaryVeteran, noble::Noble,
    outlander::Outlander, sage::Sage, sailor::Sailor, soldier::Soldier,
    urban_bounty_hunter::UrbanBountyHunter, urchin::Urchin,
    uthgardt_tribe_member::UthgardtTribeMember, waterdhavian_noble::WaterdhavianNoble,
};

use super::{
    ability::Skill, backstory::Backstory, equipment::StartingEquipment, features::Features,
    proficiencies::Proficiencies, Character,
};

pub(crate) fn max_skill_weight(skills: &[Skill], character: &Character) -> f64 {
    let mut sorted = skills.iter().sorted_by(|a, b| {
        b.weight(character)
            .partial_cmp(&a.weight(character))
            .unwrap()
    });
    let main_weight = sorted.next().map_or(0.0, |s| s.weight(character));
    // Not exponential, just minor bump for second highest
    let secondary = sorted.next().map_or(0.0, |s| {
        f64::from(s.modifier(character) + character.abilities.shift_weight_by())
    });
    main_weight + secondary
}

/// Trait for backgrounds to build from
#[typetag::serde(tag = "type")]
pub(crate) trait Background:
    Backstory
    + Citations
    + Features
    + Languages
    + PersonalityOptions
    + Proficiencies
    + StartingEquipment
    + fmt::Display
{
    /// Generate a new instance of the background
    fn gen(rng: &mut impl Rng, character: &Character) -> Box<dyn Background>
    where
        Self: Sized;

    /// Return list of skills background gives proficiency in
    fn skills() -> Vec<Skill>
    where
        Self: Sized;

    /// Max skill modifier of background for weighting
    fn weight(character: &Character) -> f64
    where
        Self: Sized,
    {
        max_skill_weight(&Self::skills(), character)
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
    MercenaryVeteran,
    Noble,
    Outlander,
    Sage,
    Sailor,
    Soldier,
    UrbanBountyHunter,
    Urchin,
    UthgardtTribeMember,
    WaterdhavianNoble,
}

impl BackgroundOption {
    /// Choose a random background option, weighted by proficiency bonuses, and map to corresponding generator
    pub(crate) fn gen(rng: &mut impl Rng, character: &Character) -> Box<dyn Background> {
        let options: Vec<BackgroundOption> = Self::iter().collect();
        let option = options
            .choose_weighted(rng, |o| o.weight(character))
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
            Self::MercenaryVeteran => MercenaryVeteran::gen(rng, character),
            Self::Noble => Noble::gen(rng, character),
            Self::Outlander => Outlander::gen(rng, character),
            Self::Sage => Sage::gen(rng, character),
            Self::Sailor => Sailor::gen(rng, character),
            Self::Soldier => Soldier::gen(rng, character),
            Self::UrbanBountyHunter => UrbanBountyHunter::gen(rng, character),
            Self::Urchin => Urchin::gen(rng, character),
            Self::UthgardtTribeMember => UthgardtTribeMember::gen(rng, character),
            Self::WaterdhavianNoble => WaterdhavianNoble::gen(rng, character),
        }
    }

    /// Max skill modifier of background for weighting
    fn weight(&self, character: &Character) -> f64 {
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
            Self::MercenaryVeteran => MercenaryVeteran::weight(character),
            Self::Noble => Noble::weight(character),
            Self::Outlander => Outlander::weight(character),
            Self::Sage => Sage::weight(character),
            Self::Sailor => Sailor::weight(character),
            Self::Soldier => Soldier::weight(character),
            Self::UrbanBountyHunter => UrbanBountyHunter::weight(character),
            Self::Urchin => Urchin::weight(character),
            Self::UthgardtTribeMember => UthgardtTribeMember::weight(character),
            Self::WaterdhavianNoble => WaterdhavianNoble::weight(character),
        }
    }
}
