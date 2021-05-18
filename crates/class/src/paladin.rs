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
pub struct Paladin;

impl Backstory for Paladin {}

impl Citations for Paladin {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 82)])
    }
}

impl Class for Paladin {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Strength],
            vec![AbilityScoreType::Charisma],
        )
    }
}

impl Features for Paladin {}

impl Languages for Paladin {}

impl Pantheons for Paladin {}

impl Proficiencies for Paladin {}

impl StartingEquipment for Paladin {}

impl fmt::Display for Paladin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Paladin")
    }
}
