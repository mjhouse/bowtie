use crate::error::*;
pub use bowtie_data::schema::*;

use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

use crate::View;

#[derive(Serialize, Queryable, Debug)]
pub struct Post {
    pub id:      i32,
    pub view_id: i32,
    pub title:   String,
    pub body:    String,
    pub created: NaiveDateTime
}

#[derive(Identifiable,Insertable,Debug,Serialize)]
#[table_name="posts"]
pub struct PostModel {
    pub id:      Option<i32>,
    pub view_id: i32,
    pub title:   String,
    pub body:    String,
    pub created: NaiveDateTime
}

impl Post {
    
    pub fn create_from(
        t_conn: &PgConnection, 
        t_view: i32, 
        t_title: &str, 
        t_body: &str) -> Result<Post,Error> {

        Ok(diesel::insert_into(posts::table)
        .values(&PostModel {
            id:      None,
            view_id: t_view,
            title:   t_title.into(),
            body:    t_body.into(),
            created: Utc::now().naive_utc()
        })
        .get_result(t_conn)?)
    }

    pub fn delete_from(t_conn: &PgConnection, t_view: i32, t_id: i32) -> Result<Post,Error> {
        t_conn.transaction::<_, Error, _>(|| {
            
            // delete all comments
            diesel::delete(comments::table)
                .filter(comments::post.eq(t_id))
                .execute(t_conn)?;

            // delete the post
            let result = diesel::delete(posts::table)
                .filter(
                    posts::view_id.eq(t_view)
                    .and(posts::id.eq(t_id))
                )
                .get_result(t_conn)?;

            // return the deleted post
            Ok(result)
        })
    }

    pub fn for_view(t_conn: &PgConnection, t_id: i32) -> Vec<Post> {
        match posts::table
            .filter(posts::view_id.eq(t_id))
            .order(posts::created.asc())
            .load::<Post>(t_conn) {
                Ok(p) => p,
                Err(e) => {
                    warn!("Error during query: {}",e);
                    vec![]
                }
        }
    }

    pub fn for_id(t_conn: &PgConnection, t_id: i32) -> Result<(View,Post),Error> {
        match views::table
            .inner_join(
                posts::table
                .on(posts::view_id.eq(views::id))
            )
            .filter(posts::id.eq(t_id))
            .get_result::<(View,Post)>(t_conn) {
                Ok(p)  => Ok(p),
                Err(e) => {
                    warn!("Error during query: {}",e);
                    Err(BowtieError::RecordNotFound)?
                }
            }      
    }

    pub fn for_friends(t_conn: &PgConnection, t_id: i32) -> Vec<(View,Post)> {
        match views::table
            .inner_join(
                friends::table
                .on(friends::sender.eq(views::id)
                    .and(friends::receiver.eq(t_id))
                .or(friends::receiver.eq(views::id)
                    .and(friends::sender.eq(t_id)))
                .and(friends::accepted.eq(true)))
            )
            .inner_join(
                posts::table 
                .on(posts::view_id.eq(views::id))
            )
            .filter(views::id.ne(t_id))
            .select(
                ((
                    views::id,
                    views::user_id,
                    views::name,
                ),
                (    
                    posts::id,
                    posts::view_id,
                    posts::title,
                    posts::body,
                    posts::created
                )))
            .load::<(View,Post)>(t_conn) {
                Ok(r)  => r,
                Err(e) => {
                    warn!("Error during query: {}",e);
                    vec![]
                }
            }
    }

    pub fn for_followed(t_conn: &PgConnection, t_id: i32) -> Vec<(View,Post)> {
        match views::table
            .inner_join(
                follows::table
                .on(follows::publisher.eq(views::id)
                .and(follows::subscriber.eq(t_id)))
            )
            .inner_join(
                posts::table 
                .on(posts::view_id.eq(views::id))
            )
            .filter(views::id.ne(t_id))
            .select(
                ((
                    views::id,
                    views::user_id,
                    views::name,
                ),
                (    
                    posts::id,
                    posts::view_id,
                    posts::title,
                    posts::body,
                    posts::created
                )))
            .load::<(View,Post)>(t_conn) {
                Ok(r)  => r,
                Err(e) => {
                    warn!("Error during query: {}",e);
                    vec![]
                }
            }
    }

}