use regex::Regex;
use reqwest::StatusCode;
use xml::reader::{EventReader, XmlEvent};
//use std::io::prelude::*;
use crate::sec_entry::{FilingType, SECEntry};
use std::collections::HashSet;

const NUM_ENTRY_ELEMENTS: usize = 4;

pub fn read_rss(website: &str) -> Result<StatusCode, reqwest::Error> {
    let xml = reqwest::get(website)?.text()?;
    let parsed_xml = parse_xml(xml);
    let entries = clean_xml(parsed_xml, HashSet::new()); //TODO replace
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

pub fn clean_xml(xml: Vec<String>, ignore: HashSet<FilingType>) -> Result<Vec<SECEntry>, ()> {
    //! This function will clean up the XML given to it, and create a vector of
    //! entries that describe the SEC Filings
    // Assumption: The form of the `Entry` XMLElement to be parsed will be as follows
    // A title, which has the Type of Filing, Conformed Company Name, Central Index Key (CIK)
    // A filing information index, which has the Accession Number, and Data of Filing
    // A timestamp
    // A Tag that is ignored

    let mut entries: Vec<SECEntry> = Vec::new();
    assert!(xml.len() % NUM_ENTRY_ELEMENTS == 0);
    // Routine for every 4 entries
    println!("{:#?}", xml);
    let mut element_it = xml.iter();
    for _ in xml.iter().step_by(NUM_ENTRY_ELEMENTS) {
        let (filing_type, conformed_name, cik) =
            clean_title(element_it.next()).expect("Unable to get title element");

        let filing_enum = FilingType::which(filing_type)?;

        /* Ignore if of certain filing type(s)*/
        if ignore.contains(&filing_enum) {
            ignore_filing(&mut element_it);
        } else {
            let (date, acc_number) =
                clean_filing(element_it.next()).expect("Unable to get filing element");
            let timestamp =
                clean_timestamp(element_it.next()).expect("Unable to get timestamp element");
            element_it.next();

            let entry = SECEntry::new(
                filing_enum,
                conformed_name.to_owned(),
                cik,
                acc_number,
                date,
                timestamp.to_owned(),
            );
            entries.push(entry);
        }
    }
    Ok(entries)
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
pub fn clean_filing(input: Option<&String>) -> Result<(usize, usize), &str> {
    match input {
        Some(f) => {
            let re = Regex::new(r"(\d*-\d*-\d*)").unwrap();
            let mut matches = re.captures_iter(&f)
                .map(|a| a[1].to_owned())
                .collect::<Vec<String>>();

            // In place manipulation, for_each is eager as opposed to map()
            matches
                .iter_mut()
                .for_each(|y: &mut String| y.retain(|x| x != '-'));

            Ok((
                matches[0]
                    .parse::<usize>()
                    .expect("Could not convert to usize"),
                matches[1]
                    .parse::<usize>()
                    .expect("Could not convert to usize"),
            ))
        }
        _ => Err("Filing Title unclean"),
    }
}

pub fn clean_timestamp(input: Option<&String>) -> Result<(&String), &str> {
    match input {
        Some(x) => Ok(&x),
        None => Err("asdf"),
    }
}
///
pub fn clean_title(input: Option<&String>) -> Result<(&str, &str, usize), &str> {
    //! TODO: Make Errors that are helpful
    match input {
        Some(t) => {
            /* Get the form name, it may contain a -, which is why we take this approach */
            let split_names = t.split(" - ").collect::<Vec<&str>>();

            /* Get the conformed name, and accession number. The accession number is between tow parens */
            let vec = split_names[1]
                .split(|c| c == '(' || c == ')')
                .map(str::trim)
                .collect::<Vec<&str>>();

            Ok((
                split_names[0],
                vec[0],
                vec[1].parse::<usize>().expect("Could Not convert to usize"),
            ))
        }
        None => panic!("No title for xml"),
    }
}

#[inline]
pub fn ignore_filing<T: Iterator>(iter: &mut T) {
    iter.next();
    iter.next();
    iter.next();
}

#[cfg(test)]
mod rss_tests {
    use super::*;

    #[test]
    fn clean_title_test_s1a() {
        assert_eq!(
            clean_title(Some(
                &"S-1/A - Tipmefast, Inc. (0001726079) (Filer)".to_owned()
            )),
            Ok(("S-1/A", "Tipmefast, Inc.", 1726079))
        );
    }

    #[test]
    fn clean_title_test_standard() {
        assert_eq!(
            clean_title(Some(&"4 - Wang Janet (0001655081) (Reporting)".to_owned())),
            Ok(("4", "Wang Janet", 1655081))
        );
    }

    #[test]
    #[ignore]
    fn read_rss_test() {
        assert_eq!(read_rss("https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&CIK=&type=&company=&dateb=&owner=include&start=0&count=40&output=atom").unwrap(), StatusCode::Ok);
        assert!(read_rss("asdfajc").is_err());
    }

    #[test]
    fn clean_filing_test() {
        assert_eq!(
            (20180629, 114036118030802),
            clean_filing(Some(
                &"<b>Filed:</b> 2018-06-29 <b>AccNo:</b> 0001140361-18-030802 <b>Size:</b> 25 KB"
                    .to_string()
            )).expect("")
        );
    }

    #[test]
    fn clean_xml_ignore0() {
        let test =
            vec![    "4/A - Wilson Andrew (0001545193) (Reporting)",
    "\n <b>Filed:</b> 2018-07-05 <b>AccNo:</b> 0001454387-18-000188 <b>Size:</b> 5 KB\n",
    "2018-07-05T20:51:01-04:00",
                                     "urn:tag:sec.gov,2008:accession-number=0001454387-18-000188"];
        let vec = test.into_iter().map(String::from).collect::<Vec<String>>();
        let mut ignore_set = HashSet::new();
        ignore_set.insert(FilingType::Sec4A);

        assert_eq!(Ok(Vec::new()), clean_xml(vec, ignore_set));
    }
    #[test]
    fn clean_xml_ignore1() {
        let entry = SECEntry::new(
            FilingType::Sec4A,
            String::from("Wilson Andrew"),
            1545193,
            145438718000188,
            20180705,
            String::from("2018-07-05T20:51:01-04:00"),
        );

        let test = vec![
            "4/A - Wilson Andrew (0001545193) (Reporting)",
            "\n <b>Filed:</b> 2018-07-05 <b>AccNo:</b> 0001454387-18-000188 <b>Size:</b> 5 KB\n",
            "2018-07-05T20:51:01-04:00",
            "urn:tag:sec.gov,2008:accession-number=0001454387-18-000188",
            "4 - Wilson Andrew (0001545193) (Reporting)",
            "\n <b>Filed:</b> 2018-07-05 <b>AccNo:</b> 0001454387-18-000188 <b>Size:</b> 5 KB\n",
            "2018-07-05T20:51:01-04:00",
            "urn:tag:sec.gov,2008:accession-number=0001454387-18-000188",
        ];
        let vec = test.into_iter().map(String::from).collect::<Vec<String>>();
        let mut ignore_set = HashSet::new();
        ignore_set.insert(FilingType::Sec4A);
        let entry = SECEntry::new(
            FilingType::Sec4,
            String::from("Wilson Andrew"),
            1545193,
            145438718000188,
            20180705,
            String::from("2018-07-05T20:51:01-04:00"),
        );

        assert_eq!(Some(entry), clean_xml(vec, ignore_set).unwrap().pop());
    }

    #[test]
    fn clean_xml_test() {
        let test =
            vec![    "4/A - Wilson Andrew (0001545193) (Reporting)",
    "\n <b>Filed:</b> 2018-07-05 <b>AccNo:</b> 0001454387-18-000188 <b>Size:</b> 5 KB\n",
    "2018-07-05T20:51:01-04:00",
                             "urn:tag:sec.gov,2008:accession-number=0001454387-18-000188"];
        let vec = test.into_iter().map(str::to_owned).collect::<Vec<String>>();
        let entry = SECEntry::new(
            FilingType::Sec4A,
            String::from("Wilson Andrew"),
            1545193,
            145438718000188,
            20180705,
            String::from("2018-07-05T20:51:01-04:00"),
        );
        assert_eq!(Some(entry), clean_xml(vec, HashSet::new()).unwrap().pop());
    }

}
