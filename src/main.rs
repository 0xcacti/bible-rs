use bible_rs::{
    get_bibles, get_daily_verse, get_new_verse, get_new_verse_from_book, list_books, Config,
};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

use clap::{crate_version, Parser, Subcommand};
use std::{env, process};

/// bible-rs is a command line tool for getting a random verse from the Bible.
#[derive(Debug, Parser)]
#[command(name="bible-rs", version=crate_version!(), about="daily bread", long_about = "Get a random verse from the Bible.", arg_required_else_help(true))]
struct BibleParser {
    /// The subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,
    /// The version of the Bible to use
    #[arg(short, long, required = false, global = true)]
    bible_version: Option<String>,
    /// The API key to use
    #[arg(short, long, required = false, global = true)]
    api_key: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Get a list of Books in the provided Bible version
    List,
    /// Get the daily random verse from the Bible
    Daily,
    /// Get a new random verse from the Bible
    New,
    /// Get a random verse from a specific book of the Bible
    Book {
        /// The book of the Bible to get a random verse from
        #[arg(required = true)]
        book: String,
    },
    /// Get available Bible versions
    Bibles,
}

#[tokio::main]
async fn main() {
    let mut config: Config = Figment::new()
        .merge(Toml::file("bible-rs.toml"))
        .merge(Env::prefixed("BIBLE_RS_"))
        .extract()
        .unwrap();

    let args = BibleParser::parse();

    // Check for API key
    match args.api_key {
        Some(api_key) => config.api_key = Some(api_key),
        None => match config.api_key {
            Some(api_key) => config.api_key = Some(api_key),
            None => {
                eprintln!("No API key provided. Please provide an API key using the --api-key flag, setting api_key in the bible-rs.toml file, or by setting the BIBLE_RS_API_KEY environment variable.");
                process::exit(1);
            }
        },
    }
    // Check for Bible version
    match args.bible_version {
        Some(bible_version) => config.bible_version = Some(bible_version),
        None => match config.bible_version {
            Some(bible_version) => config.bible_version = Some(bible_version),
            None => {
                eprintln!("No Bible version provided. Please provide a Bible version using the --bible-version flag, setting bible_version in the bible-rs.toml file, or by setting the BIBLE_RS_BIBLE_VERSION environment variable.");
                process::exit(1);
            }
        },
    }

    // handle commands
    match &args.command {
        Some(Commands::List) => match list_books(&config).await {
            Ok(books) => println!("{}", books),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        Some(Commands::Daily) => match get_daily_verse(&config).await {
            Ok(verse) => println!("{}", verse),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        Some(Commands::New) => match get_new_verse(&config).await {
            Ok(verse) => println!("{}", verse),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        Some(Commands::Book { book }) => {
            match get_new_verse_from_book(&config, book.as_str()).await {
                Ok(verse) => println!("{}", verse),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        Some(Commands::Bibles) => match get_bibles(&config).await {
            Ok(bibles) => {
                for bible in bibles {
                    println!("{}", bible);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        None => return,
    }
}
