mod dragonlance;
mod dwarven;
mod eberron;
mod elven;
mod forgotten_realms;
mod gnomish;
mod greyhawk;
mod halfling;
mod orc;

use serde::{Deserialize, Serialize};

use crate::alignment::Alignment;

use self::{
    dragonlance::Dragonlance,
    dwarven::{Duergar, Dwarven},
    eberron::Eberron,
    elven::{Drow, Elven},
    forgotten_realms::ForgottenRealms,
    gnomish::Gnomish,
    greyhawk::Greyhawk,
    halfling::Halfling,
};

#[derive(Deserialize, Serialize)]
pub(crate) enum Domain {
    Arcana,
    Death,
    Knowledge,
    Life,
    Light,
    Nature,
    Tempest,
    Trickery,
    War,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Deity {
    name: &'static str,
    titles: Vec<&'static str>,
    alignment: Alignment,
    domains: Vec<Domain>,
    symbols: Vec<&'static str>,
}

pub(crate) trait Pantheon {
    fn deities() -> Vec<Deity>;
}

pub(crate) enum Pantheons {
    Dragonlance,
    Drow,
    Duergar,
    Dwarven,
    Eberron,
    Elven,
    ForgottenRealms,
    Gnomish,
    Greyhawk,
    Halfling,
}

impl Pantheons {
    fn deities(&self) -> Vec<Deity> {
        match self {
            Self::Dragonlance => Dragonlance::deities(),
            Self::Drow => Drow::deities(),
            Self::Dwarven => Dwarven::deities(),
            Self::Duergar => Duergar::deities(),
            Self::Eberron => Eberron::deities(),
            Self::Elven => Elven::deities(),
            Self::ForgottenRealms => ForgottenRealms::deities(),
            Self::Gnomish => Gnomish::deities(),
            Self::Greyhawk => Greyhawk::deities(),
            Self::Halfling => Halfling::deities(),
        }
    }
}
