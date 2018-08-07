extern crate diesel;

use super::models::{NewPost, Post};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// TODO This should return a result
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn write_number(
    conn: &PgConnection,
    acc_number: usize,
) -> Result<(i32, String), diesel::result::Error> {
    use super::schema::posts;

    let acc_number = acc_number.to_string();

    let new_post = NewPost {
        acc_number: &acc_number,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
}

pub fn get_posts(conn: &PgConnection) {
    use super::schema::posts::dsl::*;

    let results = posts
        .limit(5)
        .load::<Post>(conn)
        .expect("Error loading posts");

    for post in results {
        println!("{}", post.acc_number);
    }
}

pub fn delete_all_posts(conn: &PgConnection) {
    use super::schema::posts::dsl::*;

    diesel::delete(posts)
        .execute(conn)
        .expect("Error deleting posts");
}

#[cfg(test)]
mod postgres_tests {
    use super::*;

    #[test]
    fn connection_test() {
        // Test will panic otherwise
        establish_connection();
    }

    #[test]
    fn write_test() {
        use crate::schema::posts::dsl::*;
        let conn = establish_connection();
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
        let conn = establish_connection();
        write_number(&conn, 6);
        delete_all_posts(&conn);
        let results = posts
            .limit(1)
            .load::<Post>(&conn)
            .expect("Error Loading posts");
        assert!(results.iter().next().is_none());
    }
}
