use clap::Parser;

#[derive(Parser)]
struct Cli {
    // book of the Bible
    book: String,
    // date of the verse
    date: String,
}
fn main() {
    let args = clap::parse();
}
