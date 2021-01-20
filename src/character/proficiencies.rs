use strum_macros::Display;

use super::equipment::{
    armor::ArmorType,
    tools::Tool,
    weapons::{WeaponCategory, WeaponType},
};

#[derive(Debug, Display)]
pub(crate) enum WeaponProficiency {
    Category(WeaponCategory),
    Specific(WeaponType),
}

#[derive(Debug, Display)]
pub(crate) enum Proficiency {
    Armor(ArmorType),
    Tool(Tool),
    Weapon(WeaponProficiency),
}
