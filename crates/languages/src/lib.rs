use deities::{Pantheon, PantheonWeight, Pantheons};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
pub enum LanguageType {
    Exotic,
    Standard,
}

impl LanguageType {
    fn weight(self) -> f64 {
        match self {
            Self::Exotic => 1.0,
            Self::Standard => 2.0,
        }
    }
}

/// Available languages for a character to learn.
#[allow(dead_code)]
#[derive(Clone, Copy, Deserialize, Display, EnumIter, Eq, Hash, PartialEq, Serialize)]
pub enum Language {
    Abyssal,
    Celestial,
    Common,
    #[strum(serialize = "Deep Speech")]
    DeepSpeech,
    Draconic,
    Dwarvish,
    Elvish,
    Giant,
    Gith,
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
    pub fn gen(
        rng: &mut impl Rng,
        current_languages: &[Language],
        (amount, language_type): (usize, Option<LanguageType>),
    ) -> Vec<Self> {
        language_type
            .map_or_else(
                || Self::iter().collect::<Vec<Self>>(),
                |lt| {
                    let filtered = Self::iter()
                        .filter(|l| l.language_type() == lt)
                        .collect::<Vec<Self>>();
                    if filtered.is_empty() {
                        Self::iter().collect::<Vec<Self>>()
                    } else {
                        filtered
                    }
                },
            )
            .iter()
            .filter(|l| !current_languages.contains(l))
            .copied()
            .collect::<Vec<_>>()
            .choose_multiple_weighted(rng, amount, |l| l.language_type().weight())
            .unwrap()
            .copied()
            .collect()
    }

    fn language_type(self) -> LanguageType {
        match self {
            Self::Common
            | Self::Dwarvish
            | Self::Elvish
            | Self::Giant
            | Self::Gnomish
            | Self::Goblin
            | Self::Halfling
            | Self::Orc => LanguageType::Standard,
            Self::Abyssal
            | Self::Celestial
            | Self::DeepSpeech
            | Self::Draconic
            | Self::Gith
            | Self::Infernal
            | Self::Primordial
            | Self::Sylvan
            | Self::Undercommon => LanguageType::Exotic,
        }
    }
}

impl Pantheons for Language {
    fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {
        (match self {
            Language::Abyssal
            | Language::Celestial
            | Language::DeepSpeech
            | Language::Gith
            | Language::Infernal
            | Language::Primordial
            | Language::Sylvan => vec![],
            Language::Common => vec![Pantheon::ForgottenRealms],
            Language::Draconic => vec![Pantheon::Dragon, Pantheon::Kobold, Pantheon::Lizardfolk],
            Language::Dwarvish => vec![Pantheon::Dwarven],
            Language::Elvish => vec![Pantheon::Elven],
            Language::Giant => vec![Pantheon::Giant],
            Language::Gnomish => vec![Pantheon::Gnomish],
            Language::Goblin => vec![Pantheon::Bugbear, Pantheon::Goblin],
            Language::Halfling => vec![Pantheon::Halfling],
            Language::Orc => vec![Pantheon::Orc],
            Language::Undercommon => vec![Pantheon::Drow, Pantheon::Duergar],
        })
        .into_iter()
        .map(|p| (p, PantheonWeight::Possible))
        .collect::<Vec<_>>()
    }
}

/// Trait for describing what languages are provided by a given object.
pub trait Languages {
    /// Return list of languages for this object
    fn languages(&self) -> Vec<Language> {
        vec![]
    }

    /// Additional languages this character can learn.
    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (0, None)
    }
}
