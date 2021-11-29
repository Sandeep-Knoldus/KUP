#[cfg(test)]
pub use crate::question::library_system::Books;
#[test]
pub fn add_new_book_success() {
    env_logger::init();
    let mut book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };
    book_list.new_book("Tutorials Point".to_string(), "Python".to_string(), 2);
    book_list.new_book("Tutorials Point".to_string(), "Rust".to_string(), 3);
    book_list.new_book("Javatpoint".to_string(), "Java".to_string(), 4);
    book_list.new_book("Javatpoint".to_string(), "Rust Basics".to_string(), 5);

    assert_eq!(
        book_list.new_book("Tutorials Point".to_string(), "Java".to_string(), 1),
        true
    );
}
#[test]
pub fn add_new_book_failure() {
    let mut book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };
    book_list.new_book("Tutorials Point".to_string(), "Python".to_string(), 1);
    book_list.new_book("Tutorials Point".to_string(), "Rust".to_string(), 3);
    book_list.new_book("Javatpoint".to_string(), "Java".to_string(), 4);
    book_list.new_book("Javatpoint".to_string(), "Rust Basics".to_string(), 5);

    assert_eq!(
        book_list.new_book("Tutorials Point".to_string(), "Java".to_string(), 1),
        false
    );
}

#[test]
pub fn book_display_success() {
    let mut book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };
    book_list.new_book("Tutorials Point".to_string(), "Java".to_string(), 1);
    book_list.new_book("Tutorials Point".to_string(), "Python".to_string(), 2);
    book_list.new_book("Tutorials Point".to_string(), "Rust".to_string(), 3);
    book_list.new_book("Javatpoint".to_string(), "Java".to_string(), 4);
    book_list.new_book("Javatpoint".to_string(), "Rust Basics".to_string(), 5);

    assert_eq!(book_list.library_display(), true);
}
#[test]
pub fn book_display_failure() {
    let book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };

    assert_eq!(book_list.library_display(), false);
}

#[test]
pub fn books_count_success() {
    let mut book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };
    book_list.new_book("Tutorials Point".to_string(), "Java".to_string(), 1);
    book_list.new_book("Tutorials Point".to_string(), "Python".to_string(), 2);
    book_list.new_book("Tutorials Point".to_string(), "Rust".to_string(), 3);
    book_list.new_book("Javatpoint".to_string(), "Java".to_string(), 4);
    book_list.new_book("Javatpoint".to_string(), "Rust Basics".to_string(), 5);

    assert_eq!(book_list.total_number(), 5);
}
#[test]
pub fn books_count_failure() {
    let book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };

    assert_eq!(book_list.total_number(), 0);
}

#[test]
pub fn display_book_author_success() {
    let mut book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };
    book_list.new_book("Tutorials Point".to_string(), "Java".to_string(), 1);
    book_list.new_book("Tutorials Point".to_string(), "Python".to_string(), 2);
    book_list.new_book("Tutorials Point".to_string(), "Rust".to_string(), 3);
    book_list.new_book("Javatpoint".to_string(), "Java".to_string(), 4);
    book_list.new_book("Javatpoint".to_string(), "Rust Basics".to_string(), 5);

    assert_eq!(
        book_list.display_book_author("Javatpoint".to_string()),
        true
    );
}
#[test]
pub fn display_book_author_failure() {
    let mut book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };
    book_list.new_book("Tutorials Point".to_string(), "Java".to_string(), 1);
    book_list.new_book("Tutorials Point".to_string(), "Python".to_string(), 2);
    book_list.new_book("Tutorials Point".to_string(), "Rust".to_string(), 3);
    book_list.new_book("Javatpoint".to_string(), "Java".to_string(), 4);
    book_list.new_book("Javatpoint".to_string(), "Rust Basics".to_string(), 5);

    assert_eq!(book_list.display_book_author("Sandeep".to_string()), false);
}

#[test]
pub fn display_book_title_success() {
    let mut book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };
    book_list.new_book("Tutorials Point".to_string(), "Java".to_string(), 1);
    book_list.new_book("Tutorials Point".to_string(), "Python".to_string(), 2);
    book_list.new_book("Tutorials Point".to_string(), "Rust".to_string(), 3);
    book_list.new_book("Javatpoint".to_string(), "Java".to_string(), 4);
    book_list.new_book("Javatpoint".to_string(), "Rust Basics".to_string(), 5);

    assert_eq!(book_list.display_book_title("Java".to_string()), true);
}
#[test]
pub fn display_book_title_failure() {
    let mut book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };
    book_list.new_book("Tutorials Point".to_string(), "Java".to_string(), 1);
    book_list.new_book("Tutorials Point".to_string(), "Python".to_string(), 2);
    book_list.new_book("Tutorials Point".to_string(), "Rust".to_string(), 3);
    book_list.new_book("Javatpoint".to_string(), "Java".to_string(), 4);
    book_list.new_book("Javatpoint".to_string(), "Rust Basics".to_string(), 5);

    assert_eq!(book_list.display_book_title("C++".to_string()), false);
}

#[test]
pub fn issue_book_success() {
    let mut book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };
    book_list.new_book("Tutorials Point".to_string(), "Java".to_string(), 1);
    book_list.new_book("Tutorials Point".to_string(), "Python".to_string(), 2);
    book_list.new_book("Tutorials Point".to_string(), "Rust".to_string(), 3);
    book_list.new_book("Javatpoint".to_string(), "Java".to_string(), 4);
    book_list.new_book("Javatpoint".to_string(), "Rust Basics".to_string(), 5);

    let issue = book_list.issue_book(3);
    assert_eq!(issue, true);
    if issue {
        log::info!("Book Issued")
    } else {
        log::info!("No book issued")
    }
    book_list.library_display();
}
#[test]
pub fn issue_book_failure() {
    let mut book_list = Books {
        author: vec![],
        book_no: vec![],
        book_title: vec![],
        flag: 0,
    };
    book_list.new_book("Tutorials Point".to_string(), "Java".to_string(), 1);
    book_list.new_book("Tutorials Point".to_string(), "Python".to_string(), 2);
    book_list.new_book("Tutorials Point".to_string(), "Rust".to_string(), 3);
    book_list.new_book("Javatpoint".to_string(), "Java".to_string(), 4);
    book_list.new_book("Javatpoint".to_string(), "Rust Basics".to_string(), 5);

    assert_eq!(book_list.issue_book(7), false);
}
