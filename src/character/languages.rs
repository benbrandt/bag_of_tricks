use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

/// Available languages for a character to learn.
#[allow(dead_code)]
#[derive(Clone, Copy, Deserialize, Display, EnumIter, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum Language {
    Abyssal,
    Auran,
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

/// Trait for describing what languages are provided by a given object.
pub(crate) trait Languages {
    /// Return list of languages for this object
    fn languages(&self) -> Vec<Language> {
        vec![]
    }

    /// Additional languages this character can learn.
    fn addl_languages(&self) -> usize {
        0
    }
}
