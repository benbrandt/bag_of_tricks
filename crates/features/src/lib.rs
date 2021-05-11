use std::fmt;

use citation::Citation;
use serde::{Deserialize, Serialize};

/// A feature or trait a character has.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Feature {
    /// Name of the feature or trait.
    pub title: &'static str,
    /// Citation for where more information about this feature is available.
    pub citation: Citation,
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.citation)
    }
}

/// Trait for objects that provide features to a character.
pub trait Features {
    // Return a list of features this thing provides
    fn features(&self) -> Vec<Feature> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use citation::Book;

    #[test]
    fn test_display() {
        let feature = Feature {
            title: "Title",
            citation: Citation(Book::Phb, 1),
        };
        insta::assert_snapshot!(format!("{}", feature));
    }
}
