use reqwest::{Response, StatusCode};
use xml::reader::{EventReader, XmlEvent};
use std::io::prelude::*;
use super::sec_entry::SECEntry;

pub fn read_rss(website: &str) -> Result<StatusCode, reqwest::Error> {
    println!("Reading");
    let xml = reqwest::get(website)?.text()?;
    let parsed_xml = parse_xml(xml);
    clean_xml(parsed_xml);
    Ok(reqwest::StatusCode::Ok)
}

pub fn parse_xml(xml: String) -> Vec<String> {
    let parser = EventReader::from_str(&xml);
    let mut entries: Vec<String> = Vec::new();
    let mut entry_tag: bool = false;
    // Parse and aggregate information that occurs within an entry element
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name.contains("entry") {
                    entry_tag = true;
                }
            }
            Ok(XmlEvent::Characters(c)) => {
                if entry_tag {
                    println!("{}",&c);
                    // TODO
                    // Do we already have this thing in our accession number list
                    // ENDTODO
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
    entries
}

pub fn clean_xml(xml: Vec<String>) {
    //! This function will clean up the XML given to it, and create a vector of
    //! entries that describe the SEC Filings
    // Assumption: The form of the `Entry` XMLElement to be parsed will be as follows
    // A title, which has the Type of Filing, Conformed Company Name, Central Index Key (CIK)
    // A filing information index, which has the Accession Number, and Data of Filing
    // A timestamp
    // A Tag that is ignored
    
    
    
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
