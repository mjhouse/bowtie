use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

use serde::{
    Serialize,
    Deserialize
};

use crate::user::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {

}

impl Site {
    pub fn new() -> Self {
        Self {}
    }
    pub fn database( &self ) -> Result<PgConnection,()> {
        env::var("DATABASE_URL")
        .or_else(|e| Err(()))
        .and_then(|u| {
            PgConnection::establish(&u)
                .or_else(|e| Err(()))
        })
    }
}