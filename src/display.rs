use std::fmt;

pub struct Verse {
    pub book: String,
    pub chapter: String,
    pub number: String,
    pub verse: String,
}

pub struct Books {
    pub version: String,
    pub books: Vec<String>,
}

pub struct Bible {
    pub name: String,
    pub description: String,
    pub language: String,
    pub id: String,
}

impl Books {
    pub fn new(version: String, books: Vec<String>) -> Books {
        Books { version, books }
    }
}

impl Verse {
    /// Creates a new [`Verse`].
    pub fn new(verse: String, book: String, chapter: String, number: String) -> Verse {
        Verse {
            book,
            chapter,
            number,
            verse,
        }
    }
}

impl Bible {
    pub fn new(name: String, description: String, language: String, id: String) -> Bible {
        Bible {
            name,
            description,
            language,
            id,
        }
    }
}

impl fmt::Display for Verse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // create a reference for the verse
        let reference = format!("{} {}:{}", self.book, self.chapter, self.number);

        // Determine the width of the console.
        let size = termsize::get().unwrap();
        let width = size.cols as usize;

        // determine offset for reference to pretty print
        let mut padding = 0;
        if width < 10 {
        } else if width < self.verse.len() {
            padding = width;
        } else {
            padding = self.verse.len();
        }

        // write to the formatter
        write!(
            f,
            "{}\n\n{:>padding$}",
            self.verse,
            reference,
            padding = padding
        )
    }
}

impl fmt::Display for Books {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // contruct string with bible name
        let mut output = String::new();
        output.push_str(&self.version);
        output.push('\n');
        output.push_str(&"=".repeat(self.version.len()));
        for book in &self.books {
            output.push('\n');
            output.push_str(book);
        }
        write!(f, "{}", output)
    }
}

impl fmt::Display for Bible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // construct string
        let mut output = String::new();
        output.push_str("Bible: ");
        output.push_str(&self.name);
        output.push('\n');
        output.push_str("Description: ");
        output.push_str(&self.description);
        output.push('\n');
        output.push_str("Language: ");
        output.push_str(&self.language);
        output.push('\n');
        output.push_str("ID: ");
        output.push_str(&self.id);
        output.push('\n');
        writeln!(f, "{}", output)
    }
}
