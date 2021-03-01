use serde::{Deserialize, Serialize};
use std::fmt;

use crate::citation::Citation;

/// A feature or trait a character has.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub(crate) struct Feature {
    /// Name of the feature or trait.
    pub(crate) title: &'static str,
    /// Citation for where more information about this feature is available.
    pub(crate) citation: Citation,
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.citation)
    }
}

/// Trait for objects that provide features to a character.
pub(crate) trait Features {
    // Return a list of features this thing provides
    fn features(&self) -> Vec<Feature> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::citation::Book;

    #[test]
    fn test_display() {
        let feature = Feature {
            title: "Title",
            citation: Citation(Book::Phb, 1),
        };
        insta::assert_snapshot!(format!("{}", feature));
    }
}
