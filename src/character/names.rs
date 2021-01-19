pub(crate) mod dragonborn;
pub(crate) mod dwarf;
pub(crate) mod elf;
pub(crate) mod gnome;
pub(crate) mod halfling;
pub(crate) mod human;
pub(crate) mod orc;
pub(crate) mod tiefling;

use rand::Rng;

use super::characteristics::{CharacteristicDetails, Characteristics};

pub(crate) trait Name: Characteristics {
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String;
}
