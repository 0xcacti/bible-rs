pub fn print_verse(verse: &str, book: &str, chapter: &str, verse_number: &str) {
    let reference = format!("{} {}:{}", book, chapter, verse_number);
    println!("{}\n", reference);
    let verse = "For God so loved the world, that he gave his only Son, that whoever believes in him should not perish but have eternal life.";
    let reference = "John 3:16";

    // Determine the width of the console. You may want to set this to a fixed value if you prefer.
    let size = termsize::get().unwrap();
    // Get the width of the terminal
    let width = size.cols as usize;

    if width < 10 {
        return;
    }
    if width < verse.len() {
        println!("{}\n", verse);
        let padding = width - reference.len();
        println!("{:padding$}{}", "", reference, padding = padding);
    } else {
        println!("{}\n", verse);
        let padding = verse.len() - reference.len();
        println!("{:padding$}{}", "", reference, padding = padding);
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
