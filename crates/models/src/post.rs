pub use bowtie_data::{schema::*,traits::*};
use crate::view::View;

use diesel::prelude::*;
use serde::{Serialize};
use chrono::prelude::*;
use std::env;

use diesel::result::Error as DieselError;

#[derive(FromForm)]
pub struct PostForm {
    pub title:   String,
    pub body:    String,
}

model!(
    table:  posts,
    owner:  (View),
    traits: [Identifiable,Associations],
    Post {
        view_id: i32,
        title:   String,
        body:    String,
        created: NaiveDateTime
});

access!( Post,
    id:i32     => posts::id,
    title:&str => posts::title
);

impl Post {
    
    pub fn create(t_conn: &PgConnection, view: &View, t_title: &str, t_body: &str) -> Result<Post,DieselError> {
        let post = Post {
            id:      None,
            view_id: view.id.unwrap_or(-1),
            title:   t_title.into(),
            body:    t_body.into(),
            created: Utc::now().naive_utc()
        };
    
        diesel::insert_into(posts::table)
            .values(&post)
            .get_result(t_conn)
            .or_else(|e|  Err(e))
            .and_then(|p: PostModel| Ok(p.into()))
    }

    pub fn delete(&self, t_conn: &PgConnection) -> Result<(),DieselError> {
        match self.id {
            Some(id) => {
                match diesel::delete(
                        posts::dsl::posts.filter(posts::id.eq(id)))
                        .execute(t_conn) {
                            Ok(_)  => Ok(()),
                            Err(e) => Err(e)
                        }
            }
            None => Err(DieselError::NotFound)
        }
    }

    pub fn for_user(t_conn: &PgConnection, t_id: i32) -> Vec<Post> {
        query!(many: t_conn, posts::view_id.eq(t_id), posts::created.asc())
    }

}