use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::user::*;

const DATETIME_FORMAT: &str = "%Y";

#[derive(Serialize)]
pub struct Context {
    pub datetime: String,
    pub user: Option<User>
}

impl Context {
    
    pub fn empty() -> Self {
        Self {
            datetime: Utc::now().format(DATETIME_FORMAT).to_string(),
            user: None
        }
    }

    pub fn test() -> Self {
        Self {
            datetime: Utc::now().format(DATETIME_FORMAT).to_string(),
            user: Some(User {
                email:    String::from("test@test.com"),
                username: String::from("Username"),
                passhash: String::new()
            })
        }
    }

}