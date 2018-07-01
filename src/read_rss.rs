use reqwest::StatusCode;
use xml::reader::{EventReader, XmlEvent};
//use std::io::prelude::*;
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
            _ => (),
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

    let entries: Vec<SECEntry> = Vec::new();
    assert!(xml.len() % 4 == 0);
    // Routine for every 4 entries
    let mut element_it = xml.iter();
    for _ in xml.iter().step_by(4) {
        clean_title(element_it.next());
        clean_filing(element_it.next());
        clean_timestamp(element_it.next());
        element_it.next();
    }
}

pub fn clean_filing(input: Option<&String>) -> Result<(&str),&str> {
    unimplemented!();
}

pub fn clean_timestamp(input: Option<&String>) -> Result<(&str),&str> {
    unimplemented!();
}

pub fn clean_title(input: Option<&String>) -> Result<(&str,&str,&str),&str> {
    match input {
        Some(t) => {
            let vec = t.split(|c| c == '-'|| c == '(' || c == ')').map(|x| str::trim(x)).collect::<Vec<&str>>();
            Ok((vec[0],vec[1],vec[2]))
        },
        None => panic!("No title for xml"),

    }

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
