pub mod display;

use anyhow::Result;
use chrono::Local;
use display::{Books, Verse};
use rand::{rngs::StdRng, Rng};
use rand_core::SeedableRng;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT},
    Client,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    pub bible_version: Option<String>,
}

impl Config {
    pub fn new(api_key: Option<String>, bible_version: Option<String>) -> Config {
        Config {
            api_key,
            bible_version,
        }
    }

    pub fn api_key(&self) -> &str {
        self.api_key.as_ref().unwrap()
    }

    pub fn bible_version(&self) -> &str {
        self.bible_version.as_ref().unwrap()
    }
}

// define constants

const BASE_URL: &str = "https://api.scripture.api.bible/v1/bibles/";

// define public functions

/// fetch a daily random verse
pub async fn get_daily_verse(config: &Config) -> Result<Verse> {
    let mut rng = get_rng_from_date();

    let book = get_random_book(config, &mut rng).await?;
    let chapter = get_random_chapter(config, book.as_ref(), &mut rng).await?;
    let (verse_text, verse_id) = get_random_verse(config, chapter.as_ref(), &mut rng).await?;

    let verse = display::Verse::new(verse_text, verse_id);
    Ok(verse)
}

pub async fn get_new_verse(config: &Config) -> Result<Verse> {
    let mut rng = get_rng();

    let book = get_random_book(&config, &mut rng).await?;
    let chapter = get_random_chapter(&config, book.as_ref(), &mut rng).await?;
    let (verse, verse_id) = get_random_verse(config, chapter.as_ref(), &mut rng).await?;
    let verse = display::Verse::new(verse, verse_id);

    Ok(verse)
}

pub async fn get_new_verse_from_book(config: &Config, book: &str) -> Result<Verse> {
    // check book is in the list of books
    let book_names = get_books_by_name(config).await;
    let mut book_found = false;
    let mut book_id: usize = 0;
    for (i, b) in book_names.unwrap().iter().enumerate() {
        if b.to_lowercase() == book.to_lowercase() {
            book_found = true;
            book_id = i;
        }
    }
    if !book_found {
        println!("Book not found - please check the provided book name is correct. You can get a list of books by running `bible-rs list`");
    }
    let book_ids = get_books_by_id(config).await;
    let book_id = &book_ids.unwrap()[book_id];
    let seed: u64 = rand::thread_rng().gen();
    let mut rng = StdRng::seed_from_u64(seed);
    let chapter = get_random_chapter(config, &book_id, &mut rng).await;
    let (verse, verse_id) = get_random_verse(config, &chapter.as_ref().unwrap(), &mut rng)
        .await
        .unwrap();
    let verse = display::Verse::new(verse, verse_id);
    Ok(verse)
}

pub async fn list_books(config: &Config) -> Result<Books> {
    let name = get_bible_info(config).await?;
    let books = get_books_by_name(config).await?;
    let book_info = Books::new(name, books);
    Ok(book_info)
}

async fn get_books_by_id(config: &Config) -> Result<Vec<String>> {
    let client = Client::new();
    // Set up the request headers with the API api_key
    let mut headers = HeaderMap::new();
    let url = format!(
        "{BASE_URL}{version}/books",
        version = config.bible_version()
    );
    headers.insert("api-key", HeaderValue::from_str(config.api_key()).unwrap());

    let resp = client.get(url).headers(headers).send().await?;

    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");

    let json_book_data = json["data"].as_array().unwrap();
    let mut books: Vec<String> = Vec::new();
    for book in json_book_data {
        books.push(book["id"].as_str().unwrap().to_string());
    }
    Ok(books)
}

async fn get_bible_info(config: &Config) -> Result<String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!("{BASE_URL}{version}", version = config.bible_version());
    headers.insert("api-key", HeaderValue::from_str(config.api_key()).unwrap());
    let resp = client.get(url).headers(headers).send().await?;
    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
    let bible_name = json["data"]["name"].as_str().unwrap();
    Ok(bible_name.to_string())
}

async fn get_books_by_name(config: &Config) -> Result<Vec<String>> {
    let client = Client::new();
    // Set up the request headers with the API api_key
    let mut headers = HeaderMap::new();
    let url = format!(
        "{BASE_URL}{version}/books",
        version = config.bible_version()
    );
    headers.insert("api-key", HeaderValue::from_str(config.api_key()).unwrap());

    let resp = client.get(url).headers(headers).send().await?;

    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");

    let json_book_data = json["data"].as_array().unwrap();
    let mut books: Vec<String> = Vec::new();
    for book in json_book_data {
        books.push(book["name"].as_str().unwrap().to_string());
    }
    Ok(books)
}

