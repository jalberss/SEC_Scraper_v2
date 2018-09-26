//! The goal of this module is to see when to fetch data from the SEC
//!
//! -> could be based on time
//! -> could be based on when the rss feed updates
use crate::errors::*;
use reqwest::header::{ETAG, IF_NONE_MATCH};

pub fn get_rss(website: &str, etag: Option<&str>) -> Result<(String, String)> {
    let client = reqwest::Client::new();
    let mut res = match etag {
        Some(e) => client
            .get(website)
            .header(IF_NONE_MATCH, e)
            .send()
            .chain_err(|| "No Updated Website")?,
        None => client
            .get(website)
            .send()
            .chain_err(|| "Website not reached")?,
    };
    let etag = res
        .headers()
        .get(ETAG)
        .chain_err(|| "Could not get etag")?
        .to_str()
        .unwrap()
        .to_owned();

    let a = res.text().chain_err(|| "Unable to extract text")?;
    Ok((a, etag))
}

#[cfg(test)]
mod timing_test {
    use super::*;

    #[test]
    fn get_rss_1() {
        let res = get_rss("askduhalskjfgnawuehflnk", None);
        assert!(res.is_err());
        match res {
            Err(x) => assert!(x.kind().description() == "Website not reached"),
            _ => assert!(false),
        };
    }
}
