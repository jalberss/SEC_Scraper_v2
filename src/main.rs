use std::path::Path;

use sec_scraper::cli::*;
use sec_scraper::read_rss::read_rss;
use sec_scraper::timing::get_rss;
use sec_scraper::write_entries::write_table;
use std::collections::HashSet;

const SEC_RSS_URL: &str = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&CIK=&type=&company=&dateb=&owner=include&start=0&count=40&output=atom";

fn main() {
    let _ = init();
    let log_file: &Path = Path::new("accession_numbers.txt");
    println!("Hello, world!");
    let etag: Option<&str> = None;
    let (xml, _) = get_rss(SEC_RSS_URL, etag).unwrap();
    let entries = read_rss(&xml, HashSet::new());
    if let Ok(entries) = entries {
        write_table(log_file, entries).expect("Could not write table");
    }
}
