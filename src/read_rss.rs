use crate::models::Post;
use crate::postgres::*;
use crate::sec_entry::{FilingType, SECEntry};
use regex::Regex;
use reqwest::StatusCode;
use std::collections::HashSet;
use std::io::{BufReader, LineWriter, Read, Write};
use std::{fs::File, path::Path};
use xml::reader::{EventReader, XmlEvent};

use crate::errors::*;

const NUM_ENTRY_ELEMENTS: usize = 4;

pub fn read_rss(website: &str) -> Result<Vec<SECEntry>> {
    let xml = reqwest::get(website)
        .chain_err(|| "Unable to reach website")?
        .text()
        .chain_err(|| "Unable to get website text")?;
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

    println!("Ignore Set: {:#?}", &ignore);
    println!("{:#?}", &xml);

    // Routine for every 4 entries
    let mut element_it = xml.iter();
    for x in xml.iter().step_by(NUM_ENTRY_ELEMENTS) {
        println!("{}\n{:?}", &x, &element_it);
        let (filing_type, conformed_name, cik) =
            clean_title(element_it.next()).expect("Unable to get title element");

        let filing_enum = FilingType::which(filing_type).expect("Unable to find filing enum"); //.chain_err(|| "Unknown filing type given")?;

        println!("There");
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

            let entry = SECEntry::new(
                filing_enum,
                conformed_name.to_owned(),
                cik,
                acc_number,
                date,
                timestamp.to_owned(),
            );
            println!("Entry: {:#?}", &entry);
            //write_accession_number(acc_number).expect("Unable to write accession number");
            entries.push(entry);
        }
    }
    Ok(entries)
}

/// This function will check to see if an accesion number is not unique, and thus
/// must be ignore. The Programmer regrets this function, and will replace it with
/// database query
fn has_accession_number(acc_number: usize) -> Option<Vec<Post>> {
    let conn = establish_connection();
    get_number(&conn, acc_number)
}

