#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::fmt;

use dice_roller::RollCmd;
use serde::{Deserialize, Serialize};
use strum::Display;

/// List of types of damage available
#[allow(dead_code)]
#[derive(Deserialize, Display, Serialize)]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

/// Damage information for a given object or attack
pub struct Damage {
    /// Type of damage the attack does
    pub damage_type: DamageType,
    /// Modifier to add to the roll
    pub modifier: i16,
    /// Damage roll
    pub roll: RollCmd,
}

impl fmt::Display for Damage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:+} {}", self.roll, self.modifier, self.damage_type)
    }
}

// pub(crate) enum Hit {
//     Roll(u8),
//     DC(AbilityScoreType, i16),
// }

// impl fmt::Display for Hit {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Roll(m) => write!(f, "{:+}", m),
//             Self::DC(a, s) => write!(f, "{} {}", s, a),
//         }
//     }
// }

// pub(crate) struct Attack<'a> {
//     pub(crate) citation: Citation,
//     pub(crate) hit: Hit,
//     pub(crate) damage: Damage,
//     pub(crate) name: &'a str,
//     pub(crate) range: &'a str,
// }

// impl fmt::Display for Attack<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(
//             f,
//             "{:20} {:20} {:20} {:20} {}",
//             self.name, self.range, self.hit, self.damage, self.citation
//         )
//     }
// }

/// Trait to encapuslate resistances
pub trait Resistances {
    /// Return list of immunities for this object
    fn immunities(&self) -> Vec<DamageType> {
        vec![]
    }
    /// Return list of resistances for this object
    fn resistances(&self) -> Vec<DamageType> {
        vec![]
    }
}
