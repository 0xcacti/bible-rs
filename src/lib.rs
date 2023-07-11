pub mod display;
pub mod utils;

use anyhow::Result;
use display::{Books, Verse};
use rand::{rngs::StdRng, Rng};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT},
    Client,
};
use serde::Deserialize;
use utils::{get_client_and_headers, get_rng, get_rng_from_date};

use crate::display::Bible;

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

const BASE_URL: &str = "https://api.scripture.api.bible/v1/bibles/";

/// fetch a daily random verse
pub async fn get_daily_verse(config: &Config) -> Result<Verse> {
    let mut rng = get_rng_from_date();
    let book = get_random_book(config, &mut rng).await?;
    let chapter = get_random_chapter(config, book.as_ref(), &mut rng).await?;
    let (verse, verse_id) = get_random_verse(config, chapter.as_ref(), &mut rng).await?;
    let verse_identifiers = verse_id.split(".").collect::<Vec<&str>>();
    let book_name = book_id_to_name(config, verse_identifiers[0]).await?;
    let verse = Verse::new(
        verse,
        book_name,
        verse_identifiers[1].to_string(),
        verse_identifiers[2].to_string(),
    );
    Ok(verse)
}

/// fetch a new random verse
pub async fn get_new_verse(config: &Config) -> Result<Verse> {
    let mut rng = get_rng();
    let book = get_random_book(&config, &mut rng).await?;
    let chapter = get_random_chapter(&config, book.as_ref(), &mut rng).await?;
    let (verse, verse_id) = get_random_verse(config, chapter.as_ref(), &mut rng).await?;
    let verse_identifiers = verse_id.split(".").collect::<Vec<&str>>();
    let book_name = book_id_to_name(config, verse_identifiers[0]).await?;
    let verse = Verse::new(
        verse,
        book_name,
        verse_identifiers[1].to_string(),
        verse_identifiers[2].to_string(),
    );
    Ok(verse)
}

/// fetch a new random verse from a specific book of the Bible
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
    let mut rng = get_rng();
    let chapter = get_random_chapter(config, &book_id, &mut rng).await;
    let (verse, verse_id) = get_random_verse(config, &chapter.as_ref().unwrap(), &mut rng)
        .await
        .unwrap();
    let verse_identifiers = verse_id.split(".").collect::<Vec<&str>>();
    let verse = Verse::new(
        verse,
        book.to_string(),
        verse_identifiers[1].to_string(),
        verse_identifiers[2].to_string(),
    );
    Ok(verse)
}

/// list books for the current Bible version
pub async fn list_books(config: &Config) -> Result<Books> {
    let name = get_bible_info(config).await?;
    let books = get_books_by_name(config).await?;
    let book_info = Books::new(name, books);
    Ok(book_info)
}

/// get the name of the current Bible version
pub async fn get_bibles(config: &Config) -> Result<Vec<Bible>> {
    let url = BASE_URL[..BASE_URL.len() - 1].to_string();
    let (client, headers) = get_client_and_headers(config.api_key())?;
    let resp = client.get(url).headers(headers).send().await?;

    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
    let json_bibles = json["data"].as_array().unwrap();
    let mut bibles: Vec<Bible> = Vec::new();
    for bible in json_bibles {
        let name = bible["name"].as_str().unwrap().to_string();
        let id = bible["id"].as_str().unwrap().to_string();
        let description = bible["description"].as_str().unwrap_or("").to_string();
        let language = bible["language"]["name"].as_str().unwrap_or("").to_string();
        let bible = Bible::new(name, id, description, language);
        bibles.push(bible);
    }

    Ok(bibles)
}

async fn get_books_by_id(config: &Config) -> Result<Vec<String>> {
    let url = format!(
        "{BASE_URL}{version}/books",
        version = config.bible_version()
    );

    let (client, headers) = get_client_and_headers(config.api_key())?;
    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    let json: serde_json::Value = serde_json::from_str(&resp).expect("JSON was not well-formatted");

    let json_book_data = json["data"].as_array().unwrap();
    let mut books: Vec<String> = Vec::new();
    for book in json_book_data {
        books.push(book["id"].as_str().unwrap().to_string());
    }
    Ok(books)
}