fn write_accession_number(acc_number: usize) -> Result<(i32, String)> {
    let conn = establish_connection();
    write_number(&conn, acc_number).chain_err(|| "Unable to write accession Number")
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
pub fn clean_title(input: Option<&String>) -> Result<(&str, &str, usize)> {
    //! TODO: Make Errors that are helpful
    match input {
        Some(t) => {
            /* Get the form name, it may contain a -, which is why we take this approach */
            let split_names = t.split(" - ").collect::<Vec<&str>>();

            /* Get the conformed name, and accession number. The accession number is between two parens */
            let vec = split_names[1]
                .split(|c| c == '(' || c == ')')
                .map(str::trim)
                .collect::<Vec<&str>>();

            Ok((
                split_names[0],
                vec[0],
                vec[1]
                    .parse::<usize>()
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

    #[test]
    fn clean_title_test_s1a() {
        assert_eq!(
            clean_title(Some(
                &"S-1/A - Tipmefast, Inc. (0001726079) (Filer)".to_owned()
            )).unwrap(),
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
    #[ignore]
    fn read_rss_test() {
        assert!(read_rss("https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&CIK=&type=&company=&dateb=&owner=include&start=0&count=40&output=atom").is_ok());
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

        assert_eq!(Vec::<SECEntry>::new(), clean_xml(vec, ignore_set).unwrap());
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
        ].into_iter()
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
    fn
        "4 - Doyle Amy (0001708785) (Reporting)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001209191-18-049456 <b>Size:</b> 9 KB\n",
    "2018-09-05T13:14:49-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001209191-18-049456",
    "4 - LEAR CORP (0000842162) (Issuer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001209191-18-049456 <b>Size:</b> 9 KB\n",
    "2018-09-05T13:14:49-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001209191-18-049456",
    "D - CORRE HORIZON FUND, LP (0001751529) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000919574-18-006015 <b>Size:</b> 15 KB\n<br>Item 3C: Investment Company Act Section 3(c)\n<br>Item 3C.7: Section 3(c)(7)\n",
    "2018-09-05T13:14:10-04:00",
    "urn:tag:sec.gov,2008:accession-number=0000919574-18-006015",
    "497 - JNLNY SEPARATE ACCOUNT I (0001045032) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001045032-18-000285 <b>Size:</b> 67 KB\n",
    "2018-09-05T13:13:47-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001045032-18-000285",
    "13F-HR - Sturgeon Ventures LLP (0001720346) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001720346-18-000004 <b>Size:</b> 13 KB\n",
    "2018-09-05T13:12:55-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001720346-18-000004",
    "6-K - UNILEVER PLC (0000217410) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001564590-18-022654 <b>Size:</b> 60 MB\n",
    "2018-09-05T13:12:44-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001564590-18-022654",
    "8-K - CASEYS GENERAL STORES INC (0000726958) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000726958-18-000111 <b>Size:</b> 3 MB\n<br>Item 7.01: Regulation FD Disclosure\n<br>Item 9.01: Financial Statements and Exhibits\n",
    "2018-09-05T13:12:10-04:00",
    "urn:tag:sec.gov,2008:accession-number=0000726958-18-000111",
    "424B2 - JPMorgan Chase Financial Co. LLC (0001665650) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001615774-18-009188 <b>Size:</b> 313 KB\n",
    "2018-09-05T13:11:34-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001615774-18-009188",
    "424B2 - JPMORGAN CHASE & CO (0000019617) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001615774-18-009188 <b>Size:</b> 313 KB\n",
    "2018-09-05T13:11:34-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001615774-18-009188",
    "497 - JACKSON NATIONAL SEPARATE ACCOUNT V (0001072423) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001072423-18-000019 <b>Size:</b> 69 KB\n",
    "2018-09-05T13:11:08-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001072423-18-000019",
    "FWP - BANK OF NOVA SCOTIA (0000009631) (Subject)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000914121-18-001656 <b>Size:</b> 53 KB\n",
    "2018-09-05T13:10:41-04:00",
    "urn:tag:sec.gov,2008:accession-number=0000914121-18-001656",
    "4 - WILLIAMS PAUL S (0001236458) (Reporting)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001209191-18-049454 <b>Size:</b> 4 KB\n",
    "2018-09-05T13:08:50-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001209191-18-049454",
    "4 - ESSENDANT INC (0000355999) (Issuer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001209191-18-049454 <b>Size:</b> 4 KB\n",
    "2018-09-05T13:08:50-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001209191-18-049454",
    "D - UTAH LITHOTRIPSY LP (0001170568) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001170568-18-000001 <b>Size:</b> 5 KB\n",
    "2018-09-05T13:08:43-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001170568-18-000001",
    "497 - JACKSON NATIONAL SEPARATE ACCOUNT III (0001045034) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001045034-18-000020 <b>Size:</b> 69 KB\n",
    "2018-09-05T13:08:42-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001045034-18-000020",
    "FWP - CREDIT SUISSE AG (0001053092) (Subject)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000950103-18-010482 <b>Size:</b> 55 KB\n",
    "2018-09-05T13:07:31-04:00",
    "urn:tag:sec.gov,2008:accession-number=0000950103-18-010482",
    "8-K/A - BENCHMARK 2018-B2 Mortgage Trust (0001728339) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001539497-18-001414 <b>Size:</b> 23 KB\n<br>Item 8.01: Other Events\n",
    "2018-09-05T13:06:34-04:00",
    "urn:tag:sec.gov,2008:accession-number=0001539497-18-001414",
    "497 - JACKSON NATIONAL SEPARATE ACCOUNT - I (0000927730) (Filer)",
    "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000927730-18-000500 <b>Size:</b> 69 KB\n",
    "2018-09-05T13:06:11-04:00",
    "urn:tag:sec.gov,2008:accession-number=0000927730-18-000500",
    Iter(["497 - JACKSON NATIONAL SEPARATE ACCOUNT - I (0000927730) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000927730-18-000500 <b>Size:</b> 69 KB\n", "2018-09-05T13:06:11-04:00", "urn:tag:sec.gov,2008:accession-number=0000927730-18-000500", "FWP - BARCLAYS BANK PLC (0000312070) (Subject)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000950103-18-010481 <b>Size:</b> 115 KB\n", "2018-09-05T13:06:02-04:00", "urn:tag:sec.gov,2008:accession-number=0000950103-18-010481", "N-30B-2 - COUNTRY INVESTORS VARIABLE ANNUITY ACCOUNT (0001223662) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000910819-18-000067 <b>Size:</b> 3 KB\n", "2018-09-05T13:05:34-04:00", "urn:tag:sec.gov,2008:accession-number=0000910819-18-000067", "497 - VALIC Co I (0000719423) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001104659-18-055258 <b>Size:</b> 603 KB\n", "2018-09-05T13:05:23-04:00", "urn:tag:sec.gov,2008:accession-number=0001104659-18-055258", "4 - Knickerbocker Aron Marc (0001578766) (Reporting)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001209191-18-049452 <b>Size:</b> 7 KB\n", "2018-09-05T13:05:11-04:00", "urn:tag:sec.gov,2008:accession-number=0001209191-18-049452", "4 - FIVE PRIME THERAPEUTICS INC (0001175505) (Issuer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001209191-18-049452 <b>Size:</b> 7 KB\n", "2018-09-05T13:05:11-04:00", "urn:tag:sec.gov,2008:accession-number=0001209191-18-049452", "424B2 - CREDIT SUISSE AG (0001053092) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000950103-18-010480 <b>Size:</b> 253 KB\n", "2018-09-05T13:05:09-04:00", "urn:tag:sec.gov,2008:accession-number=0000950103-18-010480", "497 - JACKSON NATIONAL SEPARATE ACCOUNT - I (0000927730) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000927730-18-000499 <b>Size:</b> 69 KB\n", "2018-09-05T13:04:23-04:00", "urn:tag:sec.gov,2008:accession-number=0000927730-18-000499", "4 - Turitz Andrew (0001726221) (Reporting)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001182489-18-000322 <b>Size:</b> 8 KB\n", "2018-09-05T13:04:11-04:00", "urn:tag:sec.gov,2008:accession-number=0001182489-18-000322", "4 - Teladoc Health, Inc. (0001477449) (Issuer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0001182489-18-000322 <b>Size:</b> 8 KB\n", "2018-09-05T13:04:11-04:00", "urn:tag:sec.gov,2008:accession-number=0001182489-18-000322", "424B2 - BARCLAYS BANK PLC (0000312070) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000950103-18-010479 <b>Size:</b> 369 KB\n", "2018-09-05T13:04:10-04:00", "urn:tag:sec.gov,2008:accession-number=0000950103-18-010479", "DEF 14A - American Funds Retirement Income Portfolio Series (0001640102) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905", "DEF 14A - AMERICAN FUNDS DEVELOPING WORLD GROWTH & INCOME FUND (0001584433) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905", "DEF 14A - AMERICAN FUNDS INFLATION LINKED BOND FUND (0001553197) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905", "DEF 14A - AMERICAN FUNDS CORPORATE BOND FUND (0001553195) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905", "DEF 14A - AMERICAN FUNDS PORTFOLIO SERIES (0001537151) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905", "DEF 14A - AMERICAN FUNDS GLOBAL BALANCED FUND (0001505612) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905", "DEF 14A - AMERICAN FUNDS TAX-EXEMPT FUND OF NEW YORK (0001496999) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905", "DEF 14A - AMERICAN FUNDS MORTGAGE FUND (0001496998) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905", "DEF 14A - American Funds U.S. Government Money Market Fund (0001454975) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905", "DEF 14A - INTERNATIONAL GROWTH & INCOME FUND (0001439297) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905", "DEF 14A - American Funds Target Date Retirement Series (0001380175) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905", "DEF 14A - Short-Term Bond Fund of America (0001368040) (Filer)", "\n <b>Filed:</b> 2018-09-05 <b>AccNo:</b> 0000051931-18-000905 <b>Size:</b> 209 KB\n", "2018-09-05T13:03:32-04:00", "urn:tag:sec.gov,2008:accession-number=0000051931-18-000905"])
}
