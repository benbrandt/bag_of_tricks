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
pub struct Monk;

impl Backstory for Monk {}

impl Citations for Monk {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 76)])
    }
}

impl Class for Monk {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Dexterity],
            vec![AbilityScoreType::Wisdom],
        )
    }
}

impl Features for Monk {}

impl Languages for Monk {}

impl Pantheons for Monk {}

impl Proficiencies for Monk {}

impl StartingEquipment for Monk {}

impl fmt::Display for Monk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monk")
    }
}
