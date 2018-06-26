extern crate reqwest;
extern crate xml;

use read_rss::reqwest::{Response, StatusCode};
use read_rss::xml::reader::{EventReader, XmlEvent};
use std::io::prelude::*;

const SEC_RSS_URL: &'static str = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&CIK=&type=&company=&dateb=&owner=include&start=0&count=40&output=atom";

pub fn read_rss() -> Result<StatusCode, reqwest::Error> {
    println!("Reading");
    let xml = reqwest::get(SEC_RSS_URL)?.text()?;
    parse_xml(xml);
    Ok(reqwest::StatusCode::Ok)
}

pub fn parse_xml(xml: String) {
    let parser = EventReader::from_str(&xml);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("name {}", name);
            }
            Ok(e) => println!("{:#?}", e),
            _ => println!("Nothing"),
        }
    }
}

#[cfg(test)]
mod rss_tests {
    use super::*;
    #[test]
    fn read_rss_test() {
        assert_eq!(read_rss().unwrap(), StatusCode::Ok);
    }

}
