pub mod bible;

use bible::{get_daily_verse, get_new_verse_from_book, list_books};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

use clap::{crate_version, Parser, Subcommand};
use serde::Deserialize;
use std::env;

use crate::bible::get_new_verse;

#[derive(Debug, Deserialize)]
struct Config {
    api_key: Option<String>,
    bible_version: Option<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            api_key: Some("de4e12af7f28f599-02".to_string()),
            bible_version: Some("b9f970d519f43f80d3d1818a74cb674b".to_string()),
        }
    }
}

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
    /// get a new random verse from the Bible
    New,
    /// Get a random verse from a specific book of the Bible
    Book {
        /// The book of the Bible to get a random verse from
        #[arg(required = true)]
        book: String,
    },
}

#[tokio::main]
async fn main() {
    let mut config: Config = Figment::new()
        .merge(Toml::file("bible-rs.toml"))
        .merge(Env::prefixed("BIBLE_RS_"))
        .extract()
        .unwrap();

    let args = BibleParser::parse();
    match args.api_key {
        Some(api_key) => config.api_key = Some(api_key),
        None => match config.api_key {
            Some(api_key) => config.api_key = Some(api_key),
            None => config.api_key = Config::default().bible_version,
        },
    }
    match args.bible_version {
        Some(bible_version) => config.bible_version = Some(bible_version),
        None => match config.bible_version {
            Some(bible_version) => config.bible_version = Some(bible_version),
            None => config.bible_version = Config::default().bible_version,
        },
    }

    match &args.command {
        Some(Commands::List) => {
            match list_books(
                config.api_key.unwrap().as_str(),
                config.bible_version.unwrap().as_str(),
            )
            .await
            {
                Ok(_) => return,
                Err(e) => println!("Error: {}", e),
            }
        }
        Some(Commands::Daily) => {
            match get_daily_verse(
                config.api_key.unwrap().as_str(),
                config.bible_version.unwrap().as_str(),
            )
            .await
            {
                Ok(_) => return,
                Err(e) => println!("Error: {}", e),
            }
        }
        Some(Commands::New) => {
            println!("New");
            match get_new_verse(
                config.api_key.unwrap().as_str(),
                config.bible_version.unwrap().as_str(),
            )
            .await
            {
                Ok(_) => return,
                Err(e) => println!("Error: {}", e),
            }
        }
        Some(Commands::Book { book }) => {
            get_new_verse_from_book(
                config.api_key.unwrap().as_str(),
                config.bible_version.unwrap().as_str(),
                book.as_str(),
            )
            .await
        }
        None => return,
    }
}
