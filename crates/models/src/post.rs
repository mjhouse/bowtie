use crate::view::View;
use crate::error::*;

pub use bowtie_data::schema::*;

use diesel::prelude::*;
use serde::{Serialize};
use chrono::prelude::*;

use bowtie_data::schema::posts::dsl::posts as posts_dsl;
use failure::*;

// Creates insertion and query structs (<Object>/<Object>Model),
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

// Creates 'for_<field>' query functions.
queries!( 
    table: posts,
    model: Post,
    one: {
        id:i32 => posts::id
    }
);

impl Post {
    
    pub fn create_from(t_conn: &PgConnection, t_view: i32, t_title: &str, t_body: &str) -> Result<Post,Error> {
        Post::create(
            t_conn,
            Post {
                id:      None,
                view_id: t_view,
                title:   t_title.into(),
                body:    t_body.into(),
                created: Utc::now().naive_utc()
            }
        )
    }

    pub fn delete_from(t_conn: &PgConnection, t_view: i32, t_id: i32) -> Result<Post,Error> {
        let model: PostModel = 
        diesel::delete(
            posts_dsl.filter(
                posts::view_id.eq(t_view)
                .and(posts::id.eq(t_id))
            ))
            .get_result(t_conn)?;

        // return the deleted post
        Ok(model.into())
    }

    pub fn create(t_conn: &PgConnection, t_post: Post) -> Result<Post,Error> {
        // create model
        let model: PostModel = 
            diesel::insert_into(posts::table)
            .values(&t_post)
            .get_result(t_conn)?;

        Ok(model.into())
    }

    pub fn delete(t_conn: &PgConnection, t_post: Post) -> Result<Post,Error> {
        t_conn.transaction::<_, Error, _>(|| {
            let id = match t_post.id {
                Some(id) => id,
                _ => Err(BowtieError::NoId)?
            };

            // @todo delete comments when post is deleted
            // @body need to create comment model first

            // delete the post
            let model: PostModel = 
            diesel::delete(
                posts_dsl.filter(
                    posts::id.eq(id)))
                .get_result(t_conn)?;

            // return the deleted post
            Ok(model.into())
        })
    }

    pub fn for_view(t_conn: &PgConnection, t_id: i32) -> Vec<Post> {
        match posts::table
            .filter(posts::view_id.eq(t_id))
            .order(posts::created.asc())
            .load::<PostModel>(t_conn) {
            Ok(p) => {
                p.into_iter()
                    .map(|m| m.into())
                    .collect()
            },
            Err(e) => {
                warn!("Error during query: {}",e);
                vec![]
            }
        }
    }

}