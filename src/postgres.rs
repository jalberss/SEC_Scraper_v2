extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

// TODO This should return a result
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

//pub fn write_number(

#[cfg(test)]
mod postgres_tests {
    use super::*;

    #[test]
    fn connection_test(){
        // Test will panic otherwise
        establish_connection();
    }

        
}

