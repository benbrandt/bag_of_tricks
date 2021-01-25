use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt,
};
use strum_macros::Display;

/// Titles of the available D&D Books.
#[derive(Clone, Copy, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum Book {
    PHB,
}

/// Book and page number for citations.
#[derive(Deserialize, Serialize)]
pub(crate) struct Citation(pub(crate) Book, pub(crate) u16);

impl fmt::Display for Citation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} p{}", self.0, self.1)
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CitationList(pub(crate) Vec<Citation>);

impl fmt::Display for CitationList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut citations = HashMap::new();
        for c in &self.0 {
            citations
                .entry(c.0)
                .or_insert_with(HashSet::new)
                .insert(c.1);
        }
        for (book, page_nums) in citations {
            let mut pages = page_nums
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            pages.sort();
            write!(f, "{} p{}", book, pages.join(","))?;
        }
        write!(f, "")
    }
}

pub(crate) trait Citations {
    /// Return list of citations for the object in question
    fn citations(&self) -> CitationList;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_display() {
        assert_eq!(format!("{}", Book::PHB), "PHB");
    }

    #[test]
    fn test_citation_display() {
        let citation = Citation(Book::PHB, 123);
        assert_eq!(format!("{}", citation), "PHB p123");
    }

    #[test]
    fn test_citations_display() {
        let citations = CitationList(vec![Citation(Book::PHB, 123), Citation(Book::PHB, 125)]);
        assert_eq!(format!("{}", citations), "PHB p123,125");
    }
}
