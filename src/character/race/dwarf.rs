use rand::prelude::IteratorRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::ability::{AbilityScore, AbilityScoreType, AbilityScores},
    citation::{Book, Citation, Citations},
};

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum DwarfSubrace {
    Hill,
    Mountain,
}
#[derive(Deserialize, Serialize)]
pub(crate) struct Dwarf {
    subrace: DwarfSubrace,
}

#[typetag::serde]
impl Race for Dwarf {
    fn new(rng: &mut impl Rng) -> Self {
        Self {
            subrace: DwarfSubrace::iter()
                .choose(rng)
                .unwrap_or(DwarfSubrace::Hill),
        }
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Constitution, 2),
            match self.subrace {
                DwarfSubrace::Hill => AbilityScore(AbilityScoreType::Wisdom, 1),
                DwarfSubrace::Mountain => AbilityScore(AbilityScoreType::Strength, 2),
            },
        ])
    }

    fn citations(&self) -> Citations {
        let race = Citation {
            book: Book::PHB,
            page: 18,
        };
        let subrace = match self.subrace {
            DwarfSubrace::Hill | DwarfSubrace::Mountain => Citation {
                book: Book::PHB,
                page: 20,
            },
        };
        Citations(vec![race, subrace])
    }
}

impl fmt::Display for Dwarf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Dwarf", self.subrace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_snapshot() {
        let mut rng = Pcg64::seed_from_u64(1);
        let dwarf = Dwarf::new(&mut rng);
        insta::assert_yaml_snapshot!(dwarf);
        let dwarf = Dwarf::new(&mut rng);
        insta::assert_yaml_snapshot!(dwarf);
    }

    #[test]
    fn test_snapshot_display() {
        let dwarf = Dwarf {
            subrace: DwarfSubrace::Hill,
        };
        insta::assert_snapshot!(format!("{}", dwarf));
        let dwarf = Dwarf {
            subrace: DwarfSubrace::Mountain,
        };
        insta::assert_snapshot!(format!("{}", dwarf));
    }

    #[test]
    fn test_snapshot_abilities() {
        let dwarf = Dwarf {
            subrace: DwarfSubrace::Hill,
        };
        insta::assert_yaml_snapshot!(dwarf.abilities());
        let dwarf = Dwarf {
            subrace: DwarfSubrace::Mountain,
        };
        insta::assert_yaml_snapshot!(dwarf.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let dwarf = Dwarf {
            subrace: DwarfSubrace::Hill,
        };
        insta::assert_yaml_snapshot!(dwarf.citations());
        let dwarf = Dwarf {
            subrace: DwarfSubrace::Mountain,
        };
        insta::assert_yaml_snapshot!(dwarf.citations());
    }
}
