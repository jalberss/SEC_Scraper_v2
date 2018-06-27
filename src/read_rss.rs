extern crate reqwest;
extern crate xml;

use read_rss::reqwest::{Response, StatusCode};
use read_rss::xml::reader::{EventReader, XmlEvent};
use std::io::prelude::*;

pub fn read_rss(website: &str) -> Result<StatusCode, reqwest::Error> {
    println!("Reading");
    let xml = reqwest::get(website)?.text()?;
    parse_xml(xml);
    Ok(reqwest::StatusCode::Ok)
}

pub fn parse_xml(xml: String) {
    let parser = EventReader::from_str(&xml);
    let mut entries: Vec<String> = Vec::new();
    let mut entry_tag: bool = false;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name.contains("entry") {
                    // This sucks
                    entry_tag = true;
                }
            }
            Ok(XmlEvent::Characters(c)) => {
                if entry_tag {
                    println!("chars {}", &c);
                    entries.push(c);
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name.contains("entry") {
                    entry_tag = false;
                }
            }

            _ => println!("Nothing"),
        }
    }
}

pub fn clean_xml(xml: String) {
    unimplemented!();
}

#[cfg(test)]
mod rss_tests {
    use super::*;

    #[test]
    fn read_rss_test() {
        assert_eq!(read_rss("https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&CIK=&type=&company=&dateb=&owner=include&start=0&count=40&output=atom").unwrap(), StatusCode::Ok);
        assert!(read_rss("asdfajc").is_err());
    }

}
