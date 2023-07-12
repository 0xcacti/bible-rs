use anyhow::{Context, Result};
use chrono::Local;
use rand::{rngs::StdRng, Rng};
use rand_core::SeedableRng;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

pub fn get_date() -> String {
    let date = Local::now().naive_local().date();
    date.to_string()
}

pub fn get_rng_from_date(date: String) -> StdRng {
    let date_hash = sha256::digest(date.as_bytes());
    let truncated_hash = &date_hash[0..16];
    let seed = hex_to_u64(truncated_hash.as_bytes()).unwrap();
    StdRng::seed_from_u64(seed)
}

pub fn get_rng() -> StdRng {
    let seed: u64 = rand::thread_rng().gen();
    StdRng::seed_from_u64(seed)
}

pub fn hex_to_u64(b: &[u8]) -> Option<u64> {
    let a = std::str::from_utf8(b).ok()?;
    u64::from_str_radix(a, 16).ok()
}

pub fn get_client_and_headers(api_key: &str) -> Result<(Client, HeaderMap)> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "api-key",
        HeaderValue::from_str(api_key).context("error inserting api-key")?,
    );
    Ok((client, headers))
}
