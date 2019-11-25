use std::env;
use diesel::prelude::*;

pub struct Config {

}

impl Config {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn establish_connection( &self ) -> Option<PgConnection> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        Some(PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url)))
    }

}