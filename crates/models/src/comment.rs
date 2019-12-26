use bowtie_data::schema::comments::dsl::comments as comments_dsl;
pub use bowtie_data::schema::*;

use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

use crate::error::*;
use crate::view::*;

model!(
    table:  comments,
    traits: [Identifiable,Associations],
    Comment {
        author:  i32,
        post:    i32,
        parent:  Option<i32>,
        body:    String,
        created: NaiveDateTime
});

#[derive(Serialize,Debug)]
pub struct Comments {
    pub root:     Option<Comment>,
    pub children: Vec<Box<Comments>>
}

impl Comments {

    pub fn new() -> Self {
        Self {
            root:     None,
            children: vec![]
        }
    }

    pub fn from( t_root: Option<Comment> ) -> Self {
        Self {
            root:     t_root,
            children: vec![]
        }
    }

    pub fn for_post(t_conn: &PgConnection, t_post: i32) -> Comments {
        let mut children: Vec<Comment> = 
        match comments::table
            .filter(comments::post.eq(t_post))
            .load::<CommentModel>(t_conn) {
                Ok(p)  => p.into_iter()
                        .map(|m| m.into())
                        .collect(),
                Err(_) => vec![]
            };

        Comments::for_data(None,&mut children)
    }

    pub fn for_data( t_root: Option<Comment>, t_comments: &mut Vec<Comment> ) -> Comments {
        // get id of the root
        let id = match t_root.as_ref() {
            Some(c) => c.id.clone(),
            None => None
        };

        let mut comments = Comments::from(t_root);
        
        // get all direct children of the root
        let mut i = 0;
        while i != t_comments.len() {
            if t_comments[i].parent == id {
                let value = t_comments.remove(i);
                let child = Comments::for_data(
                    Some(value),
                    t_comments
                );
                comments.children
                        .push(Box::new(child));
            } else {
                i += 1;
            }
        }

        // return
        comments
    }

}

impl Comment {

    pub fn create_from(t_conn: &PgConnection, t_author: i32, t_post: i32, t_parent: Option<i32>, t_body: String) -> Result<Comment,Error> {
        Comment::create(
            t_conn,
            Comment {
                id:      None,
                author:  t_author,
                post:    t_post,
                parent:  t_parent,
                body:    t_body,
                created: Utc::now().naive_utc()
            }
        )
    }

    pub fn delete_from(t_conn: &PgConnection, t_author: i32, t_id: i32) -> Result<Comment,Error> {
        let model: CommentModel = 
        diesel::delete(
            comments_dsl.filter(
                comments::author.eq(t_author)
                .and(comments::id.eq(t_id))
            ))
            .get_result(t_conn)?;

        // return the deleted model
        Ok(model.into())
    }

    pub fn create(t_conn: &PgConnection, t_comment: Comment) -> Result<Comment,Error> {
        // create model
        let model: CommentModel = 
            diesel::insert_into(comments::table)
            .values(&t_comment)
            .get_result(t_conn)?;

        Ok(model.into())
    }

    pub fn delete(t_conn: &PgConnection, t_author: View, t_id: i32) -> Result<Comment,Error> {
        match t_author.id {
            Some(id) => Comment::delete_from(t_conn,id,t_id),
            _ => Err(BowtieError::NoId)?
        }
    }
}