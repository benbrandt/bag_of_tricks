use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Character;

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

impl Language {
    // Sometimes you end up with dupes. Consume and replace with a new option
    pub(crate) fn gen(rng: &mut impl Rng, character: &Character, amount: usize) -> Vec<Self> {
        Language::iter()
            .filter(|l| !character.languages.contains(l))
            .choose_multiple(rng, amount)
    }
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
