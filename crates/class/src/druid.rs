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
pub struct Druid;

impl Backstory for Druid {}

impl Citations for Druid {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 64)])
    }
}

impl Class for Druid {
    fn gen(_: &mut impl Rng) -> Self {
        Self
    }

    fn ability_rank() -> (Vec<AbilityScoreType>, Vec<AbilityScoreType>) {
        (
            vec![AbilityScoreType::Wisdom],
            vec![AbilityScoreType::Constitution],
        )
    }
}

impl Features for Druid {}

impl Languages for Druid {}

impl Pantheons for Druid {}

impl Proficiencies for Druid {}

impl StartingEquipment for Druid {}

impl fmt::Display for Druid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Druid")
    }
}
