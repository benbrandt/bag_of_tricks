#![deny(clippy::all)]
#![warn(clippy::pedantic)]
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use serde::{Deserialize, Serialize};
use strum::Display;

/// Titles of the available D&D Books.
#[derive(Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub enum Book {
    #[strum(serialize = "PHB")]
    Phb,
    #[strum(serialize = "SCAG")]
    Scag,
    #[strum(serialize = "VGTM")]
    Vgtm,
}

/// Book and page number for citations.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Citation(pub Book, pub u16);

impl fmt::Display for Citation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} p{}", self.0, self.1)
    }
}

/// List of multiple citations.
#[derive(Deserialize, Serialize)]
pub struct CitationList(pub Vec<Citation>);

/// Displays multiple citations from the same book together.
impl fmt::Display for CitationList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut citations = HashMap::new();
        for c in &self.0 {
            citations
                .entry(c.0)
                .or_insert_with(HashSet::new)
                .insert(c.1);
        }
        let mut books = citations
            .iter()
            .map(|(book, page_nums)| {
                let mut pages = page_nums
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                pages.sort();
                format!("{} p{}", book, pages.join(","))
            })
            .collect::<Vec<_>>();
        books.sort();
        write!(f, "{}", books.join(" "))
    }
}

/// Trait for any entity/object in need of citation.
///
/// Makes it easer for users to find more information in the source books.
pub trait Citations {
    /// Return list of citations for the object in question
    fn citations(&self) -> CitationList;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_display() {
        assert_eq!(format!("{}", Book::Phb), "PHB");
    }

    #[test]
    fn test_citation_display() {
        let citation = Citation(Book::Phb, 123);
        assert_eq!(format!("{}", citation), "PHB p123");
    }

    #[test]
    fn test_citations_display() {
        let citations = CitationList(vec![
            Citation(Book::Phb, 123),
            Citation(Book::Phb, 125),
            Citation(Book::Vgtm, 125),
        ]);
        assert_eq!(format!("{}", citations), "PHB p123,125 VGTM p125");
    }
}
