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
        match env::var("DATABASE_URL") {
            Ok(p) => {
                match PgConnection::establish(&p) {
                    Ok(c) => Some(c),
                    _ => None
                } 
            },
            _ => None
        }
    }

}