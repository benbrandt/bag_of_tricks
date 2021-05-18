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
pub struct Barbarian;

impl Backstory for Barbarian {}

impl Citations for Barbarian {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 46)])
    }
}

impl Class for Barbarian {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Strength],
            vec![AbilityScoreType::Constitution],
        )
    }
}

impl Features for Barbarian {}

impl Languages for Barbarian {}

impl Pantheons for Barbarian {}

impl Proficiencies for Barbarian {}

impl StartingEquipment for Barbarian {}

impl fmt::Display for Barbarian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Barbarian")
    }
}
