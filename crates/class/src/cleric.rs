use std::fmt;

use backstory::Backstory;
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use features::Features;
use languages::Languages;
use rand::Rng;
use serde::{Deserialize, Serialize};
use stats::{
    ability::AbilityScoreType, equipment::StartingEquipment, proficiencies::Proficiencies,
};

use super::Class;

#[derive(Default, Deserialize, Serialize)]
pub struct Cleric;

impl Backstory for Cleric {}

impl Citations for Cleric {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 56)])
    }
}

impl Class for Cleric {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Wisdom],
            vec![AbilityScoreType::Strength, AbilityScoreType::Constitution],
        )
    }
}

impl Features for Cleric {}

impl Languages for Cleric {}

impl Pantheons for Cleric {}

impl Proficiencies for Cleric {}

impl StartingEquipment for Cleric {}

impl fmt::Display for Cleric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cleric")
    }
}
