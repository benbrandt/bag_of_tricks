use std::fmt;
use strum_macros::Display;

/// Titles of the available D&D Books.
#[derive(Display)]
pub(crate) enum Book {
    #[strum(serialize = "PHB")]
    PlayersHandbook,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_display() {
        assert_eq!(format!("{}", Book::PlayersHandbook), "PHB");
    }

    #[test]
    fn test_citation_display() {
        let citation = Citation {
            book: Book::PlayersHandbook,
            page: 123,
        };
        assert_eq!(format!("{}", citation), "PHB p123");
    }
}
