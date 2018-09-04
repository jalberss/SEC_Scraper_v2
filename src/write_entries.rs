use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::errors::*;
use crate::sec_entry::SECEntry;

fn write_table(path: &Path, entries: Vec<SECEntry>) -> Result<()> {
    let mut file = File::create(path).chain_err(|| format!("{:#?} not found", path))?;
    Ok(())
}

fn write_entry(entry: SECEntry, file: File) {}

#[cfg(test)]
mod write_entries_tests {
    use super::*;
    #[test]
    fn write_table_test() {
        assert!(write_table(Path::new("asdf.txt"), vec![]).is_ok());
        std::fs::remove_file("asdf.txt");
    }

}
