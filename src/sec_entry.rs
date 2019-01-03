use crate::errors::*;
use std::fmt::Write;

#[derive(Debug, PartialEq, Eq)]
pub struct SECEntry {
    filing_type: FilingType,
    name: String,
    cik: usize,
    accession_number: usize,
    date: usize,
    timestamp: String,
    url: String,
}

impl SECEntry {
    pub fn new(
        filing_type: FilingType,
        name: String,
        cik: usize,
        accession_number: usize,
        date: usize,
        timestamp: String,
    ) -> SECEntry {
        let url_ = SECEntry::get_url(cik, accession_number);

        SECEntry {
            filing_type,
            name,
            cik,
            accession_number,
            date,
            timestamp,
            url: url_,
        }
    }

    pub fn string(&self) -> String {
        let mut s = String::new();
        write!(
            s,
            "{:#?}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.filing_type,
            self.name,
            self.cik,
            self.accession_number,
            self.date,
            self.timestamp,
            self.url,
        )
        .expect("Could not write string");
        s
    }
    pub fn get_url(cik: usize, acc: usize) -> String {
        let mut s = String::new();
        let size_url = 18;

        let mut acc_hypen = acc.to_string();
        while acc_hypen.len() < size_url {
            acc_hypen.insert(0, '0');
        }
        let acc = acc_hypen.clone();
        acc_hypen.insert(acc_hypen.len() - 6, '-');
        acc_hypen.insert(acc_hypen.len() - 9, '-');

        write!(
            s,
            "https://www.sec.gov/Archives/edgar/data/{}/{}/{}-index.htm",
            cik, acc, acc_hypen
        );
        s
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FilingType {
    SecS1,
    Sec3,
    Sec3A,
    Sec4,
    Sec4A,
    Sec5,
    Sec6K,
    SecD,
    SecDA,
    SecFWP,
    Sec424B2,
    Sec497,
    Sec497K,
    Sec1012GA,
    Sec485APOS,
    SecN2,
    Sec10K,
    Sec10Q,
    Sec8K,
    Sec8KA,
    Sec13FHR,
    Sec13GA,
    SecPOSAM,
    Sec424B5,
    SecPRE14A,
    SecDFAN14A,
    SecSC13DA,
    Sec144,
    Sec13G,
    SecF10,
    Sec425,
    SecF3D,
    SecPRER14A,
    SecPRE14C,
}

impl FilingType {
    pub fn which(filing_type: &str) -> Result<FilingType> {
        println!("{}", filing_type);
        match filing_type {
            "S-1/A" => Ok(FilingType::SecS1),
            "5" => Ok(FilingType::Sec5),
            "4" => Ok(FilingType::Sec4),
            "4/A" => Ok(FilingType::Sec4A),
            "3" => Ok(FilingType::Sec3),
            "6-K" => Ok(FilingType::Sec6K),
            "FWP" => Ok(FilingType::SecFWP),
            "425" => Ok(FilingType::Sec425),
            "8-K/A" => Ok(FilingType::Sec8KA),
            "8-K" => Ok(FilingType::Sec8K),
            "497" => Ok(FilingType::Sec497),
            "D" => Ok(FilingType::SecD),
            "424B2" => Ok(FilingType::Sec424B2),
            "13F-HR" => Ok(FilingType::Sec13FHR),
            "SC 13G" => Ok(FilingType::Sec13G),
            "SC 13G/A" => Ok(FilingType::Sec13GA),
            "SC 13D/A" => Ok(FilingType::SecSC13DA),
            "497" => Ok(FilingType::Sec497),
            "497K" => Ok(FilingType::Sec497K),
            "POS AM" => Ok(FilingType::SecPOSAM),
            "D/A" => Ok(FilingType::SecDA),
            "424B5" => Ok(FilingType::Sec424B5),
            "PRE 14A" => Ok(FilingType::SecPRE14A),
            "DFAN14A" => Ok(FilingType::SecDFAN14A),
            "144" => Ok(FilingType::Sec144),
            "F-10" => Ok(FilingType::SecF10),
            "F-3D" => Ok(FilingType::SecF3D),
            "PRER14A" => Ok(FilingType::SecPRER14A),
            "PRE 14C" => Ok(FilingType::SecPRE14C),
            _ => Err("Filing not recognized")?,
        }
    }
}

#[cfg(test)]
mod entry_tests {
    use super::*;

    #[test]
    fn which_test_s1() {
        assert_eq!(FilingType::which("S-1/A").unwrap(), FilingType::SecS1);
        assert_eq!(FilingType::which("497").unwrap(), FilingType::Sec497);
    }

    #[test]
    fn which_test_345() {
        assert_eq!(FilingType::which("3").unwrap(), FilingType::Sec3);
        assert_eq!(FilingType::which("4").unwrap(), FilingType::Sec4);
        assert_eq!(FilingType::which("5").unwrap(), FilingType::Sec5);
    }

    #[test]
    fn stringify_entry() {
        let entry = SECEntry::new(
            FilingType::SecS1,
            String::from("Bollocks"),
            0,
            0,
            0,
            String::from("Also Bollocks"),
        );

        let mut oracle = String::new();
        write!(
            oracle,
            "SecS1\tBollocks\t0\t0\t0\tAlso Bollocks\t{}",
            SECEntry::get_url(0, 0)
        );
        assert_eq!(oracle, entry.string());
    }

    #[test]
    fn get_url_test() {
        //https://www.sec.gov/Archives/edgar/data/894158/000114420418049861/0001144204-18-049861-index.htm
        let mut s = SECEntry::get_url(894158, 114420418049861);
        assert_eq!(s,"https://www.sec.gov/Archives/edgar/data/894158/000114420418049861/0001144204-18-049861-index.htm");

        s = SECEntry::get_url(1525201, 90445418000574);
        assert_eq!(s,"https://www.sec.gov/Archives/edgar/data/1525201/000090445418000574/0000904454-18-000574-index.htm");
    }

}
