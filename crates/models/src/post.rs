pub use bowtie_data::{schema::*,traits::*};
use crate::view::View;
use crate::error::*;

use diesel::prelude::*;
use serde::{Serialize};
use chrono::prelude::*;
use std::env;

use bowtie_data::schema::posts::dsl::posts as posts_dsl;

use diesel::ConnectionError as ConnectionError;
use diesel::result::Error as DieselError;
use failure::*;

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

impl_for!( Post,
    id:i32     => posts::id,
    title:&str => posts::title
);

impl Post {
    
    pub fn create_from(t_view: i32, t_title: &str, t_body: &str) -> Result<Post,Error> {
        Post::create(Post {
            id:      None,
            view_id: t_view,
            title:   t_title.into(),
            body:    t_body.into(),
            created: Utc::now().naive_utc()
        })
    }

    pub fn create(t_post: Post) -> Result<Post,Error> {
        let uri  = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&uri)?;

        conn.transaction::<_, Error, _>(|| {
            // create model
            let model: PostModel = 
                diesel::insert_into(posts::table)
                .values(&t_post)
                .get_result(&conn)?;

            Ok(model.into())
        })
    }

    pub fn delete(t_post: Post) -> Result<Post,Error> {
        let uri  = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&uri)?;

        conn.transaction::<_, Error, _>(|| {
            let id = match t_post.id {
                Some(id) => id,
                _ => Err(BowtieError::NoId)?
            };

            // @todo delete comments when post is deleted
            // @body not need to create comment model first

            // delete the post
            let model: PostModel = 
            diesel::delete(
                posts_dsl.filter(
                    posts::id.eq(id)))
                .get_result(&conn)?;

            // return the deleted post
            Ok(model.into())
        })
    }

    pub fn for_view(t_conn: &PgConnection, t_id: i32) -> Vec<Post> {
        query!(many: t_conn, posts::view_id.eq(t_id), posts::created.asc())
    }

}