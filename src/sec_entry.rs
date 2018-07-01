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
