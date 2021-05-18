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
pub struct Sorcerer;

impl Backstory for Sorcerer {}

impl Citations for Sorcerer {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 99)])
    }
}

impl Class for Sorcerer {
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

impl Features for Sorcerer {}

impl Languages for Sorcerer {}

impl Pantheons for Sorcerer {}

impl Proficiencies for Sorcerer {}

impl StartingEquipment for Sorcerer {}

impl fmt::Display for Sorcerer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sorcerer")
    }
}
