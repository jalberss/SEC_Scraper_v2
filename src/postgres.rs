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
) -> Result<(i32, String), diesel::result::Error> {
    use super::schema::accession_numbers;

    let new_post = NewAccessionNumber {
        accession_number: BigDecimal::from(acc_number as u64),
    };

    diesel::insert_into(accession_numbers::table)
        .values(&new_post)
        .get_result(conn)
}

pub fn delete_number(conn: &PgConnection, acc: usize) -> Result<usize, diesel::result::Error> {
    unimplemented!()
}
//     use super::schema::posts::dsl::*;

//     let acc = acc.to_string();
//     diesel::delete(posts.filter(acc_number.like(acc))).execute(conn)
// }

pub fn get_number(conn: &PgConnection, acc: usize) -> Option<Vec<AccessionNumber>> {
    unimplemented!()
}
//     use super::schema::posts::dsl::*;
//     let acc = acc.to_string();
//     let result = posts.filter(acc_number.like(acc)).load::<Post>(conn);
//     match result {
//         Ok(ref x) if (!x.is_empty()) => Some(x.to_owned()),
//         _ => None,
//     }
// }

// pub fn get_numbers(conn: &PgConnection) -> Vec<usize> {
//     use super::schema::posts::dsl::*;

//     let results = posts
//         .limit(5)
//         .load::<Post>(conn)
//         .expect("Error loading posts");

//     results
//         .iter()
//         .map(|x| x.acc_number.parse::<usize>().unwrap())
//         .collect::<Vec<usize>>()
// }

// pub fn get_posts(conn: &PgConnection) {
//     use super::schema::posts::dsl::*;

//     let results = posts
//         .limit(5)
//         .load::<Post>(conn)
//         .expect("Error loading posts");

//     for post in results {
//         println!("{}", post.acc_number);
//     }
// }

// pub fn delete_all_posts(conn: &PgConnection) {
//     use super::schema::posts::dsl::*;

//     diesel::delete(posts)
//         .execute(conn)
//         .expect("Error deleting posts");
// }
// pub fn write_test_number(
//     conn: &PgConnection,
//     acc_number: usize,
// ) -> Result<(i32, String), diesel::result::Error> {
//     use super::schema::test_posts;

//     let acc_number = acc_number.to_string();

//     let new_post = NewPost {
//         acc_number: &acc_number,
//     };

//     diesel::insert_into(test_posts::table)
//         .values(&new_post)
//         .get_result(conn)
// }

#[cfg(test)]
mod postgres_tests {
    use super::*;

    #[test]
    fn connection_test() {
        // Test will panic otherwise
        establish_connection("");
    }

    #[test]
    fn write_test() {
        use crate::schema::posts::dsl::*;
        let conn = establish_connection("");
        delete_all_posts(&conn);
        write_number(&conn, 6);
        let results = posts
            .limit(1)
            .load::<Post>(&conn)
            .expect("Error loading posts");
        assert!(results.iter().any(|a| a.acc_number == "6"));
    }

    #[test]
    fn delete_test() {
        use crate::schema::posts::dsl::*;
        let conn = establish_connection("");
        write_number(&conn, 6);
        delete_all_posts(&conn);
        let results = posts
            .limit(1)
            .load::<Post>(&conn)
            .expect("Error Loading posts");
        assert!(results.iter().next().is_none());
    }

    #[test]
    fn get_numbers_test() {
        let conn = establish_connection("");
        delete_all_posts(&conn);

        write_number(&conn, 1);
        write_number(&conn, 2);
        write_number(&conn, 3);
        assert_eq!(vec![1, 2, 3], get_numbers(&conn));
    }
}
