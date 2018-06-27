extern crate sec_scraper;

use sec_scraper::read_rss::read_rss;

const SEC_RSS_URL: &'static str = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&CIK=&type=&company=&dateb=&owner=include&start=0&count=40&output=atom";

fn main() {
    println!("Hello, world!");
    read_rss(SEC_RSS_URL);
}
