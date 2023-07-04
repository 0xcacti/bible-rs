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
pub async fn get_daily_verse(api_key: &str, version: &str) -> Result<(), reqwest::Error> {
    // I need to create an algorithm to first get a seeded random book, then a random chapter, then a
    // random verse
    // this should all be fairly straight forward.
    let seed = get_rng_seed_from_date();
    let mut rng = StdRng::seed_from_u64(seed);
    println!("seed = {:?}", rng);

    let book = get_random_book(api_key, version, &mut rng).await;
    let chapter = get_random_chapter(api_key, version, &book.as_ref().unwrap(), &mut rng).await;
    let verse = get_random_verse(
        api_key,
        version,
        &book.unwrap(),
        &chapter.unwrap(),
        &mut rng,
    )
    .await;

    // get a random book
    Ok(())
}

pub async fn get_new_verse(api_key: &str, version: &str) -> Result<(), reqwest::Error> {
    let seed: u64 = rand::thread_rng().gen();
    let mut rng = StdRng::seed_from_u64(seed);

    let book = get_random_book(api_key, version, &mut rng).await;
    let chapter = get_random_chapter(api_key, version, &book.as_ref().unwrap(), &mut rng).await;
    let verse = get_random_verse(
        api_key,
        version,
        &book.unwrap(),
        &chapter.unwrap(),
        &mut rng,
    )
    .await;
    Ok(())
}

pub async fn get_new_verse_from_book(
    api_key: &str,
    version: &str,
    book: &str,
) -> Result<(), reqwest::Error> {
    Ok(())
}

pub async fn get_books(api_key: &str, version: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    // Set up the request headers with the API api_key
    let mut headers = HeaderMap::new();
    let url = format!("{BASE_URL}{version}/books");
    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());

    let resp = client.get(url).headers(headers).send().await?;

    let resp_body = resp.text().await?;

    println!("Response body = {:?}", resp_body);
    println!("body = {:?}", resp_body);
    Ok(())
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

async fn get_bibles(api_key: &str) -> Result<(), reqwest::Error> {
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

async fn get_random_book(
    api_key: &str,
    version: &str,
    rng: &mut StdRng,
) -> Result<(String), reqwest::Error> {
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
    println!("book = {:?}", book);
    let book = book["id"].as_str().unwrap().to_string();

    Ok(book)
}

async fn get_random_chapter(
    api_key: &str,
    version: &str,
    book: &str,
    rng: &mut StdRng,
) -> Result<(String), reqwest::Error> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!("{BASE_URL}{version}/books/{book}/chapters");
    println!("{url}");
    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
    let resp = client.get(url).headers(headers).send().await?;
    let resp_body = resp.text().await?;
    let json: serde_json::Value =
        serde_json::from_str(&resp_body).expect("JSON was not well-formatted");
    let chapter_list = json["data"].as_array().unwrap();
    println!("chapter_list = {:?}", chapter_list);
    let mut chapter_index = rng.gen_range(0..chapter_list.len());
    let mut chapter = chapter_list.get(chapter_index).unwrap();
    if chapter["number"] == "intro" {
        chapter_index = chapter_index + 1;
        chapter = chapter_list.get(chapter_index).unwrap();
    }
    let chapter = chapter["id"].as_str().unwrap().to_string();
    println!("chapter = {:?}", chapter);
    Ok(chapter)
}

async fn get_random_verse(
    api_key: &str,
    version: &str,
    book: &str,
    chapter: &str,
    rng: &mut StdRng,
) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let url = format!("{BASE_URL}{version}/chapters/{chapter}/verses");

    println!("{url}");

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
    println!("json = {:?}", json);

    let verse_text = json["data"]["content"].as_str().unwrap();
    let verse = String::from(verse_text);
    println!("verse = {:?}", verse.trim());
    Ok(())
}
