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
pub struct Rogue;

impl Backstory for Rogue {}

impl Citations for Rogue {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 94)])
    }
}

impl Class for Rogue {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Dexterity],
            vec![AbilityScoreType::Charisma, AbilityScoreType::Intelligence],
        )
    }
}

impl Features for Rogue {}

impl Languages for Rogue {}

impl Pantheons for Rogue {}

impl Proficiencies for Rogue {}

impl StartingEquipment for Rogue {}

impl fmt::Display for Rogue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rogue")
    }
}
