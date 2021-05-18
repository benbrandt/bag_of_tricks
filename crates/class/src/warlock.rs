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
pub struct Warlock;

impl Backstory for Warlock {}

impl Citations for Warlock {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 105)])
    }
}

impl Class for Warlock {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Charisma],
            vec![AbilityScoreType::Constitution],
        )
    }
}

impl Features for Warlock {}

impl Languages for Warlock {}

impl Pantheons for Warlock {}

impl Proficiencies for Warlock {}

impl StartingEquipment for Warlock {}

impl fmt::Display for Warlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Warlock")
    }
}
