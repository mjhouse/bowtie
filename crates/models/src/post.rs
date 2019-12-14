pub use bowtie_data::{schema::*,traits::*};
use crate::view::View;

use diesel::prelude::*;
use serde::{Serialize};
use chrono::prelude::*;
use std::env;

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
    
    pub fn create(t_view: i32, t_title: &str, t_body: &str) -> Result<Post,Error> {
        let uri  = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&uri)?;

        let post = Post {
            id:      None,
            view_id: t_view,
            title:   t_title.into(),
            body:    t_body.into(),
            created: Utc::now().naive_utc()
        };
        
        conn.transaction::<_, Error, _>(|| {
            // create model
            let mut model: PostModel = 
                diesel::insert_into(posts::table)
                .values(&post)
                .get_result(&conn)?;

            Ok(model.into())
        })
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

    // pub fn destroy_all(t_id:i32) -> Result<(),Error> {
    //     let uri  = env::var("DATABASE_URL")?;
    //     let conn = PgConnection::establish(&uri)?;

    //     diesel::delete(
    //         dsl::posts.filter(posts::view_id.eq(t_id)))
    //         .execute(&conn)?;

        
    //     Ok(())
    // }

    pub fn for_view(t_conn: &PgConnection, t_id: i32) -> Vec<Post> {
        query!(many: t_conn, posts::view_id.eq(t_id), posts::created.asc())
    }

}