async fn book_id_to_name(config: &Config, book_id: &str) -> Result<String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!(
        "{BASE_URL}{version}/books/{book_id}",
        version = config.bible_version(),
        book_id = book_id
    );
    headers.insert("api-key", HeaderValue::from_str(config.api_key()).unwrap());
    let resp = client.get(url).headers(headers).send().await?;
    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
    let book_name = json["data"]["name"].as_str().unwrap();
    Ok(book_name.to_string())
}

// private functions
fn get_rng_from_date() -> StdRng {
    // get the current date
    // hash the date string
    // truncate the hash to 8
    // convert the truncated hash to a u64
    let date = Local::now().naive_local().date();
    let date_hash = sha256::digest(date.to_string().as_bytes());
    let truncated_hash = &date_hash[0..16];
    let seed = hex_to_u64(truncated_hash.as_bytes()).unwrap();
    StdRng::seed_from_u64(seed)
}

fn get_rng() -> StdRng {
    let seed: u64 = rand::thread_rng().gen();
    StdRng::seed_from_u64(seed)
}

fn hex_to_u64(b: &[u8]) -> Option<u64> {
    let a = std::str::from_utf8(b).ok()?;
    u64::from_str_radix(a, 16).ok()
}

async fn get_bibles(api_key: &str) -> Result<()> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = BASE_URL;

    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());

    let resp = client.get(url).headers(headers).send().await?;

    let resp_body = resp.text().await?;

    println!("Response body = {:?}", resp_body);
    println!("body = {:?}", resp_body);

    Ok(())
}

async fn get_random_verse(
    config: &Config,
    chapter: &str,
    rng: &mut StdRng,
) -> Result<(String, String)> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!(
        "{BASE_URL}{version}/chapters/{chapter}/verses",
        version = config.bible_version()
    );
    headers.insert("api-key", HeaderValue::from_str(config.api_key()).unwrap());
    let resp = client.get(url).headers(headers).send().await?;
    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
    let verse_list = json["data"].as_array().unwrap();
    let verse_index = rng.gen_range(0..verse_list.len());
    let verse = verse_list.get(verse_index).unwrap();
    let verse_id = verse["id"].as_str().unwrap().to_string();
    let url = format!(
        "{BASE_URL}{version}/verses/{verse_id}",
        version = config.bible_version()
    );

    let mut headers = HeaderMap::new();
    headers.insert("api-key", HeaderValue::from_str(config.api_key()).unwrap());
    headers.insert(ACCEPT, HeaderValue::from_static("application/json")); // Adding the Accept header
    let resp = client
        .get(url)
        .query(&[
            ("content-type", "text"),
            ("include-notes", "false"),
            ("include-titles", "false"),
            ("include-chapter-numbers", "false"),
            ("include-verse-numbers", "false"),
            ("include-verse-spans", "false"),
            ("use-org-id", "false"),
        ])
        .headers(headers)
        .send()
        .await?;
    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
    let verse_text = json["data"]["content"].as_str().unwrap().trim();
    let verse = String::from(verse_text);
    Ok((verse, verse_id))
}

async fn get_random_book(config: &Config, rng: &mut StdRng) -> Result<String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!(
        "{BASE_URL}{version}/books",
        version = config.bible_version()
    );
    headers.insert("api-key", HeaderValue::from_str(config.api_key()).unwrap());
    let resp = client.get(url).headers(headers).send().await?;
    let resp_body = resp.text().await?;

    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
    let book_list = json["data"].as_array().unwrap();
    let book_index = rng.gen_range(0..book_list.len());

    let book = book_list.get(book_index).unwrap();
    let book = book["id"].as_str().unwrap().to_string();

    Ok(book)
}

async fn get_random_chapter(config: &Config, book: &str, rng: &mut StdRng) -> Result<String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!(
        "{BASE_URL}{version}/books/{book}/chapters",
        version = config.bible_version(),
        book = book
    );
    headers.insert("api-key", HeaderValue::from_str(config.api_key()).unwrap());
    let resp = client.get(url).headers(headers).send().await?;
    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
    let chapter_list = json["data"].as_array().unwrap_or_else();
    let mut chapter_index = rng.gen_range(0..chapter_list.len());
    let mut chapter = chapter_list.get(chapter_index).unwrap();
    if chapter["number"] == "intro" {
        chapter_index = chapter_index + 1;
        chapter = chapter_list.get(chapter_index).unwrap();
    }
    let chapter = chapter["id"].as_str().unwrap().to_string();
    Ok(chapter)
}
