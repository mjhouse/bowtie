use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::user::*;

const DATETIME_FORMAT: &str = "%Y";

#[derive(Serialize)]
pub struct Context {
    pub datetime: String,
    pub user:  Option<User>,
    pub flash: Option<String>
}

impl Context {
    
    pub fn empty() -> Self {
        Self {
            datetime: Utc::now().format(DATETIME_FORMAT).to_string(),
            user:  None,
            flash: None
        }
    }

    pub fn flash( flash: Option<String> ) -> Self {
        let mut context = Context::empty();
        context.flash = flash;
        context
    }

    pub fn test() -> Self {
        let mut context = Context::empty();
        context.user = Some(User {
            email:    Some(String::from("test@test.com")),
            username: String::from("Username"),
            passhash: String::new()
        });
        context
    }

}