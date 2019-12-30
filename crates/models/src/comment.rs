pub use bowtie_data::schema::*;

use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

use crate::view::*;
use crate::error::*;

model!(
    table:  comments,
    traits: [Identifiable,Associations],
    Comment {
        author:  i32,
        parent:  Option<i32>,
        body:    String,
        created: NaiveDateTime,
        post:    i32,
        path:    String
});

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

            let model: CommentModel = 
            diesel::insert_into(comments::table)
                .values(&Comment {
                    id:      None,
                    author:  t_author,
                    parent:  t_parent,
                    body:    t_body,
                    created: Utc::now().naive_utc(),
                    post:    t_post,
                    path:    path.join(",")
                })
                .get_result(t_conn)?;

            Ok(model.into())
        })
    }

    pub fn delete(t_conn: &PgConnection, t_view: i32, t_id: i32) -> Result<Comment,Error> {
        let exists = diesel::select(
            diesel::dsl::exists(
                comments::table.filter(
                    comments::parent.eq(t_id))))
            .get_result::<bool>(t_conn)?;
        
        let model: CommentModel = if !exists { 
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

        Ok(model.into())
    }    

    pub fn for_comment(t_conn: &PgConnection, t_id: i32) -> Vec<(View,Comment)> {
        match views::table
        .inner_join(
            comments::table
            .on(comments::author.eq(views::id))
        )
        .filter(
            comments::parent.eq(t_id))
        .order(
            comments::created.desc()
        )
        .load::<(ViewModel,CommentModel)>(t_conn) {
            Ok(p)  => p.into_iter()
                        .map(|r| (r.0.into(),r.1.into()))
                        .collect(),
            Err(_) => vec![]
        }
    }

    pub fn for_post(t_conn: &PgConnection, t_post: i32) -> Vec<(View,Comment)> {
        match views::table
        .inner_join(
            comments::table
            .on(comments::author.eq(views::id))
        )
        .filter(
            comments::post.eq(t_post)
            .and(comments::parent.is_null()))
        .order(
            comments::created.desc()
        )
        .load::<(ViewModel,CommentModel)>(t_conn) {
            Ok(p)  => p.into_iter()
                        .map(|r| (r.0.into(),r.1.into()))
                        .collect(),
            Err(_) => vec![]
        }
    }

    pub fn for_view(t_conn: &PgConnection, t_view: i32) -> Vec<Comment> {
        match comments::table
            .filter(comments::author.eq(t_view))
            .load::<CommentModel>(t_conn) {
                Ok(p)  => p.into_iter()
                            .map(|p| p.into())
                            .collect(),
                Err(_) => vec![]
            }        
    }

    pub fn for_id(t_conn: &PgConnection, t_id: i32) -> Result<(View,Comment),Error> {
        match views::table
            .inner_join(
                comments::table
                .on(comments::author.eq(views::id))
            )
            .filter(comments::id.eq(t_id))
            .get_result::<(ViewModel,CommentModel)>(t_conn) {
                Ok(p)  => Ok((p.0.into(),p.1.into())),
                Err(_) => Err(BowtieError::RecordNotFound)?
            }      
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

        for i in 0..1000 {
            let c1 = Comment::create(&db,
                63,
                37,
                None,
                "Comment".to_string()
            );
            assert!(c1.is_ok());
        }
    }

}