use super::schema::accession_numbers;
use bigdecimal::BigDecimal;

#[derive(Queryable)]
pub struct AccessionNumber {
    pub id: i32,
    pub accession_number: BigDecimal,
}

#[derive(Insertable)]
#[table_name = "accession_numbers"]
pub struct NewAccessionNumber {
    pub accession_number: BigDecimal,
}
