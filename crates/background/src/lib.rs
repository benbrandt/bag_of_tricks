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
mod haunted_one;
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
use citation::{CitationList, Citations};
use deities::{Pantheon, PantheonWeight, Pantheons};
use features::{Feature, Features};
use gear::currency::Coin;
use haunted_one::HauntedOne;
use itertools::Itertools;
use languages::{Language, LanguageType, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScores, Skill},
    equipment::{Equipment, EquipmentOption, StartingEquipment},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
};
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
pub trait Background:
    Backstory
    + Citations
    + Features
    + Languages
    + Pantheons
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
    ) -> Self;

    /// Return list of skills background gives proficiency in
    fn skills() -> Vec<Skill>;

    /// Max skill modifier of background for weighting
    fn weight(
        ability_scores: &AbilityScores,
        proficiencies: &[Proficiency],
        proficiency_bonus: i16,
    ) -> f64 {
        max_skill_weight(
            &Self::skills(),
            ability_scores,
            proficiencies,
            proficiency_bonus,
        )
    }
}

#[impl_enum::with_methods {
    pub fn addl_equipment(&self) -> Vec<EquipmentOption> {}
    pub fn addl_languages(&self) -> (usize, Option<LanguageType>) {}
    pub fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {}
    pub fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {}
    pub fn backstory(&self) -> Vec<String> {}
    pub fn bonds(&self) -> Vec<String> {}
    pub fn citations(&self) -> CitationList {}
    pub fn coins(&self) -> (Coin, u8) {}
    pub fn deity_required(&self) -> bool {}
    pub fn equipment(&self) -> Vec<Equipment> {}
    pub fn features(&self) -> Vec<Feature> {}
    pub fn flaws(&self) -> Vec<String> {}
    pub fn ideals(&self) -> Vec<(String, Influence)> {}
    pub fn languages(&self) -> Vec<Language> {}
    pub fn proficiencies(&self) -> Vec<Proficiency> {}
    pub fn traits(&self) -> Vec<String> {}
    pub fn weight(
        ability_scores: &AbilityScores,
        proficiencies: &[Proficiency],
        proficiency_bonus: i16,
    ) -> f64 {}
}]
/// List of currently supported background options
#[derive(Deserialize, EnumIter, Serialize)]
pub enum BackgroundOption {
    Acolyte(Acolyte),
    Charlatan(Charlatan),
    CityWatch(CityWatch),
    ClanCrafter(ClanCrafter),
    CloisteredScholar(CloisteredScholar),
    Courtier(Courtier),
    Criminal(Criminal),
    Entertainer(Entertainer),
    FactionAgent(FactionAgent),
    FarTraveler(FarTraveler),
    FolkHero(FolkHero),
    GuildArtisan(GuildArtisan),
    Hermit(Hermit),
    HauntedOne(HauntedOne),
    Inheritor(Inheritor),
    KnightOfTheOrder(KnightOfTheOrder),
    MercenaryVeteran(MercenaryVeteran),
    Noble(Noble),
    Outlander(Outlander),
    Sage(Sage),
    Sailor(Sailor),
    Soldier(Soldier),
    UrbanBountyHunter(UrbanBountyHunter),
    Urchin(Urchin),
    UthgardtTribeMember(UthgardtTribeMember),
    WaterdhavianNoble(WaterdhavianNoble),
}

impl BackgroundOption {
    /// Choose a random background option, weighted by proficiency bonuses, and map to corresponding generator
    pub fn gen(
        rng: &mut impl Rng,
        ability_scores: &AbilityScores,
        proficiencies: &[Proficiency],
        proficiency_bonus: i16,
    ) -> Self {
        let options: Vec<BackgroundOption> = Self::iter().collect();
        let option = options
            .choose_weighted(rng, |o| {
                o.weight(ability_scores, proficiencies, proficiency_bonus)
            })
            .unwrap();
        match option {
            Self::Acolyte(_) => Self::Acolyte(Acolyte::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Charlatan(_) => Self::Charlatan(Charlatan::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::CityWatch(_) => Self::CityWatch(CityWatch::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::ClanCrafter(_) => Self::ClanCrafter(ClanCrafter::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::CloisteredScholar(_) => Self::CloisteredScholar(CloisteredScholar::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Courtier(_) => Self::Courtier(Courtier::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Criminal(_) => Self::Criminal(Criminal::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Entertainer(_) => Self::Entertainer(Entertainer::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::FactionAgent(_) => Self::FactionAgent(FactionAgent::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::FarTraveler(_) => Self::FarTraveler(FarTraveler::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::FolkHero(_) => Self::FolkHero(FolkHero::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::GuildArtisan(_) => Self::GuildArtisan(GuildArtisan::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Hermit(_) => Self::Hermit(Hermit::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::HauntedOne(_) => Self::HauntedOne(HauntedOne::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Inheritor(_) => Self::Inheritor(Inheritor::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::KnightOfTheOrder(_) => Self::KnightOfTheOrder(KnightOfTheOrder::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::MercenaryVeteran(_) => Self::MercenaryVeteran(MercenaryVeteran::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Noble(_) => Self::Noble(Noble::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Outlander(_) => Self::Outlander(Outlander::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Sage(_) => Self::Sage(Sage::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Sailor(_) => Self::Sailor(Sailor::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Soldier(_) => Self::Soldier(Soldier::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::UrbanBountyHunter(_) => Self::UrbanBountyHunter(UrbanBountyHunter::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::Urchin(_) => Self::Urchin(Urchin::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::UthgardtTribeMember(_) => Self::UthgardtTribeMember(UthgardtTribeMember::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
            Self::WaterdhavianNoble(_) => Self::WaterdhavianNoble(WaterdhavianNoble::gen(
                rng,
                ability_scores,
                proficiencies,
                proficiency_bonus,
            )),
        }
    }
}

impl fmt::Display for BackgroundOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Acolyte(b) => write!(f, "{}", b),
            Self::Charlatan(b) => write!(f, "{}", b),
            Self::CityWatch(b) => write!(f, "{}", b),
            Self::ClanCrafter(b) => write!(f, "{}", b),
            Self::CloisteredScholar(b) => write!(f, "{}", b),
            Self::Courtier(b) => write!(f, "{}", b),
            Self::Criminal(b) => write!(f, "{}", b),
            Self::Entertainer(b) => write!(f, "{}", b),
            Self::FactionAgent(b) => write!(f, "{}", b),
            Self::FarTraveler(b) => write!(f, "{}", b),
            Self::FolkHero(b) => write!(f, "{}", b),
            Self::GuildArtisan(b) => write!(f, "{}", b),
            Self::Hermit(b) => write!(f, "{}", b),
            Self::HauntedOne(b) => write!(f, "{}", b),
            Self::Inheritor(b) => write!(f, "{}", b),
            Self::KnightOfTheOrder(b) => write!(f, "{}", b),
            Self::MercenaryVeteran(b) => write!(f, "{}", b),
            Self::Noble(b) => write!(f, "{}", b),
            Self::Outlander(b) => write!(f, "{}", b),
            Self::Sage(b) => write!(f, "{}", b),
            Self::Sailor(b) => write!(f, "{}", b),
            Self::Soldier(b) => write!(f, "{}", b),
            Self::UrbanBountyHunter(b) => write!(f, "{}", b),
            Self::Urchin(b) => write!(f, "{}", b),
            Self::UthgardtTribeMember(b) => write!(f, "{}", b),
            Self::WaterdhavianNoble(b) => write!(f, "{}", b),
        }
    }
}
