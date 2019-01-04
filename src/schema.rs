table! {
    accession_numbers (id) {
        id -> Int4,
        accession_number -> Numeric,
    }
}

table! {
    test_accession_numbers (id) {
        id -> Int4,
        accession_number -> Numeric,
    }
}

allow_tables_to_appear_in_same_query!(
    accession_numbers,
    test_accession_numbers,
);
