use serde::{Deserialize, Serialize};
use std::fmt;

use crate::citation::Citation;

#[derive(Deserialize, Serialize)]
pub(crate) struct Feature<'a> {
    pub(crate) title: &'a str,
    pub(crate) description: &'a str,
    pub(crate) citation: Citation,
}

impl fmt::Display for Feature<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.title)?;
        writeln!(f, "{} ({})", self.description, self.citation)
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
            description: "Description",
            citation: Citation {
                book: Book::PHB,
                page: 1,
            },
        };
        insta::assert_snapshot!(format!("{}", feature));
    }
}
