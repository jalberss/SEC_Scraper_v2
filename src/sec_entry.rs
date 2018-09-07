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
        SECEntry {
            filing_type,
            name,
            cik,
            accession_number,
            date,
            timestamp,
        }
    }

    pub fn string(&self) -> String {
        let mut s = String::new();
        write!(
            s,
            "{:#?}\t{}\t{}\t{}\t{}\t{}",
            self.filing_type, self.name, self.cik, self.accession_number, self.date, self.timestamp
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
    }

    #[test]
    fn which_test_345() {
        assert_eq!(FilingType::which("3").unwrap(), FilingType::Sec3);
        assert_eq!(FilingType::which("4").unwrap(), FilingType::Sec4);
        assert_eq!(FilingType::which("5").unwrap(), FilingType::Sec5);
    }

    #[test]
    fn stringify_entry() {
        //assert_eq!(FilingType::SecS1.string(), "S-1/A");
        let entry = SECEntry::new(
            FilingType::SecS1,
            String::from("Bollocks"),
            0,
            0,
            0,
            String::from("Also Bollocks"),
        );
        assert_eq!(entry.string(), "SecS1\tBollocks\t0\t0\t0\tAlso Bollocks");
    }

}
