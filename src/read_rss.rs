use crate::models::AccessionNumber;
use crate::postgres::*;
use crate::sec_entry::{FilingType, SECEntry};
use regex::Regex;
use std::collections::HashSet;
use xml::reader::{EventReader, XmlEvent};

use crate::errors::*;

const NUM_ENTRY_ELEMENTS: usize = 4;

pub fn read_rss(xml: &str) -> Result<Vec<SECEntry>> {
    let parsed_xml = parse_xml(&xml);
    clean_xml(parsed_xml, HashSet::new()) //TODO replace
}

pub fn parse_xml(xml: &str) -> Vec<String> {
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

pub fn clean_xml(xml: Vec<String>, ignore: HashSet<FilingType>) -> Result<Vec<SECEntry>> {
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
    let mut element_it = xml.iter();
    for _ in xml.iter().step_by(NUM_ENTRY_ELEMENTS) {
        let (filing_type, conformed_name, cik) =
            clean_title(element_it.next()).expect("Unable to get title element");

        let filing_enum =
            FilingType::which(filing_type).chain_err(|| "Unknown filing type given")?;

        /* Ignore if of certain filing type(s)*/
        if ignore.contains(&filing_enum) {
            ignore_filing(&mut element_it);
        } else {
            let (date, acc_number) =
                clean_filing(element_it.next()).chain_err(|| "Unable to get filing element")?;

            /* CIKs are not unique, i.e. a company/individual will have the same*/
            /* CIK each time it files with the SEC */

            let timestamp = clean_timestamp(element_it.next())
                .chain_err(|| "Unable to get timestamp element")?;
            element_it.next();

            println!("cik: {:#?}", &cik);
            let entry = SECEntry::new(
                filing_enum,
                conformed_name.to_owned(),
                cik,
                acc_number,
                date,
                timestamp.to_owned(),
            );
            if has_accession_number(acc_number).is_some() {
                write_accession_number(acc_number);
                entries.push(entry);
            }
        }
    }
    Ok(entries)
}

/// This function will check to see if an accesion number is not unique, and thus
/// must be ignore. The Programmer regrets this function, and will replace it with
/// database query
/// ^
/// It's replaced :)
fn has_accession_number(acc_number: usize) -> Option<Vec<AccessionNumber>> {
    let conn = establish_connection("");
    get_number(&conn, acc_number).and_then(|c| if c.len() == 0 { None } else { Some(c) })
}

fn write_accession_number(acc_number: usize) {
    // -> Result<(i32, String)> {
    let conn = establish_connection("");
    write_number(&conn, acc_number).chain_err(|| "Unable to write accession Number");
}

fn delete_accession_number(acc_number: usize) -> Result<usize> {
    let conn = establish_connection("");
    delete_number(&conn, acc_number).chain_err(|| "Unable to delete accession Number")
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
pub fn clean_filing(input: Option<&String>) -> Result<(usize, usize)> {
    match input {
        Some(f) => {
            let re = Regex::new(r"(\d*-\d*-\d*)").unwrap();
            let mut matches = re
                .captures_iter(&f)
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
        _ => bail!("Filing title unclean"),
    }
}

pub fn clean_timestamp(input: Option<&String>) -> Result<(&String)> {
    match input {
        Some(x) => Ok(&x),
        None => Err("Unable to clean timestamp")?,
    }
}
///
pub fn clean_title<'a>(input: Option<&'a String>) -> Result<(&'a str, &'a str, usize)> {
    //! TODO: Make Errors that are helpful
    match input {
        Some(t) => {
            /* Get the form name, it may contain a -, which is why we take this approach */
            let split_names = t.split(" - ").collect::<Vec<&str>>();
            /* Get the conformed name, and accession number. The accession number is between two parens */

            let re = Regex::new(r"\((\d+)\)").unwrap();
            let cik = &re.captures_iter(&t).next().unwrap()[1];

            let name: Vec<&'a str> = (&split_names[1]).split('(').map(&str::trim).collect();

            Ok((
                split_names[0],
                name[0],
                cik.parse::<usize>()
                    .chain_err(|| "Could not convert to usize")?,
            ))
        }
        None => Err("No xml title found")?,
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
    use bigdecimal::BigDecimal;

    #[test]
    fn clean_title_test_s1a() {
        assert_eq!(
            clean_title(Some(
                &"S-1/A - Tipmefast, Inc. (0001726079) (Filer)".to_owned()
            ))
            .unwrap(),
            ("S-1/A", "Tipmefast, Inc.", 1726079)
        );
    }

    #[test]
    fn clean_title_test_standard() {
        assert_eq!(
            clean_title(Some(&"4 - Wang Janet (0001655081) (Reporting)".to_owned())).unwrap(),
            ("4", "Wang Janet", 1655081)
        );
    }

    #[test]
    fn clean_filing_test() {
        assert_eq!(
            (20180629, 114036118030802),
            clean_filing(Some(
                &"<b>Filed:</b> 2018-06-29 <b>AccNo:</b> 0001140361-18-030802 <b>Size:</b> 25 KB"
                    .to_string()
            ))
            .expect("")
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

        assert_eq!(Vec::<SECEntry>::new(), clean_xml(vec, ignore_set).unwrap());
    }

    #[test]
    fn clean_xml_ignore1() {
        let _entry = SECEntry::new(
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

    #[test]
    fn clean_xml_mega_test() {
        let test = vec![
            "4 - REDIKER DENNIS L (0001189878) (Reporting)",
            "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001127602-18-026759 <b>Size:</b> 4 KB\n",
            "2018-09-05T12:36:45-04:00",
            "urn:tag:sec.gov,2008:accession-number=0001127602-18-026759",
            "4 - MARTIN MARIETTA MATERIALS INC (0000916076) (Issuer)",
            "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001127602-18-026759 <b>Size:</b> 4 KB\n",
            "2018-09-05T12:36:45-04:00",
            "urn:tag:sec.gov,2008:accession-number=0001127602-18-026759",
            "FWP - WELLS FARGO & COMPANY/MN (0000072971) (Subject)",
            "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001387131-18-004493 <b>Size:</b> 108 KB\n",
            "2018-09-05T12:36:29-04:00",
            "urn:tag:sec.gov,2008:accession-number=0001387131-18-004493",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        if let Ok(x) = clean_xml(test, HashSet::new()) {
            println!("{:#?}", &x);
            assert_eq!(x.len(), 3);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn clean_xml_mega_mega_test() {
        let test = vec![
            "497 - JACKSON NATIONAL SEPARATE ACCOUNT - I (0000927730) (Filer)",
            "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000927730-18-000500 <b>Size:</b> 69 KB\n",
            "2018-09-05T13:06:11-04:00",
            "urn:tag:sec.gov,2008:accession-number=0000927730-18-000500",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        let entry = SECEntry::new(
            FilingType::Sec497,
            String::from("JACKSON NATIONAL SEPARATE ACCOUNT"),
            927730,
            92773018000500,
            20180905,
            String::from("2018-09-05T13:06:11-04:00"),
        );

        if let Ok(mut x) = clean_xml(test, HashSet::new()) {
            assert_eq!(x.pop().unwrap(), entry);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn accession_number_test() {
        let x = 1337;
        let val = BigDecimal::from(1337);
        delete_accession_number(x).expect("Better Not Fail");
        assert_eq!(has_accession_number(x), None);
        write_accession_number(x);
        assert_eq!(
            has_accession_number(x)
                .unwrap()
                .pop()
                .unwrap()
                .accession_number,
            val
        );
        assert!(delete_accession_number(x).is_ok());
    }
}
