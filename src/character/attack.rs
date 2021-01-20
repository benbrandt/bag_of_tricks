use std::fmt;

use strum_macros::Display;

use crate::dice_roller::RollCmd;

#[allow(dead_code)]
#[derive(Display)]
pub(crate) enum DamageType {
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

pub(crate) struct Damage {
    pub(crate) damage_type: DamageType,
    pub(crate) modifier: i16,
    pub(crate) roll: RollCmd,
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
