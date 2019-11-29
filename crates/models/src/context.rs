use serde::{Serialize};
use chrono::{Utc};
use crate::user::*;
use crate::post::*;

const DATETIME_FORMAT: &str = "%Y";

#[derive(Serialize,Default)]
pub struct Context {
    pub datetime: String,
    pub user:  Option<User>,
    pub posts: Vec<Post>,
    pub flash: Option<String>
}

impl Context {

    pub fn empty() -> Self {
        Self {
            datetime: Utc::now().format(DATETIME_FORMAT).to_string(),
            user:  None,
            posts: vec![],
            flash: None
        }
    }

}