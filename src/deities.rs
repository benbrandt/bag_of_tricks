mod dwarven;
mod forgotten_realms;

use serde::{Deserialize, Serialize};

use crate::alignment::Alignment;

use self::{
    dwarven::{Duergar, Dwarven},
    forgotten_realms::ForgottenRealms,
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
    description: &'static str,
    alignment: Alignment,
    domains: Vec<Domain>,
    symbol: &'static str,
}

pub(crate) trait Pantheon {
    fn deities() -> Vec<Deity>;
}

pub(crate) enum Pantheons {
    Duergar,
    Dwarven,
    ForgottenRealms,
}

impl Pantheons {
    fn deities(&self) -> Vec<Deity> {
        match self {
            Self::Dwarven => Dwarven::deities(),
            Self::Duergar => Duergar::deities(),
            Self::ForgottenRealms => ForgottenRealms::deities(),
        }
    }
}
