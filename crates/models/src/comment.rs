pub use bowtie_data::schema::*;

use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

use crate::view::*;

#[derive(Serialize, Queryable, Debug)]
pub struct Comment {
    pub id:      i32,
    pub author:  i32,
    pub parent:  Option<i32>,
    pub body:    String,
    pub created: NaiveDateTime,
    pub post:    i32,
    pub path:    String
}

#[derive(Identifiable,Insertable,Debug,Serialize)]
#[table_name="comments"]
pub struct CommentModel {
    pub id:      Option<i32>,
    pub author:  i32,
    pub parent:  Option<i32>,
    pub body:    String,
    pub created: NaiveDateTime,
    pub post:    i32,
    pub path:    String
}

impl Comment {

    pub fn create(t_conn: &PgConnection, 
                  t_author: i32, 
                  t_post:   i32, 
                  t_parent: Option<i32>, 
                  t_body:   String ) -> Result<Comment,Error> 
    {
        t_conn.transaction::<_, Error, _>(|| {
            let mut path = vec![];

            if let Some(id) = t_parent {
                let (_,p) = Comment::for_id(t_conn,id)?;
                path = p.path
                        .split(",")
                        .map(String::from)
                        .collect::<Vec<String>>();
                path.push(id.to_string());
            }

            let result = diesel::insert_into(comments::table)
                .values(&CommentModel {
                    id:      None,
                    author:  t_author,
                    parent:  t_parent,
                    body:    t_body,
                    created: Utc::now().naive_utc(),
                    post:    t_post,
                    path:    path.join(",")
                })
                .get_result(t_conn)?;

            Ok(result)
        })
    }

    pub fn delete(t_conn: &PgConnection, t_view: i32, t_id: i32) -> Result<Comment,Error> {
        t_conn.transaction::<_, Error, _>(|| {
            let exists = diesel::select(
                diesel::dsl::exists(
                    comments::table.filter(
                        comments::parent.eq(t_id))))
                .get_result::<bool>(t_conn)?;
            
            let model: Comment = if !exists { 
                diesel::delete(comments::table)
                    .filter(
                        comments::id.eq(t_id)
                        .and(comments::author.eq(t_view)))
                    .get_result(t_conn)?
            }
            else {
                diesel::update(comments::table)
                    .filter(
                        comments::id.eq(t_id)
                        .and(comments::author.eq(t_view)))
                    .set(comments::body.eq(""))
                    .get_result(t_conn)?
            };

            Ok(model)
        })
    }    

    pub fn for_comment(t_conn: &PgConnection, t_id: i32, t_start: i64, t_count: i64) -> Vec<(View,Comment)> {
        match views::table
        .inner_join(
            comments::table
            .on(comments::author.eq(views::id))
        )
        .filter(comments::parent.eq(t_id))
        .order(comments::created.desc())
        .offset(t_start)
        .limit(t_count)
        .load::<(View,Comment)>(t_conn) {
            Ok(p)  => p,
            Err(e) => {
                warn!("Error during query: {}",e);
                vec![]
            }
        }
    }

    pub fn for_post(t_conn: &PgConnection, t_post: i32, t_start: i64, t_count: i64) -> Vec<(View,Comment)> {
        match views::table
        .inner_join(
            comments::table
            .on(comments::author.eq(views::id))
        )
        .filter(
            comments::post.eq(t_post)
            .and(comments::parent.is_null()))
        .order(comments::created.desc())
        .offset(t_start)
        .limit(t_count)
        .load::<(View,Comment)>(t_conn) {
            Ok(p)  => p,
            Err(e) => {
                warn!("Error during query: {}",e);
                vec![]
            }
        }
    }

    pub fn for_view(t_conn: &PgConnection, t_view: i32) -> Vec<Comment> {
        match comments::table
            .filter(comments::author.eq(t_view))
            .load::<Comment>(t_conn) {
                Ok(p)  => p,
                Err(e) => {
                    warn!("Error during query: {}",e);
                    vec![]
                }
            }        
    }

    pub fn for_id(t_conn: &PgConnection, t_id: i32) -> Result<(View,Comment),Error> {
        let result = views::table
            .inner_join(
                comments::table
                .on(comments::author.eq(views::id))
            )
            .filter(comments::id.eq(t_id))
            .get_result::<(View,Comment)>(t_conn)?;

        Ok(result)
    }

    pub fn get_path( &self ) -> Vec<i32> {
        self.path.split(",")
                 .filter(|s| !s.trim().is_empty())
                 .map(|s| s.parse::<i32>())
                 .filter(Result::is_ok)
                 .map(Result::unwrap)
                 .collect()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    use dotenv::dotenv;

    #[test]
    fn test_create() {
        dotenv().ok();

        let url = std::env::var("DATABASE_URL")
            .expect("Environment variable: 'DATABASE_URL' not found");

        let db = PgConnection::establish(&url)
            .expect("Could not connect to database");


    }

}