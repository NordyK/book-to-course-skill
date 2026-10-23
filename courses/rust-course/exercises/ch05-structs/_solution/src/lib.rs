/// An International Standard Book Number.
pub struct Isbn(pub String);

/// A book in the library.
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: Isbn,
    pub year: u16,
    pub available: bool,
}

/// A new, available book.
pub fn new_book(title: String, author: String, isbn: Isbn, year: u16) -> Book {
    Book { title, author, isbn, year, available: true }
}

/// Same title and author, a new ISBN and year.
pub fn second_edition(book: &Book, isbn: Isbn, year: u16) -> Book {
    Book {
        title: book.title.clone(),
        author: book.author.clone(),
        isbn,
        year,
        available: true,
    }
}

/// "Title by Author (year)", with " [on loan]" if it is not available.
pub fn describe(book: &Book) -> String {
    let mut s = format!("{} by {} ({})", book.title, book.author, book.year);
    if !book.available {
        s.push_str(" [on loan]");
    }
    s
}

#[cfg(test)]
mod tests;
