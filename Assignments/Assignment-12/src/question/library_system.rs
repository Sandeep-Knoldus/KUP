/// Structure 'Books' having four fields
///
/// #Field
///
/// author: author is of type Vec<String>
/// book_no: book_no is of type Vec<i32>
/// book_title: book_title is of type Vec<String>
/// flag: flag is of integer i32 type
pub struct Books {
    pub author: Vec<String>,
    pub book_no: Vec<i32>,
    pub book_title: Vec<String>,
    pub flag: i32,
}

/// Implementing Struct 'Books'
impl Books {
    /// Function 'new_book' adds new book
    ///
    /// #Arguments
    ///
    /// &mut self: mutable reference of the instance of the Books structure
    /// auth: auth is of type String
    /// title: title is of type String
    /// book_number: book_number is of type i32
    ///
    /// #Return
    ///
    /// Returns boolean, true if book gets added and false if not added
    pub fn new_book(&mut self, auth: String, title: String, book_number: i32) -> bool {
        if self.book_no.contains(&book_number) {
            log::error!("Book already exists in the Library");
            false
        } else {
            self.author.push(auth);
            self.book_title.push(title);
            self.book_no.push(book_number);
            self.flag = 0;
            log::info!("Book added");
            true
        }
    }

    /// Function 'total_number' counts the number of books present
    ///
    /// #Arguments
    ///
    /// &self: mutable reference of the instance of the Books structure
    ///
    /// #Return
    ///
    /// Returns integer i32, the total count of books
    pub fn total_number(&self) -> i32 {
        let mut count = 0;
        if self.book_no.is_empty() {
            log::error!("Library is empty");
        } else {
            for _index in 0..self.book_no.len() {
                count += 1;
            }
            log::info!("Total Books: {}", count);
        }
        count
    }

    /// Function 'display_book_author' display books by author name
    ///
    /// #Arguments
    ///
    /// &self: mutable reference of the instance of the Books structure
    /// auth: auth is of String type
    ///
    /// #Return
    ///
    /// Returns boolean, true if author is present & false if not present
    pub fn display_book_author(&self, auth: String) -> bool {
        if self.author.contains(&auth) {
            for index in 0..self.book_no.len() {
                if auth == self.author[index] {
                    log::info!(
                        "Books by {:?}: {:?} with accession number: {:?}",
                        self.author[index],
                        self.book_title[index],
                        self.book_no[index]
                    );
                }
            }
            true
        } else {
            log::error!("Author not found in library");
            false
        }
    }

    /// Function 'display_book_title' display books by title
    ///
    /// #Arguments
    ///
    /// &self: mutable reference of the instance of the Books structure
    /// title: title is of String type
    ///
    /// #Return
    ///
    /// Returns boolean, true if title is present & false if not present
    pub fn display_book_title(&self, title: String) -> bool {
        if self.book_title.contains(&title) {
            for index in 0..self.book_no.len() {
                if title == self.book_title[index] {
                    log::info!(
                        "{:?} books are present in Library by authors: {:?} and accession number: {:?}",
                        self.book_title[index], self.author[index], self.book_no[index]
                    );
                }
            }
            true
        } else {
            log::error!("Book not found!!!");
            false
        }
    }

    /// Function 'issue_book' issues a book and removes the book from the Library
    ///
    /// #Arguments
    ///
    /// &mut self: mutable reference of the instance of the Books structure
    /// book_number: book_number is of integer i32 type
    ///
    /// #Return
    ///
    /// Returns boolean, true if book_number is present & false if not present
    pub fn issue_book(&mut self, book_number: i32) -> bool {
        if self.book_no.contains(&book_number) {
            log::info!("Book {:?}: Issued", book_number);
            self.book_no.remove((book_number - 1) as usize);
            self.book_title.remove((book_number - 1) as usize);
            self.author.remove((book_number - 1) as usize);
            self.flag = 1;
            true
        } else {
            log::error!("Book not found!!!");
            false
        }
    }

    /// Function 'library_display' displays the books present
    ///
    /// #Arguments
    ///
    /// &self: mutable reference of the instance of the Books structure
    ///
    /// #Return
    ///
    /// Returns boolean, true if books are displayed & false if not displayed
    pub fn library_display(&self) -> bool {
        if self.book_no.is_empty() {
            log::error!("Library is empty");
            false
        } else {
            for index in 0..self.book_no.len() {
                log::info!(
                    "Book {}: {}, by {} with flag: {}",
                    self.book_no[index],
                    self.book_title[index],
                    self.author[index],
                    self.flag
                );
            }
            true
        }
    }
}