async fn get_bible_info(config: &Config) -> Result<String> {
    let url = format!("{BASE_URL}{version}", version = config.bible_version());
    let (client, headers) = get_client_and_headers(config.api_key())?;
    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    let json: serde_json::Value = serde_json::from_str(&resp).expect("JSON was not well-formatted");
    let bible_name = json["data"]["name"].as_str().unwrap();
    Ok(bible_name.to_string())
}

async fn get_books_by_name(config: &Config) -> Result<Vec<String>> {
    let url = format!(
        "{BASE_URL}{version}/books",
        version = config.bible_version()
    );
    let (client, headers) = get_client_and_headers(config.api_key())?;

    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&resp).expect("JSON was not well-formatted");
    let json_book_data = json["data"].as_array().unwrap();
    let mut books: Vec<String> = Vec::new();
    for book in json_book_data {
        books.push(book["name"].as_str().unwrap().to_string());
    }
    Ok(books)
}

async fn book_id_to_name(config: &Config, book_id: &str) -> Result<String> {
    let url = format!(
        "{BASE_URL}{version}/books/{book_id}",
        version = config.bible_version(),
        book_id = book_id
    );
    let (client, headers) = get_client_and_headers(config.api_key())?;
    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    let json: serde_json::Value = serde_json::from_str(&resp).expect("JSON was not well-formatted");
    let book_name = json["data"]["name"].as_str().unwrap();
    Ok(book_name.to_string())
}

async fn get_random_verse(
    config: &Config,
    chapter: &str,
    rng: &mut StdRng,
) -> Result<(String, String)> {
    let url = format!(
        "{BASE_URL}{version}/chapters/{chapter}/verses",
        version = config.bible_version()
    );
    let (client, headers) = get_client_and_headers(config.api_key())?;
    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    let json: serde_json::Value = serde_json::from_str(&resp).expect("JSON was not well-formatted");
    let verse_list = json["data"].as_array().unwrap();
    let verse_index = rng.gen_range(0..verse_list.len());
    let verse = verse_list.get(verse_index).unwrap();
    let verse_id = verse["id"].as_str().unwrap().to_string();
    let url = format!(
        "{BASE_URL}{version}/verses/{verse_id}",
        version = config.bible_version()
    );
    let (client, mut headers) = get_client_and_headers(config.api_key())?;
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
        .await?
        .text()
        .await?;
    let json: serde_json::Value = serde_json::from_str(&resp).expect("JSON was not well-formatted");
    let verse_text = json["data"]["content"].as_str().unwrap().trim();
    let verse = String::from(verse_text);
    Ok((verse, verse_id))
}

async fn get_random_book(config: &Config, rng: &mut StdRng) -> Result<String> {
    let url = format!(
        "{BASE_URL}{version}/books",
        version = config.bible_version()
    );
    let (client, headers) = get_client_and_headers(config.api_key())?;
    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&resp).expect("JSON was not well-formatted");
    let book_list = json["data"].as_array().unwrap();
    let book_index = rng.gen_range(0..book_list.len());

    let book = book_list.get(book_index).unwrap();
    let book = book["id"].as_str().unwrap().to_string();

    Ok(book)
}

async fn get_random_chapter(config: &Config, book: &str, rng: &mut StdRng) -> Result<String> {
    let url = format!(
        "{BASE_URL}{version}/books/{book}/chapters",
        version = config.bible_version(),
        book = book
    );
    let (client, headers) = get_client_and_headers(config.api_key())?;
    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    let json: serde_json::Value = serde_json::from_str(&resp).expect("JSON was not well-formatted");
    let chapter_list = json["data"].as_array().unwrap();
    let mut chapter_index = rng.gen_range(0..chapter_list.len());
    let mut chapter = chapter_list.get(chapter_index).unwrap();
    if chapter["number"] == "intro" {
        chapter_index = chapter_index + 1;
        chapter = chapter_list.get(chapter_index).unwrap();
    }
    let chapter = chapter["id"].as_str().unwrap().to_string();
    Ok(chapter)
}
