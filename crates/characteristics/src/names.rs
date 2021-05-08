pub mod dragonborn;
pub mod dwarf;
pub mod elf;
pub mod gith;
pub mod gnome;
pub mod goblinoid;
pub mod goliath;
pub mod halfling;
pub mod human;
pub mod kenku;
pub mod kobold;
pub mod lizardfolk;
pub mod orc;
pub mod tabaxi;
pub mod tiefling;
pub mod triton;
pub mod yuan_ti;

use rand::Rng;

use super::{CharacteristicDetails, Characteristics};

/// Trait for entities that need names.
pub trait Name: Characteristics {
    /// Method of generating a random name. Characteristics are provided in case they influence the choice.
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String;
}
