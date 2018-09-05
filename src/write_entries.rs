use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

use crate::errors::*;
use crate::sec_entry::{FilingType, SECEntry};

pub fn write_table(path: &Path, entries: Vec<SECEntry>) -> Result<()> {
    let mut file = File::create(path).chain_err(|| format!("{:#?} not found", path))?;
    // The most functional Rust I have ever written

    write!(
        file,
        "Filing Type\tName\tCIK\tAccession Number\tDate\tTime\n"
    );
    write_entries(file, entries)
}

fn write_entries(file: File, entries: Vec<SECEntry>) -> Result<()> {
    let mut entries: Vec<String> = entries
        .iter()
        .map(|entry| entry.string())
        .collect::<Vec<String>>();

    entries.iter_mut().for_each(|entry| entry.push('\n'));

    entries
        .iter()
        .map(|entry| file.write_all(entry.as_bytes()))
        .fold(Ok(()), |acc, x| acc.and(x))
        .chain_err(|| "Write failed")?;
    Ok(())
}

#[cfg(test)]
mod write_entries_tests {
    use super::*;

    #[test]
    fn write_table_test_basic() {
        let name = String::from("asdf.txt");
        File::create(&name);
        assert!(write_entries(File::open(&name).unwrap(), vec![]).is_ok());
        std::fs::remove_file(&name).is_ok();
    }

    #[test]
    fn write_table_test_intermediate() {
        let name = String::from("int.txt");
        let file = File::create(&name).unwrap();

        let entry = SECEntry::new(
            FilingType::SecS1,
            String::from("Bollocks"),
            0,
            0,
            0,
            String::from("Also Bollocks"),
        );

        assert!(write_entries(file, vec![entry]).is_ok());

        let mut f = File::open(&name).expect("file not found");

        let mut string = String::new();

        f.read_to_string(&mut string);

        let oracle = String::from("SecS1\tBollocks\t0\t0\t0\tAlso Bollocks\n");

        assert_eq!(oracle, string);

        std::fs::remove_file(&name);
    }

    #[test]
    fn write_table_test_advanced() {
        let name = String::from("adv.txt");

        let file = File::create(&name).unwrap();

        let entry1 = SECEntry::new(
            FilingType::SecS1,
            String::from("Bollocks"),
            0,
            0,
            0,
            String::from("Also Bollocks"),
        );

        let entry2 = SECEntry::new(
            FilingType::SecS1,
            String::from("Bollocks"),
            0,
            0,
            0,
            String::from("Also Bollocks"),
        );

        assert!(write_entries(file, vec![entry1, entry2]).is_ok());

        let mut f = File::open(&name).expect("file not found");
        let mut string = String::new();

        f.read_to_string(&mut string);

        let oracle = String::from(
            "SecS1\tBollocks\t0\t0\t0\tAlso Bollocks\nSecS1\tBollocks\t0\t0\t0\tAlso Bollocks\n",
        );

        assert_eq!(oracle, string);

        std::fs::remove_file(&name);
    }

}
