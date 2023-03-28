struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

struct Library<'a> {
    books: Vec<Book<'a>>,
}

impl<'a> Library<'a> {
    fn new() -> Library<'a> {
        Library { books: Vec::new() }
    }

     // ここにメソッドの実装を記述してください。
     

    fn find_books_by_author(&self, author_name: &str) -> Vec<&'a str> {
        self.books
            .iter()
            .filter(|book| book.author == author_name)
            .map(|book| book.title)
            .collect()
    }
}

fn main() {
    let mut library = Library::new();
    library.add_book("Rust Book 1", "Author A");
    library.add_book("Rust Book 2", "Author B");
    library.add_book("Rust Book 3", "Author A");

    let author_a_books = library.find_books_by_author("Author A");
    println!("Author A's books: {:?}", author_a_books);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_books_by_author() {
        let mut library = Library::new();
        library.add_book("Rust Book 1", "Author A");
        library.add_book("Rust Book 2", "Author B");
        library.add_book("Rust Book 3", "Author A");

        let author_a_books = library.find_books_by_author("Author A");
        let expected_books = vec!["Rust Book 1", "Rust Book 3"];
        assert_eq!(author_a_books, expected_books);

        let author_b_books = library.find_books_by_author("Author B");
        let expected_books = vec!["Rust Book 2"];
        assert_eq!(author_b_books, expected_books);

        let author_c_books = library.find_books_by_author("Author C");
        let expected_books: Vec<&str> = vec![];
        assert_eq!(author_c_books, expected_books);
    }
}