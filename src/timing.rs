//! The goal of this module is to see when to fetch data from the SEC
//!
//! -> could be based on time
//! -> could be based on when the rss feed updates
use http::HttpTryFrom;
use reqwest::header::IF_NONE_MATCH;
use reqwest::*;

pub fn get_rss(website: &str, etag: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://www.rust-lang.org")
        .header(IF_NONE_MATCH, etag)
        .send()?;
    Ok(())
}
