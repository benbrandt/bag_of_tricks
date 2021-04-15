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

use giant::Giant;
use historical::{Celtic, Egyptian, Greek, Norse};
use kobold::Kobold;
use lizardfolk::Lizardfolk;
use orc::Orc;
use serde::{Deserialize, Serialize};

use crate::alignment::Alignment;

use self::{
    dragon::Dragon,
    dragonlance::Dragonlance,
    dwarven::{Duergar, Dwarven},
    eberron::Eberron,
    elven::{Drow, Elven},
    forgotten_realms::ForgottenRealms,
    gnomish::Gnomish,
    goblinoid::{Bugbear, Goblin},
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

impl Pantheons {
    fn deities(&self) -> Vec<Deity> {
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
}
