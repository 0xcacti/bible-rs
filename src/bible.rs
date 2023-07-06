pub mod display;

use anyhow::Result;
use rand_core::SeedableRng;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT},
    Client,
};

use rand::{rngs::StdRng, Rng};

use chrono::Local;

// define constants

const BASE_URL: &str = "https://api.scripture.api.bible/v1/bibles/";

// define public functions

/// fetch a daily random verse
pub async fn get_daily_verse(api_key: &str, version: &str) {
    let seed = get_rng_seed_from_date();
    let mut rng = StdRng::seed_from_u64(seed);

    let book = get_random_book(api_key, version, &mut rng).await;
    let chapter = get_random_chapter(api_key, version, &book.as_ref().unwrap(), &mut rng).await;
    let (verse, verse_id) =
        get_random_verse(api_key, version, &chapter.as_ref().unwrap(), &mut rng)
            .await
            .unwrap();
    let book_and_chapter_id = chapter.as_ref().unwrap().split_once(".").unwrap();
    display::print_verse(
        verse.as_str(),
        book_id_to_name(api_key, version, book_and_chapter_id.0)
            .await
            .unwrap()
            .as_str(),
        book_and_chapter_id.1,
        verse_id.as_str(),
    );
}

pub async fn get_new_verse(api_key: &str, version: &str) {
    let seed: u64 = rand::thread_rng().gen();
    let mut rng = StdRng::seed_from_u64(seed);

    let book = get_random_book(api_key, version, &mut rng).await;
    let chapter = get_random_chapter(api_key, version, &book.as_ref().unwrap(), &mut rng).await;
    let (verse, verse_id) =
        get_random_verse(api_key, version, &chapter.as_ref().unwrap(), &mut rng)
            .await
            .unwrap();

    display::print_verse(
        verse.as_str(),
        book.unwrap().as_str(),
        chapter.unwrap().as_str(),
        verse_id.as_str(),
    );
}

pub async fn get_new_verse_from_book(api_key: &str, version: &str, book: &str) {
    // check book is in the list of books
    let book_names = get_books_by_name(api_key, version).await;
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
    let book_ids = get_books_by_id(api_key, version).await;
    let book_id = &book_ids.unwrap()[book_id];
    let seed: u64 = rand::thread_rng().gen();
    let mut rng = StdRng::seed_from_u64(seed);
    let chapter = get_random_chapter(api_key, version, &book_id, &mut rng).await;
    let (verse, verse_id) =
        get_random_verse(api_key, version, &chapter.as_ref().unwrap(), &mut rng)
            .await
            .unwrap();
    display::print_verse(
        verse.as_str(),
        book,
        chapter.unwrap().as_str(),
        verse_id.as_str(),
    );
}

pub async fn list_books(api_key: &str, version: &str) {
    let name = get_bible_info(api_key, version).await;
    let books = get_books_by_name(api_key, version).await;
    display::print_book_list(books.unwrap(), name.unwrap().as_str());
}

async fn get_books_by_id(api_key: &str, version: &str) -> Result<Vec<String>> {
    let client = Client::new();
    // Set up the request headers with the API api_key
    let mut headers = HeaderMap::new();
    let url = format!("{BASE_URL}{version}/books");
    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());

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

async fn get_bible_info(api_key: &str, version: &str) -> Result<String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!("{BASE_URL}{version}");
    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
    let resp = client.get(url).headers(headers).send().await?;
    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
    let bible_name = json["data"]["name"].as_str().unwrap();
    Ok(bible_name.to_string())
}

async fn get_books_by_name(api_key: &str, version: &str) -> Result<Vec<String>> {
    let client = Client::new();
    // Set up the request headers with the API api_key
    let mut headers = HeaderMap::new();
    let url = format!("{BASE_URL}{version}/books");
    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());

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

async fn book_id_to_name(api_key: &str, version: &str, book_id: &str) -> Result<String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!("{BASE_URL}{version}/books/{book_id}");
    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
    let resp = client.get(url).headers(headers).send().await?;
    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
    let book_name = json["data"]["name"].as_str().unwrap();
    Ok(book_name.to_string())
}

// private functions
fn get_rng_seed_from_date() -> u64 {
    // get the current date
    // hash the date string
    // truncate the hash to 8
    // convert the truncated hash to a u64
    let date = Local::now().naive_local().date();
    let date_hash = sha256::digest(date.to_string().as_bytes());
    let truncated_hash = &date_hash[0..16];
    hex_to_u64(truncated_hash.as_bytes()).unwrap()
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

async fn get_random_book(api_key: &str, version: &str, rng: &mut StdRng) -> Result<String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!("{BASE_URL}{version}/books");
    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
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

async fn get_random_chapter(
    api_key: &str,
    version: &str,
    book: &str,
    rng: &mut StdRng,
) -> Result<String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!("{BASE_URL}{version}/books/{book}/chapters");
    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
    let resp = client.get(url).headers(headers).send().await?;
    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
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

async fn get_random_verse(
    api_key: &str,
    version: &str,
    chapter: &str,
    rng: &mut StdRng,
) -> Result<(String, String)> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!("{BASE_URL}{version}/chapters/{chapter}/verses");
    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
    let resp = client.get(url).headers(headers).send().await?;
    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
    let verse_list = json["data"].as_array().unwrap();
    let verse_index = rng.gen_range(0..verse_list.len());
    let verse = verse_list.get(verse_index).unwrap();
    let verse_id = verse["id"].as_str().unwrap().to_string();
    let url = format!("{BASE_URL}{version}/verses/{verse_id}");

    let mut headers = HeaderMap::new();
    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
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
    Ok((verse, verse_index.to_string()))
}
