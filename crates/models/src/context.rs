use serde::{Serialize};
use chrono::{Utc};
use crate::view::*;
use crate::post::*;
use crate::message::*;
use crate::search::*;
use crate::session::*;

const DATETIME_FORMAT: &str = "%Y";

#[derive(Serialize,Default)]
pub struct Context<'a> {
    pub route:     &'a str,
    pub datetime:  String,
    pub session:   Option<Session>,

    pub views:     Vec<View>,
    pub view:      Option<View>,
    pub posts:     Vec<Post>,
    pub post:      Option<Post>,
    pub messages:  Vec<Message>,
    pub message:   Option<Message>,

    pub search:    Option<Search>,
    
    pub sheet:     String,
    pub flash:     Option<String>
}

impl Context<'_> {

    pub fn empty() -> Self {
        Self {
            route:     "/",
            datetime:  Utc::now().format(DATETIME_FORMAT).to_string(),
            session:   None,

            views:     vec![],
            view:      None,
            posts:     vec![],
            post:      None,
            messages:  vec![],
            message:   None,
            
            search:    None,
            sheet:     String::new(),
            flash:     None
        }
    }

}