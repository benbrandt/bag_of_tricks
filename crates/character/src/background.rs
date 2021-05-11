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

use backstory::Backstory;
use citation::Citations;
use features::Features;
use itertools::Itertools;
use languages::Languages;
use personality::PersonalityOptions;
use rand::{prelude::SliceRandom, Rng};
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    ability::{AbilityScores, Skill},
    equipment::StartingEquipment,
    proficiencies::{Proficiencies, Proficiency},
};

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

pub(crate) fn max_skill_weight(
    skills: &[Skill],
    ability_scores: &AbilityScores,
    proficiencies: &[Proficiency],
    proficiency_bonus: i16,
) -> f64 {
    let mut sorted = skills.iter().sorted_by(|a, b| {
        b.weight(ability_scores, proficiencies, proficiency_bonus)
            .partial_cmp(&a.weight(ability_scores, proficiencies, proficiency_bonus))
            .unwrap()
    });
    let main_weight = sorted.next().map_or(0.0, |s| {
        s.weight(ability_scores, proficiencies, proficiency_bonus)
    });
    // Not exponential, just minor bump for second highest
    let secondary = sorted.next().map_or(0.0, |s| {
        f64::from(
            s.modifier(ability_scores, proficiencies, proficiency_bonus)
                + ability_scores.shift_weight_by(),
        )
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
    fn gen(
        rng: &mut impl Rng,
        ability_scores: &AbilityScores,
        proficiencies: &[Proficiency],
        proficiency_bonus: i16,
    ) -> Box<dyn Background>
    where
        Self: Sized;

    /// Return list of skills background gives proficiency in
    fn skills() -> Vec<Skill>
    where
        Self: Sized;

    /// Max skill modifier of background for weighting
    fn weight(
        ability_scores: &AbilityScores,
        proficiencies: &[Proficiency],
        proficiency_bonus: i16,
    ) -> f64
    where
        Self: Sized,
    {
        max_skill_weight(
            &Self::skills(),
            ability_scores,
            proficiencies,
            proficiency_bonus,
        )
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
    pub(crate) fn gen(
        rng: &mut impl Rng,
        ability_scores: &AbilityScores,
        proficiencies: &[Proficiency],
        proficiency_bonus: i16,
    ) -> Box<dyn Background> {
        let options: Vec<BackgroundOption> = Self::iter().collect();
        let option = options
            .choose_weighted(rng, |o| {
                o.weight(ability_scores, proficiencies, proficiency_bonus)
            })
            .unwrap();
        match option {
            Self::Acolyte => Acolyte::gen(rng, ability_scores, proficiencies, proficiency_bonus),
            Self::Charlatan => {
                Charlatan::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::CityWatch => {
                CityWatch::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::ClanCrafter => {
                ClanCrafter::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::CloisteredScholar => {
                CloisteredScholar::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::Courtier => Courtier::gen(rng, ability_scores, proficiencies, proficiency_bonus),
            Self::Criminal => Criminal::gen(rng, ability_scores, proficiencies, proficiency_bonus),
            Self::Entertainer => {
                Entertainer::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::FactionAgent => {
                FactionAgent::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::FarTraveler => {
                FarTraveler::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::FolkHero => FolkHero::gen(rng, ability_scores, proficiencies, proficiency_bonus),
            Self::GuildArtisan => {
                GuildArtisan::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::Hermit => Hermit::gen(rng, ability_scores, proficiencies, proficiency_bonus),
            Self::Inheritor => {
                Inheritor::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::KnightOfTheOrder => {
                KnightOfTheOrder::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::MercenaryVeteran => {
                MercenaryVeteran::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::Noble => Noble::gen(rng, ability_scores, proficiencies, proficiency_bonus),
            Self::Outlander => {
                Outlander::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::Sage => Sage::gen(rng, ability_scores, proficiencies, proficiency_bonus),
            Self::Sailor => Sailor::gen(rng, ability_scores, proficiencies, proficiency_bonus),
            Self::Soldier => Soldier::gen(rng, ability_scores, proficiencies, proficiency_bonus),
            Self::UrbanBountyHunter => {
                UrbanBountyHunter::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::Urchin => Urchin::gen(rng, ability_scores, proficiencies, proficiency_bonus),
            Self::UthgardtTribeMember => {
                UthgardtTribeMember::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
            Self::WaterdhavianNoble => {
                WaterdhavianNoble::gen(rng, ability_scores, proficiencies, proficiency_bonus)
            }
        }
    }

    /// Max skill modifier of background for weighting
    fn weight(
        &self,
        ability_scores: &AbilityScores,
        proficiencies: &[Proficiency],
        proficiency_bonus: i16,
    ) -> f64 {
        match self {
            Self::Acolyte => Acolyte::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::Charlatan => Charlatan::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::CityWatch => CityWatch::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::ClanCrafter => {
                ClanCrafter::weight(ability_scores, proficiencies, proficiency_bonus)
            }
            Self::CloisteredScholar => {
                CloisteredScholar::weight(ability_scores, proficiencies, proficiency_bonus)
            }
            Self::Courtier => Courtier::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::Criminal => Criminal::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::Entertainer => {
                Entertainer::weight(ability_scores, proficiencies, proficiency_bonus)
            }
            Self::FactionAgent => {
                FactionAgent::weight(ability_scores, proficiencies, proficiency_bonus)
            }
            Self::FarTraveler => {
                FarTraveler::weight(ability_scores, proficiencies, proficiency_bonus)
            }
            Self::FolkHero => FolkHero::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::GuildArtisan => {
                GuildArtisan::weight(ability_scores, proficiencies, proficiency_bonus)
            }
            Self::Hermit => Hermit::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::Inheritor => Inheritor::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::KnightOfTheOrder => {
                KnightOfTheOrder::weight(ability_scores, proficiencies, proficiency_bonus)
            }
            Self::MercenaryVeteran => {
                MercenaryVeteran::weight(ability_scores, proficiencies, proficiency_bonus)
            }
            Self::Noble => Noble::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::Outlander => Outlander::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::Sage => Sage::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::Sailor => Sailor::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::Soldier => Soldier::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::UrbanBountyHunter => {
                UrbanBountyHunter::weight(ability_scores, proficiencies, proficiency_bonus)
            }
            Self::Urchin => Urchin::weight(ability_scores, proficiencies, proficiency_bonus),
            Self::UthgardtTribeMember => {
                UthgardtTribeMember::weight(ability_scores, proficiencies, proficiency_bonus)
            }
            Self::WaterdhavianNoble => {
                WaterdhavianNoble::weight(ability_scores, proficiencies, proficiency_bonus)
            }
        }
    }
}
