use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

// define constants

const BASE_URL: &str = "https://api.scripture.api.bible/v1/bibles";

// define public functions
pub async fn get_daily_verse(api_key: &str, version: &str) -> Result<(), reqwest::Error> {
    // I need to create an algorithm to first get a seeded random book, then a random chapter, then a
    // random verse
    // this should all be fairly straight forward.

    Ok(())
}

pub async fn get_new_verse(api_key: &str, version: &str) -> Result<(), reqwest::Error> {
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

async fn get_bibles(api_key: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    // Set up the request headers with the API key
    let mut headers = HeaderMap::new();
    let url = BASE_URL;

    headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());

    let resp = client.get(url).headers(headers).send().await?;

    let resp_body = resp.text().await?;

    println!("Response body = {:?}", resp_body);
    println!("body = {:?}", resp_body);

    Ok(())
}

pub async fn list_books(api_key: &str, version: &str) -> Result<(), reqwest::Error> {
    Ok(())
}

async fn get_random_book(api_key: &str, version: &str) -> Result<(), reqwest::Error> {
    Ok(())
}

async fn get_random_chapter(api_key: &str, version: &str) -> Result<(), reqwest::Error> {
    Ok(())
}

async fn get_random_verse(api_key: &str, version: &str) -> Result<(), reqwest::Error> {
    Ok(())
}
