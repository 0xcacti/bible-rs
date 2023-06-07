use clap::{crate_version, Parser};

const ABOUT: &str = "Get a random verse from the Bible.";

/// bible-rs is a command line tool for getting a random verse from the Bible.
#[derive(Debug, Parser)]
#[command(name="bible-rs", version=crate_version!(), about="daily bread", long_about = ABOUT)]
struct BibleParser {
    /// Desired book of the Bible
    #[arg(short = 'b', long = "book")]
    book: String,
}

fn main() {
    let args = BibleParser::parse();
}
