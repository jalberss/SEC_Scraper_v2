//! The goal of this module is to see when to fetch data from the SEC
//!
//! -> could be based on time
//! -> could be based on when the rss feed updates
use crate::errors::*;
use reqwest::header::ETAG;

/// In a perfect World, this would only request when given a valid etag,
/// but no one likes to give etags.
pub fn get_rss(website: &str, _cached_etag: Option<&str>) -> Result<(String, String)> {
    let client = reqwest::Client::new();
    let mut res = client
        .get(website)
        .header(ETAG, "Blah")
        .send()
        .chain_err(|| "Website not reached")?;
    println!("{:#?}", &res);
    // Todo figure how etags work
    let etag = "NO TAG".to_string();

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

    #[test]
    fn check_etag() {
        // Etags allow clients to make conditional requests. In our case, we wish to
        // continue with the request iff the page has changed.
        //
        let res = get_rss("http://www.wsj.com", None);
        if let Err(e) = res {
            println!("{}", e);
            assert!(false);
        }
        assert!(true);
    }

}
