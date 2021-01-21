use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[allow(dead_code)]
#[derive(Clone, Copy, Deserialize, Display, EnumIter, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum Language {
    Abyssal,
    Celestial,
    Common,
    #[strum(serialize = "Deep Speech")]
    DeepSpeech,
    Draconic,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Infernal,
    Orc,
    Primordial,
    Sylvan,
    Undercommon,
}

pub(crate) trait Languages {
    /// Return list of languages for this object
    fn languages(&self) -> Vec<Language> {
        vec![]
    }

    /// Does this provide additional language options?
    fn addl_languages(&self) -> usize {
        0
    }
}
