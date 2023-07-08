use std::fmt;

pub struct Verse {
    pub book: String,
    pub chapter: String,
    pub number: String,
    pub verse: String,
}

impl Verse {
    pub fn new(verse: String, verse_id: String) -> Verse {
        let verse_identifiers = verse_id.split(".").collect::<Vec<&str>>();
        Verse {
            book: verse_identifiers[0].to_string(),
            chapter: verse_identifiers[1].to_string(),
            number: verse_identifiers[2].to_string(),
            verse,
        }
    }
}

impl fmt::Display for Verse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reference = format!("{} {}:{}", self.book, self.chapter, self.number);
        // Determine the width of the console. You may want to set this to a fixed value if you prefer.
        let size = termsize::get().unwrap();
        // Get the width of the terminal
        let width = size.cols as usize;

        let mut padding = 0;
        if width < 10 {
        } else if width < self.verse.len() {
            padding = width - reference.len();
        } else {
            padding = self.verse.len();
        }
        write!(
            f,
            "{}\n\n{:>padding$}",
            self.verse,
            reference,
            padding = padding
        )
    }
}

pub fn print_book_list(books: Vec<String>, version: &str) {
    println!("{}", version);
    let divider = "=".repeat(version.len());
    println!("{}", divider);
    for book in books {
        println!("{}", book);
    }
}
