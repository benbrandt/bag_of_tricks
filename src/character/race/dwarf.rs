use std::fmt;

use rand::Rng;
use rand_derive::Rand;

use crate::character::ability::AbilityScoreIncreases;

use super::Race;

#[derive(Debug, PartialEq, Rand)]
enum DwarfSubrace {
    Hill,
    Mountain,
}

pub(crate) struct Dwarf {
    subrace: DwarfSubrace,
}

impl Race for Dwarf {
    fn new(rng: &mut impl Rng) -> Self {
        Self {
            subrace: rng.gen::<DwarfSubrace>(),
        }
    }

    fn increases(&self) -> AbilityScoreIncreases {
        match self.subrace {
            DwarfSubrace::Hill => AbilityScoreIncreases {
                charisma: 0,
                constitution: 2,
                dexterity: 0,
                intelligence: 0,
                strength: 0,
                wisdom: 1,
            },
            DwarfSubrace::Mountain => AbilityScoreIncreases {
                charisma: 0,
                constitution: 2,
                dexterity: 0,
                intelligence: 0,
                strength: 2,
                wisdom: 0,
            },
        }
    }
}

impl fmt::Display for Dwarf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.subrace {
            DwarfSubrace::Hill => write!(f, "Hill Dwarf"),
            DwarfSubrace::Mountain => write!(f, "Mountain Dwarf"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_dwarf_new() {
        let mut rng = Pcg64::from_entropy();
        let race = Dwarf::new(&mut rng);
        assert_eq!(race.subrace, DwarfSubrace::Hill);
    }
}
