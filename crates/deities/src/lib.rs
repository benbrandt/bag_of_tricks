#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod dragon;
mod dragonlance;
mod dwarven;
mod eberron;
mod elven;
mod forgotten_realms;
mod giant;
mod gnomish;
mod goblinoid;
mod greyhawk;
mod halfling;
mod historical;
mod kobold;
mod lizardfolk;
mod orc;

use std::{f64::consts::E, fmt};

use alignment::{Alignment, Attitude, Morality};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::Display;

use self::{
    dragon::Dragon,
    dragonlance::Dragonlance,
    dwarven::{Duergar, Dwarven},
    eberron::Eberron,
    elven::{Drow, Elven},
    forgotten_realms::ForgottenRealms,
    giant::Giant,
    gnomish::Gnomish,
    goblinoid::{Bugbear, Goblin},
    greyhawk::Greyhawk,
    halfling::Halfling,
    historical::{Celtic, Egyptian, Greek, Norse},
    kobold::Kobold,
    lizardfolk::Lizardfolk,
    orc::Orc,
};

#[derive(Copy, Clone, Deserialize, Display, PartialEq, Serialize)]
pub enum Domain {
    Arcana,
    Death,
    Forge,
    Grave,
    Knowledge,
    Life,
    Light,
    Nature,
    Tempest,
    Trickery,
    War,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Deity<'a> {
    pub name: &'a str,
    pub titles: Vec<&'a str>,
    pub alignment: Alignment,
    pub domains: Vec<Domain>,
    pub symbols: Vec<&'a str>,
}

impl Deity<'_> {
    fn weight(&self, attitude_influences: &[Attitude], morality_influences: &[Morality]) -> f64 {
        self.alignment
            .weight(attitude_influences, morality_influences)
    }
}

impl<'a> fmt::Display for Deity<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CHOSEN DEITY: {}", self.name)?;
        writeln!(f, "Titles: {}", self.titles.join("; "))?;
        writeln!(f, "Alignment: {}", self.alignment)?;
        writeln!(
            f,
            "Domains: {}",
            self.domains
                .iter()
                .map(|d| format!("{}", d))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        writeln!(f, "Symbols: {}", self.symbols.join(", "))
    }
}

pub(crate) trait Deities<'a> {
    fn deities() -> Vec<Deity<'a>>;
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum PantheonWeight {
    Exotic,
    Possible,
    Likely,
}

impl PantheonWeight {
    fn weight(self) -> f64 {
        match self {
            Self::Exotic => E.powi(0),
            Self::Possible => E.powi(2),
            Self::Likely => E.powi(4),
        }
    }
}

#[derive(Copy, Clone, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Pantheon {
    Bugbear,
    Celtic,
    Dragon,
    Dragonlance,
    Drow,
    Duergar,
    Dwarven,
    Eberron,
    Egyptian,
    Elven,
    #[strum(serialize = "Forgotten Realms")]
    ForgottenRealms,
    Giant,
    Gnomish,
    Goblin,
    Greek,
    Greyhawk,
    Halfling,
    Kobold,
    Lizardfolk,
    Norse,
    Orc,
    None,
}

impl Pantheon {
    fn all_deities<'a>(self) -> Vec<Deity<'a>> {
        match self {
            Self::Bugbear => Bugbear::deities(),
            Self::Celtic => Celtic::deities(),
            Self::Dragon => Dragon::deities(),
            Self::Dragonlance => Dragonlance::deities(),
            Self::Drow => Drow::deities(),
            Self::Dwarven => Dwarven::deities(),
            Self::Duergar => Duergar::deities(),
            Self::Eberron => Eberron::deities(),
            Self::Egyptian => Egyptian::deities(),
            Self::Elven => Elven::deities(),
            Self::ForgottenRealms => ForgottenRealms::deities(),
            Self::Giant => Giant::deities(),
            Self::Gnomish => Gnomish::deities(),
            Self::Goblin => Goblin::deities(),
            Self::Greek => Greek::deities(),
            Self::Greyhawk => Greyhawk::deities(),
            Self::Halfling => Halfling::deities(),
            Self::Kobold => Kobold::deities(),
            Self::Lizardfolk => Lizardfolk::deities(),
            Self::Norse => Norse::deities(),
            Self::Orc => Orc::deities(),
            Self::None => vec![],
        }
    }

    #[must_use]
    pub fn deities<'a>(self, domain: Option<Domain>) -> Vec<Deity<'a>> {
        domain.map_or_else(
            || self.all_deities(),
            |d| {
                self.all_deities()
                    .into_iter()
                    .filter(|deity| deity.domains.contains(&d))
                    .collect::<Vec<_>>()
            },
        )
    }

    /// # Panics
    ///
    /// Will panic if no Pantheons available
    pub fn choose(
        rng: &mut impl Rng,
        addl_pantheons: Vec<(Self, PantheonWeight)>,
        domain: Option<Domain>,
        required: bool,
    ) -> Self {
        let mut options = vec![
            (Self::ForgottenRealms, PantheonWeight::Likely),
            (Self::Celtic, PantheonWeight::Exotic),
            (Self::Dragonlance, PantheonWeight::Exotic),
            (Self::Eberron, PantheonWeight::Exotic),
            (Self::Egyptian, PantheonWeight::Exotic),
            (Self::Greek, PantheonWeight::Exotic),
            (Self::Greyhawk, PantheonWeight::Exotic),
            (Self::Norse, PantheonWeight::Exotic),
        ];
        options.extend(addl_pantheons);
        if !required {
            options.push((Self::None, PantheonWeight::Likely));
        }
        domain
            .map_or(options.clone(), |_| {
                options
                    .into_iter()
                    .filter(|(p, _)| p == &Self::None || !p.deities(domain).is_empty())
                    .collect::<Vec<_>>()
            })
            .choose_weighted(rng, |(_, w)| w.weight())
            .unwrap()
            .0
    }

    pub fn choose_deity<'a>(
        self,
        rng: &mut impl Rng,
        attitude_influences: &[Attitude],
        morality_influences: &[Morality],
        domain: Option<Domain>,
    ) -> Option<Deity<'a>> {
        self.deities(domain)
            .choose_weighted(rng, |d| d.weight(attitude_influences, morality_influences))
            .ok()
            .cloned()
    }
}

pub trait Pantheons {
    fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {
        vec![]
    }

    fn deity_required(&self) -> bool {
        false
    }
}
