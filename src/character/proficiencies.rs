use strum_macros::Display;

use super::{
    ability::Skill,
    equipment::{
        armor::ArmorType,
        tools::Tool,
        weapons::{WeaponCategory, WeaponType},
    },
};

#[derive(Debug, Display)]
pub(crate) enum WeaponProficiency {
    Category(WeaponCategory),
    Specific(WeaponType),
}

#[derive(Debug, Display)]
pub(crate) enum Proficiency {
    Armor(ArmorType),
    Skill(Skill),
    Tool(Tool),
    Weapon(WeaponProficiency),
}
