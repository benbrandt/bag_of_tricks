use std::{
    collections::{HashMap, HashSet},
    fmt,
};
use strum_macros::Display;

/// Titles of the available D&D Books.
#[derive(Clone, Copy, Display, Eq, Hash, PartialEq)]
pub(crate) enum Book {
    PHB,
}

/// Book and page number for citations.
pub(crate) struct Citation {
    pub(crate) book: Book,
    pub(crate) page: u16,
}

impl fmt::Display for Citation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} p{}", self.book, self.page)
    }
}

pub(crate) struct Citations(pub(crate) Vec<Citation>);

impl fmt::Display for Citations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut citations = HashMap::new();
        for c in &self.0 {
            citations
                .entry(c.book)
                .or_insert_with(HashSet::new)
                .insert(c.page);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_display() {
        assert_eq!(format!("{}", Book::PHB), "PHB");
    }

    #[test]
    fn test_citation_display() {
        let citation = Citation {
            book: Book::PHB,
            page: 123,
        };
        assert_eq!(format!("{}", citation), "PHB p123");
    }

    #[test]
    fn test_citations_display() {
        let citations = Citations(vec![
            Citation {
                book: Book::PHB,
                page: 123,
            },
            Citation {
                book: Book::PHB,
                page: 125,
            },
        ]);
        assert_eq!(format!("{}", citations), "PHB p123,125");
    }
}
