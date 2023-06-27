pub mod api;

use clap::{crate_version, Parser, Subcommand};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;

const ABOUT: &str = "Get a random verse from the Bible.";
const DEFAULT_VERSION: &str = "kjv";
const DEFAULT_KEY: &str = "b9f970d519f43f80d3d1818a74cb674b";
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
    List {
        /// The API key to Use
        #[arg(required = false, default_value = DEFAULT_KEY)]
        api_key: Option<String>,
        #[arg(required = false, default_value = DEFAULT_VERSION)]
        version: Option<String>,
    },
    /// Get the daily random verse from the Bible
    Daily {
        /// The API key to Use
        #[arg(required = false, default_value = DEFAULT_KEY)]
        api_key: Option<String>,
        #[arg(required = false, default_value = DEFAULT_VERSION)]
        version: Option<String>,
    },
    /// get a new random verse from the Bible
    New {
        /// The API key to Use
        #[arg(required = false, default_value = DEFAULT_KEY)]
        api_key: Option<String>,
        #[arg(required = false, default_value = DEFAULT_VERSION)]
        version: Option<String>,
    },
    /// Get a random verse from a specific book of the Bible
    Book {
        /// The API key to Use
        #[arg(required = false, default_value = DEFAULT_KEY)]
        api_key: Option<String>,
        /// The version of the Bible to use
        #[arg(required = false, default_value = DEFAULT_VERSION)]
        version: Option<String>,
        /// The book of the Bible to get a verse from
        #[arg(required = true)]
        book: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let args = BibleParser::parse();

    match &args.command {
        Some(Commands::List { api_key, version }) => {
            println!("stub list");
        }
        Some(Commands::Daily { api_key, version }) => {
            println!("daily stub");
            get_daily_verse().await;
        }
        Some(Commands::New { api_key, version }) => {
            println!("New");
        }
        Some(Commands::Book {
            api_key,
            version,
            book,
        }) => {
            //            if let Some(book) = book {
            //                if BOOKS.contains(&book.as_str()) {
            //                    println!("Book: {}", book);
            //                } else {
            //                    println!("Book not found, please try again.  Use `bible-rs list` to see the Books of the Bible and their spellings.");
            //                }
            //            }
            //
            println!("stub book");
        }
        None => return,
    }
}

async fn get_books() -> Result<(), reqwest::Error> {
    Ok(())
}

async fn get_daily_verse() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_key = "b9f970d519f43f80d3d1818a74cb674b";
    // Set up the request headers with the API key
    let mut headers = HeaderMap::new();

    //    let bible_version_id = "685d1470fe4d5c3b-01";
    //    let bible_book_id = "genesis";
    let url = format!("https://api.scripture.api.bible/v1/bibles",);

    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());

    let resp = client.get(url).headers(headers).send().await?;
    let resp_body = resp.text().await?;

    println!("Response body = {:?}", resp_body);

    println!("body = {:?}", resp_body);

    Ok(())
}
