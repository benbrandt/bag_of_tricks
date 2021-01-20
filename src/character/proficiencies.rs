use serde::{Deserialize, Serialize};
use strum_macros::Display;

use super::{
    ability::Skill,
    equipment::{
        armor::ArmorType,
        tools::Tool,
        weapons::{WeaponCategory, WeaponType},
    },
};

#[derive(Clone, Debug, Deserialize, Display, Serialize)]
pub(crate) enum WeaponProficiency {
    Category(WeaponCategory),
    Specific(WeaponType),
}

#[derive(Clone, Debug, Deserialize, Display, Serialize)]
pub(crate) enum Proficiency {
    Armor(ArmorType),
    Skill(Skill),
    Tool(Tool),
    Weapon(WeaponProficiency),
}

pub(crate) trait Proficiencies {
    fn proficiencies(&self) -> Vec<Proficiency> {
        vec![]
    }
}
