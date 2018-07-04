#[derive (Debug,PartialEq,Eq)]
pub struct SECEntry {
    filing_type: FilingType,
    name: String,
    cik: usize,
    accession_number: usize,
    timestamp: String,
}

impl SECEntry {
    pub fn new(filing_type: FilingType, name: String, cik: usize, accession_number: usize, timestamp: String) -> SECEntry {
        SECEntry {
            filing_type,
            name,
            cik,
            accession_number,
            timestamp,
        }
    }

}
#[derive (Debug,PartialEq,Eq)]
pub enum FilingType {
    SecS1,
    Sec3,
    Sec3A,
    Sec4,
    Sec4A,
    Sec5,
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
    DEBUG,
}

impl FilingType {
    fn which(filing_type: &str) -> FilingType {
        match filing_type {
            "S-1/A" => FilingType::SecS1,
            _ => FilingType::DEBUG,
        }
    }

}

#[cfg(test)]
mod entry_tests {
    use super::*;

    #[test]
    fn which_test(){
        assert_eq!(FilingType::which("S-1/A"),FilingType::SecS1);
    }
}
