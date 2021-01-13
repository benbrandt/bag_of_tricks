pub(crate) mod dragonborn;
pub(crate) mod dwarf;
pub(crate) mod elf;
pub(crate) mod gnome;
pub(crate) mod halfling;
pub(crate) mod human;
pub(crate) mod orc;

use rand::Rng;

use super::characteristics::Characteristics;

pub(crate) trait Name {
    fn gen_name(rng: &mut impl Rng, characteristics: &Characteristics) -> String;
}
