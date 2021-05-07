use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use serde::{Deserialize, Serialize};
use strum::Display;

/// Types of alignment influence from personality traits
#[derive(Clone, Copy, Deserialize, Display, Serialize)]
pub enum Influence {
    Any,
    Chaotic,
    Evil,
    Good,
    Lawful,
    Neutral,
}

impl AlignmentInfluences for Influence {
    /// Return attitude influence from personality.
    /// Doubled because personality should be a major indicator of alignment.
    fn attitude(&self) -> Vec<Attitude> {
        match self {
            Self::Chaotic => vec![Attitude::Chaotic; 2],
            Self::Lawful => vec![Attitude::Lawful; 2],
            Self::Neutral => vec![Attitude::Neutral; 2],
            _ => vec![],
        }
    }

    /// Return morality influence from personality.
    /// Doubled because personality should be a major indicator of alignment.
    fn morality(&self) -> Vec<Morality> {
        match self {
            Self::Good => vec![Morality::Good; 2],
            Self::Evil => vec![Morality::Evil; 2],
            Self::Neutral => vec![Morality::Neutral; 2],
            _ => vec![],
        }
    }
}

/// Description of a character's personality
#[derive(Deserialize, Serialize)]
pub struct Personality {
    pub bond: String,
    pub flaw: String,
    pub ideal: (String, Influence),
    pub traits: Vec<String>,
}

/// Return attitude/morality influences from character's ideal
impl AlignmentInfluences for Personality {
    fn attitude(&self) -> Vec<Attitude> {
        self.ideal.1.attitude()
    }

    fn morality(&self) -> Vec<Morality> {
        self.ideal.1.morality()
    }
}

impl fmt::Display for Personality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PERSONALITY TRAITS:")?;
        for t in &self.traits {
            writeln!(f, "{}", t)?;
        }
        writeln!(f, "IDEAL: {} ({})", self.ideal.0, self.ideal.1)?;
        writeln!(f, "BOND: {}", self.bond)?;
        writeln!(f, "FLAW: {}", self.flaw)
    }
}

/// Trait to store associated constants for each background's personality tables.
pub trait PersonalityOptions {
    /// List of bonds to choose from
    fn bonds(&self) -> Vec<String> {
        vec![]
    }

    /// List of flaws to choose from
    fn flaws(&self) -> Vec<String> {
        vec![]
    }

    /// List of ideals to choose from, and their corresponding alignment influence
    fn ideals(&self) -> Vec<(String, Influence)> {
        vec![]
    }

    /// List of traits to choose from
    fn traits(&self) -> Vec<String> {
        vec![]
    }
}
