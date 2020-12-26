use std::fmt;
use strum_macros::Display;

/// Titles of the available D&D Books.
#[derive(Display)]
enum Book {
    #[strum(serialize = "Player's Handbook")]
    PlayersHandbook,
}

impl Book {
    /// Abbreviation of the book title for compact display
    fn abbreviation(&self) -> &str {
        match self {
            Book::PlayersHandbook => "PHB",
        }
    }
}

/// Book and page number for citations.
struct Citation {
    book: Book,
    page: u16,
}

impl fmt::Display for Citation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} p{}", self.book.abbreviation(), self.page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_display() {
        assert_eq!(format!("{}", Book::PlayersHandbook), "Player's Handbook");
        assert_eq!(format!("{}", Book::PlayersHandbook.abbreviation()), "PHB");
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
