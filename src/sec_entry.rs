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
}

impl FilingType {
    pub fn which(filing_type: &str) -> Result<FilingType,()> {
        match filing_type {
            "S-1/A" => Ok(FilingType::SecS1),
            "5" => Ok(FilingType::Sec5),
            "4" => Ok(FilingType::Sec4),            
            "3" => Ok(FilingType::Sec3),
            _ =>  Err(()),
        }
    }

}

#[cfg(test)]
mod entry_tests {
    use super::*;

    #[test]
    fn which_test_s1(){
        assert_eq!(FilingType::which("S-1/A"),Ok(FilingType::SecS1));
    }

    #[test]
    fn which_test_345(){
        assert_eq!(FilingType::which("3"),Ok(FilingType::Sec3));
        assert_eq!(FilingType::which("4"),Ok(FilingType::Sec4));
        assert_eq!(FilingType::which("5"),Ok(FilingType::Sec5));
    }

}
