use reqwest::StatusCode;
use regex::Regex;
use xml::reader::{EventReader, XmlEvent};
//use std::io::prelude::*;
use crate::sec_entry::SECEntry;

const NUM_ENTRY_ELEMENTS: usize = 4;

pub fn read_rss(website: &str) -> Result<StatusCode, reqwest::Error> {
    println!("Reading");
    let xml = reqwest::get(website)?.text()?;
    let parsed_xml = parse_xml(xml);
    clean_xml(parsed_xml);
    println!("Line");
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
    assert!(xml.len() % NUM_ENTRY_ELEMENTS == 0);
    // Routine for every 4 entries
    let mut element_it = xml.iter();
    for _ in xml.iter().step_by(NUM_ENTRY_ELEMENTS) {
        let (filing_type,conformed_name,cik) = clean_title(element_it.next()).expect("Unable to get title element");
        println!("Filing: {}\nConformed name: {}\ncik: {}", &filing_type, &conformed_name, &cik);
        let (date, acc_number) = clean_filing(element_it.next()).expect("Unable to get filing element");
        let timestamp = clean_timestamp(element_it.next()).expect("Unable to get timestamp element");
        element_it.next();

        
    }
}
/// This function cleans the string received in the filing information from the xml
///      <b>Filed:</b> 2018-06-29 <b>AccNo:</b> 0001140361-18-030802 <b>Size:</b> 25 KB
/// Fields of interest here are the date, and accession number, both of which are between <\b>
/// <b> tags
/// Dates & Accession Numbers will be kept in usize format, so that we don't need any string
/// overhead.
///
/// Since we the data we want will have - in it with numbers on either side, we can use a regex/// to rip things out.
/// ```
/// // use super::*;
/// // I think doc tests are broken?
/// //assert_eq!(Ok((20180629,114036118030802)),clean_filing(Some(&"<b>Filed:</b> 2018-06-29 <b>AccNo:</b> 0001140361-18-030802 <b>Size:</b> 25 KB".to_string())));
///
/// ```
pub fn clean_filing(input: Option<&String>) -> Result<(usize,usize),&str> {
    match input {
        Some(f) => {
            let re = Regex::new(r"(\d*-\d*-\d*)").unwrap();
            let mut matches = re.captures_iter(&f).map(|a| a[1].to_owned()).collect::<Vec<String>>();

            // In place manipulation, for_each is eager as opposed to map()
            matches.iter_mut().for_each(|y: &mut String| y.retain(|x| x!= '-'));
            
            Ok((matches[0].parse::<usize>().expect("Could not convert to usize"),
               matches[1].parse::<usize>().expect("Could not convert to usize")))
            },
        _ => Err("Filing Title unclean"),
    }
}

pub fn clean_timestamp(input: Option<&String>) -> Result<(&String),&str> {
    match input {
        Some(x) => {
            Ok(&x)
        }
        None => Err("asdf"),
    }
}
/// 
pub fn clean_title(input: Option<&String>) -> Result<(&str,&str,usize),&str> {
    //! TODO: Make Errors that are helpful
    match input {
        Some(t) => {
            
            let split_names = t.split(" - ").collect::<Vec<&str>>();
            let vec = split_names[1].split(|c| c == '(' || c == ')').map(str::trim).collect::<Vec<&str>>();
            
            Ok((split_names[0],vec[0],vec[1].parse::<usize>().expect("Could Not convert to usize")))
        },
        None => panic!("No title for xml"),

    }

}

#[cfg(test)]
mod rss_tests {
    use super::*;

    #[test]
    fn clean_title_test_s1a(){
        assert_eq!(clean_title(Some(&"S-1/A - Tipmefast, Inc. (0001726079) (Filer)".to_owned())), Ok(("S-1/A","Tipmefast, Inc.",1726079)));
        
    }

    #[test]
    fn clean_title_test_standard(){
        assert_eq!(clean_title(Some(&"4 - Wang Janet (0001655081) (Reporting)".to_owned())), Ok(("4","Wang Janet",1655081)));
        
    }

    
    #[test]
    fn read_rss_test() {
        assert_eq!(read_rss("https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&CIK=&type=&company=&dateb=&owner=include&start=0&count=40&output=atom").unwrap(), StatusCode::Ok);
        assert!(read_rss("asdfajc").is_err());
    }

    #[test]
    fn clean_filing_test() {
                assert_eq!((20180629,114036118030802),clean_filing(Some(&"<b>Filed:</b> 2018-06-29 <b>AccNo:</b> 0001140361-18-030802 <b>Size:</b> 25 KB".to_string())).expect(""));


    }

}
