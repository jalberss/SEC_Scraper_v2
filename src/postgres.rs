use super::models::{AccessionNumber, NewAccessionNumber};
use bigdecimal::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// Should we abstract this into allowing us to choose the table
pub fn establish_connection(url: &str) -> PgConnection {
    dotenv().ok();

    if let Ok(database_url) = env::var("DATABASE_URL") {
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    } else {
        PgConnection::establish(&url).expect("")
    }
}

pub fn write_number(
    conn: &PgConnection,
    acc_number: usize,
) -> Result<usize, diesel::result::Error> {
    use super::schema::accession_numbers;

    let new_post = NewAccessionNumber {
        accession_number: BigDecimal::from(acc_number as u64),
    };

    diesel::insert_into(accession_numbers::table)
        .values(&new_post)
        .execute(conn)
}

pub fn delete_number(conn: &PgConnection, acc: usize) -> Result<usize, diesel::result::Error> {
    use super::schema::accession_numbers::dsl::*;

    diesel::delete(accession_numbers.filter(accession_number.eq(BigDecimal::from(acc as u64))))
        .execute(conn)
}

pub fn get_number(conn: &PgConnection, acc: usize) -> Option<Vec<AccessionNumber>> {
    use super::schema::accession_numbers::dsl::*;

    accession_numbers
        .filter(accession_number.eq(BigDecimal::from(acc as u64)))
        .limit(5)
        .load::<AccessionNumber>(conn)
        .ok()
        .and_then(|x| if x.len() > 0 { Some(x) } else { None })
}

pub fn get_numbers(conn: &PgConnection) -> Option<Vec<BigDecimal>> {
    use super::schema::accession_numbers::dsl::*;

    accession_numbers
        .limit(5)
        .load::<AccessionNumber>(conn)
        .ok()
        .map(|c| {
            c.into_iter()
                .map(|x| x.accession_number)
                .collect::<Vec<BigDecimal>>()
        })
}

pub fn get_posts(conn: &PgConnection) {
    use super::schema::accession_numbers::dsl::*;

    let results = accession_numbers
        .limit(5)
        .load::<AccessionNumber>(conn)
        .expect("Error loading posts");

    for post in results {
        println!("{}", post.accession_number);
    }
}

pub fn delete_all_posts(conn: &PgConnection) {
    use super::schema::accession_numbers::dsl::*;

    diesel::delete(accession_numbers)
        .execute(conn)
        .expect("Error deleting posts");
}

#[cfg(test)]
mod postgres_tests {
    use super::*;
    use bigdecimal::BigDecimal;

    #[test]
    fn connection_test() {
        // Test will panic otherwise
        establish_connection("");
    }

    #[test]
    fn write_test() {
        use crate::schema::accession_numbers::dsl::*;
        let conn = establish_connection("");
        delete_all_posts(&conn);
        assert!(write_number(&conn, 6).is_ok());
        let results = accession_numbers
            .limit(1)
            .load::<AccessionNumber>(&conn)
            .expect("Error loading posts");
        let b = results
            .iter()
            .any(|a| a.accession_number == BigDecimal::from(6));
        delete_all_posts(&conn);
        assert!(b);
    }

    #[test]
    fn delete_test() {
        use crate::schema::accession_numbers::dsl::*;
        let conn = establish_connection("");
        assert!(write_number(&conn, 6).is_ok());
        delete_all_posts(&conn);
        let results = accession_numbers
            .limit(1)
            .load::<AccessionNumber>(&conn)
            .expect("Error Loading posts");
        assert!(results.iter().next().is_none());
    }

    #[test]
    fn get_numbers_test() {
        let conn = establish_connection("");
        delete_all_posts(&conn);

        assert!(write_number(&conn, 1).is_ok());
        assert!(write_number(&conn, 2).is_ok());
        assert!(write_number(&conn, 3).is_ok());
        let v = vec![1, 2, 3];
        let v = v
            .into_iter()
            .map(|x| BigDecimal::from(x))
            .collect::<Vec<BigDecimal>>();
        assert_eq!(v, get_numbers(&conn).unwrap());
        delete_all_posts(&conn);
    }
}
