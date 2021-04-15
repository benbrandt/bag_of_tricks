mod dwarven;
mod elven;
mod forgotten_realms;
mod halfling;

use serde::{Deserialize, Serialize};

use crate::alignment::Alignment;

use self::{
    dwarven::{Duergar, Dwarven},
    elven::{Drow, Elven},
    forgotten_realms::ForgottenRealms,
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
    symbol: &'static str,
}

pub(crate) trait Pantheon {
    fn deities() -> Vec<Deity>;
}

pub(crate) enum Pantheons {
    Drow,
    Duergar,
    Dwarven,
    Elven,
    ForgottenRealms,
    Halfling,
}

impl Pantheons {
    fn deities(&self) -> Vec<Deity> {
        match self {
            Self::Drow => Drow::deities(),
            Self::Dwarven => Dwarven::deities(),
            Self::Duergar => Duergar::deities(),
            Self::Elven => Elven::deities(),
            Self::ForgottenRealms => ForgottenRealms::deities(),
            Self::Halfling => Halfling::deities(),
        }
    }
}
