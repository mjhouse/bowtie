use crate::schema::posts;
use serde::{Serialize};

#[derive(FromForm)]
pub struct PostForm {
    pub title:   String,
    pub body:    String
}

#[derive(Insertable,Debug,Serialize)]
#[table_name="posts"]
pub struct Post {
    pub title:     String,
    pub body:      String,
    pub published: bool
}