use clap::Parser;

const ABOUT: &str = "Get a random verse from the Bible.";

/// bible-rs is a command line tool for getting a random verse from the Bible.
#[derive(Parser, Debug)]
#[command(name="bible-rs", version, about, long_about = ABOUT)]
struct BibleParser {
    /// Desired book of the Bible
    #[arg(short, long)]
    book: String,
}

fn main() {
    let args = BibleParser::parse();
}
