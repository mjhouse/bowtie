use bowtie_data::schema::*;
use crate::user::User;

use diesel::prelude::*;
use serde::{Serialize};

use diesel::result::Error as DieselError;

#[derive(FromForm)]
pub struct PostForm {
    pub title:   String,
    pub body:    String,
}

model!(
    table:  "posts",
    traits: [Identifiable,Associations],
    owner:  (User),
    Post {
        user_id: i32,
        title:   String,
        body:    String
});

impl Post {
    
    pub fn create(t_conn: &PgConnection, user: &User, t_title: &str, t_body: &str) -> Result<Post,DieselError> {
        let post = Post {
            id:      None,
            user_id: user.id.unwrap_or(-1),
            title:   t_title.into(),
            body:    t_body.into()
        };
    
        diesel::insert_into(posts::table)
            .values(&post)
            .get_result(t_conn)
            .or_else(|e|  Err(e))
            .and_then(|p: PostModel| Ok(p.into()))
    }

}