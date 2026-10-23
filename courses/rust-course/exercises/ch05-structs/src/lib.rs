/// An International Standard Book Number.
pub struct Isbn(pub String);

/// A book in the library. Add the fields: title, author, isbn, year, available.
pub struct Book {
}

/// A new, available book.
pub fn new_book(title: String, author: String, isbn: Isbn, year: u16) -> Book {
    todo!()
}

/// Same title and author, a new ISBN and year.
pub fn second_edition(book: &Book, isbn: Isbn, year: u16) -> Book {
    todo!()
}

/// "Title by Author (year)", with " [on loan]" if it is not available.
pub fn describe(book: &Book) -> String {
    todo!()
}

#[cfg(test)]
mod tests;
