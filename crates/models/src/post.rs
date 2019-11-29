pub use bowtie_data::schema::*;

use crate::user::User;

use diesel::prelude::*;
use serde::{Serialize};
use chrono::prelude::*;

use diesel::result::Error as DieselError;

#[derive(FromForm)]
pub struct PostForm {
    pub title:   String,
    pub body:    String,
}

macro_rules! query_by {
    ( $c:expr, $q:expr ) => {
        match posts::table
            .filter($q)
            .first::<PostModel>($c)
        {
            Ok(u) => Some(u.into()),
            Err(e) => {
                warn!("Error during query: {}",e);
                None
            }
        }
    }
}

model!(
    table:  "posts",
    traits: [Identifiable,Associations],
    owner:  (User),
    Post {
        user_id: i32,
        title:   String,
        body:    String,
        created: NaiveDateTime
});

impl Post {
    
    pub fn create(t_conn: &PgConnection, user: &User, t_title: &str, t_body: &str) -> Result<Post,DieselError> {
        let post = Post {
            id:      None,
            user_id: user.id.unwrap_or(-1),
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

    pub fn from_id(t_conn: &PgConnection, t_id: i32) -> Option<Post> {
        query_by!(t_conn,posts::id.eq(t_id))
    }

    pub fn for_user( t_conn: &PgConnection, t_id: i32 ) -> Vec<Post> {
        match posts::table
        .filter(posts::user_id.eq(t_id))
        .load::<PostModel>(t_conn) {
            Ok(p) => {
                p.into_iter()
                 .map(|m| m.into())
                 .collect()
            },
            Err(e) => {
                warn!("{}",e);
                vec![]
            }
        }
    }

}