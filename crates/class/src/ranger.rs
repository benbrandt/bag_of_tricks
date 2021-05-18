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
pub struct Ranger;

impl Backstory for Ranger {}

impl Citations for Ranger {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 89)])
    }
}

impl Class for Ranger {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Dexterity, AbilityScoreType::Strength],
            vec![AbilityScoreType::Wisdom],
        )
    }
}

impl Features for Ranger {}

impl Languages for Ranger {}

impl Pantheons for Ranger {}

impl Proficiencies for Ranger {}

impl StartingEquipment for Ranger {}

impl fmt::Display for Ranger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ranger")
    }
}
