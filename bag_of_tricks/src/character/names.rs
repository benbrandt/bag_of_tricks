pub(crate) mod dragonborn;
pub(crate) mod dwarf;
pub(crate) mod elf;
pub(crate) mod gnome;
pub(crate) mod goblinoid;
pub(crate) mod goliath;
pub(crate) mod halfling;
pub(crate) mod human;
pub(crate) mod kenku;
pub(crate) mod kobold;
pub(crate) mod lizardfolk;
pub(crate) mod orc;
pub(crate) mod tabaxi;
pub(crate) mod tiefling;
pub(crate) mod triton;
pub(crate) mod yuan_ti;

use rand::Rng;

use super::characteristics::{CharacteristicDetails, Characteristics};

/// Trait for entities that need names.
pub(crate) trait Name: Characteristics {
    /// Method of generating a random name. Characteristics are provided in case they influence the choice.
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String;
}