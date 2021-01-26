use serde::{Deserialize, Serialize};
use std::fmt;

use crate::citation::Citation;

#[derive(Deserialize, Serialize)]
pub(crate) struct Feature<'a> {
    pub(crate) title: &'a str,
    pub(crate) citation: Citation,
}

impl fmt::Display for Feature<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.citation)
    }
}

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
            citation: Citation(Book::PHB, 1),
        };
        insta::assert_snapshot!(format!("{}", feature));
    }
}
