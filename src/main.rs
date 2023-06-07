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
#[command(name="bible-rs", version=crate_version!(), about="daily bread", long_about = ABOUT, arg_required_else_help(true))]
struct BibleParser {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Get a list of Books in the Bible
    List,
    /// Get the daily random verse from the Bible
    Daily,
    /// get a new random verse from the Bible
    New,
    /// Get a random verse from a specific book of the Bible
    Book {
        /// Desired book of the Bible
        #[arg(required = true)]
        book: Option<String>,
    },
}

fn main() {
    let args = BibleParser::parse();

    match &args.command {
        Some(Commands::List) => {
            for book in BOOKS.iter() {
                println!("{}", book);
            }
        }
        Some(Commands::Daily) => println!("Daily"),
        Some(Commands::New) => println!("New"),
        Some(Commands::Book { book }) => {
            if let Some(book) = book {
                if BOOKS.contains(&book.as_str()) {
                    println!("Book: {}", book);
                } else {
                    println!("Book not found");
                }
            }
        }
        None => return,
    }
}
