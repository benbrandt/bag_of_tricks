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
pub struct Fighter;

impl Backstory for Fighter {}

impl Citations for Fighter {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 70)])
    }
}

impl Class for Fighter {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Strength, AbilityScoreType::Dexterity],
            vec![
                AbilityScoreType::Constitution,
                AbilityScoreType::Intelligence,
            ],
        )
    }
}

impl Features for Fighter {}

impl Languages for Fighter {}

impl Pantheons for Fighter {}

impl Proficiencies for Fighter {}

impl StartingEquipment for Fighter {}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fighter")
    }
}
