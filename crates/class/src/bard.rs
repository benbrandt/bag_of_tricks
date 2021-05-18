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
pub struct Bard;

impl Backstory for Bard {}

impl Citations for Bard {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 51)])
    }
}

impl Class for Bard {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Charisma],
            vec![AbilityScoreType::Dexterity],
        )
    }
}

impl Features for Bard {}

impl Languages for Bard {}

impl Pantheons for Bard {}

impl Proficiencies for Bard {}

impl StartingEquipment for Bard {}

impl fmt::Display for Bard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bard")
    }
}
