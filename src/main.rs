use clap::{crate_version, Parser, Subcommand};

const ABOUT: &str = "Get a random verse from the Bible.";

const BOOKS: [&'static str; 73] = [
    "Genesis",
    "Exodus",
    "Leviticus",
    "Numbers",
    "Deuteronomy",
    "Joshua",
    "Judges",
    "Ruth",
    "1 Samuel",
    "2 Samuel",
    "1 Kings",
    "2 Kings",
    "1 Chronicles",
    "2 Chronicles",
    "Ezra",
    "Nehemiah",
    "Tobit",
    "Judith",
    "Esther",
    "1 Maccabees",
    "2 Maccabees",
    "Job",
    "Psalms",
    "Proverbs",
    "Ecclesiastes",
    "Song of Songs",
    "Wisdom",
    "Sirach",
    "Isaiah",
    "Jeremiah",
    "Lamentations",
    "Baruch",
    "Ezekiel",
    "Daniel",
    "Hosea",
    "Joel",
    "Amos",
    "Obadiah",
    "Jonah",
    "Micah",
    "Nahum",
    "Habakkuk",
    "Zephaniah",
    "Haggai",
    "Zechariah",
    "Malachi",
    "Matthew",
    "Mark",
    "Luke",
    "John",
    "Acts",
    "Romans",
    "1 Corinthians",
    "2 Corinthians",
    "Galatians",
    "Ephesians",
    "Philippians",
    "Colossians",
    "1 Thessalonians",
    "2 Thessalonians",
    "1 Timothy",
    "2 Timothy",
    "Titus",
    "Philemon",
    "Hebrews",
    "James",
    "1 Peter",
    "2 Peter",
    "1 John",
    "2 John",
    "3 John",
    "Jude",
    "Revelation",
];

/// bible-rs is a command line tool for getting a random verse from the Bible.
#[derive(Debug, Parser)]
#[command(name="bible-rs", version=crate_version!(), about="daily bread", long_about = ABOUT)]
struct BibleParser {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Desired book of the Bible
    #[arg(short = 'b', long = "book")]
    book: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Get list of verses
    List,
}

fn main() {
    let args = BibleParser::parse();

    if let Some(book) = args.book.as_deref() {
        if BOOKS.contains(&book) {
            println!("Book: {}", book);
        } else {
            println!("Book not found");
        }
    }

    match &args.command {
        Some(Commands::List) => {
            for book in BOOKS.iter() {
                println!("{}", book);
            }
        }
        None => println!("No command given"),
    }
}
