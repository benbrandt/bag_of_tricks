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
pub struct Wizard;

impl Backstory for Wizard {}

impl Citations for Wizard {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 112)])
    }
}

impl Class for Wizard {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Intelligence],
            vec![
                AbilityScoreType::Charisma,
                AbilityScoreType::Constitution,
                AbilityScoreType::Dexterity,
            ],
        )
    }
}

impl Features for Wizard {}

impl Languages for Wizard {}

impl Pantheons for Wizard {}

impl Proficiencies for Wizard {}

impl StartingEquipment for Wizard {}

impl fmt::Display for Wizard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wizard")
    }
}
