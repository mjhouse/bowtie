use bowtie_data::schema::comments::dsl::comments as comments_dsl;
pub use bowtie_data::schema::*;

use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

use crate::error::*;
use crate::view::*;

use diesel::sql_types::Integer;

model!(
    table:  comments,
    traits: [Identifiable,Associations],
    Comment {
        author:  i32,
        parent:  Option<i32>,
        body:    String,
        created: NaiveDateTime,
        post:    i32
});

#[derive(QueryableByName,Debug)]
#[table_name = "comments"]
pub struct MetaComment {
    id:      i32,
    author:  i32,
    parent:  Option<i32>,
    body:    String,
    created: NaiveDateTime,
    post:    i32
}

impl Comment {

    pub fn create(t_conn: &PgConnection, 
                  t_author: i32, 
                  t_post:   i32, 
                  t_parent: Option<i32>, 
                  t_body:   String ) -> Result<Comment,Error> 
    {
        let comment = Comment {
            id:      None,
            author:  t_author,
            parent:  t_parent,
            body:    t_body,
            created: Utc::now().naive_utc(),
            post:    t_post
        };

        t_conn.transaction::<_, Error, _>(|| {
            // create a comment with the given 
            // values.
            let model: CommentModel = diesel::insert_into(comments::table)
                .values(&comment).get_result(t_conn)?;

            let cid = model.id; 

            // insert a self-referencing comments_meta record.           
            diesel::sql_query(format!("
                INSERT INTO comments_meta(parent,child,depth) 
                VALUES      ({},{},0);",cid,cid))
            .execute(t_conn)?;

            // if there is a parent id, create a comments_meta
            // record for the relationship between the parent
            // and this child.
            if let Some(pid) = comment.parent {
                diesel::sql_query(format!("
                    INSERT INTO 
                    comments_meta(parent,child,depth)
                    SELECT {}, {}, p.depth+1
                    FROM   comments_meta p
                    WHERE  p.child = {}
                    ORDER BY 
                           p.depth DESC
                    LIMIT  1;
                ",pid,cid,pid))
                .execute(t_conn)?;
            }

            Ok(model.into())
        })
    }

    pub fn thread(t_conn: &PgConnection,t_id:i32) -> Vec<Comment> {
        match diesel::sql_query(format!("
            WITH RECURSIVE metas AS (
                SELECT 
                    parent, 
                    child, 
                    depth 
                FROM comments_meta WHERE 
                    parent = {}
                UNION ALL
                SELECT 
                    c.parent,
                    c.child,
                    c.depth 
                FROM comments_meta c 
                INNER JOIN metas m 
                ON m.child = c.parent
            )
            SELECT 
                DISTINCT c.* 
            FROM 
                comments c
            INNER JOIN metas 
            ON 
                c.id = metas.parent OR 
                c.id = metas.child
            ORDER BY 
                c.parent  ASC,
                c.created ASC;
        ",t_id))
        .load::<MetaComment>(t_conn) {
            Ok(c) => {
                c.into_iter().map(|m| {
                    Comment {
                        id:      Some(m.id),
                        author:  m.author,
                        parent:  m.parent,
                        body:    m.body,
                        created: m.created,
                        post:    m.post
                    }}).collect()
            },
            Err(e) => {
                dbg!(e);
                vec![]
            }
        }
    }





    // pub fn create_from(t_conn: &PgConnection, t_author: i32, t_post: i32, t_parent: Option<i32>, t_body: String) -> Result<Comment,Error> {
    //     Comment::create(
    //         t_conn,
    //         Comment {
    //             id:      None,
    //             author:  t_author,
    //             post:    t_post,
    //             parent:  t_parent,
    //             body:    t_body,
    //             created: Utc::now().naive_utc()
    //         }
    //     )
    // }

    // pub fn delete_from(t_conn: &PgConnection, t_author: i32, t_id: i32) -> Result<Comment,Error> {

    //     let dependents: i64 = 
    //         comments::table
    //             .filter(comments::parent.eq(t_id))
    //             .count()
    //             .first(t_conn).unwrap_or(0);

    //     let model: CommentModel = 
    //     if dependents > 0 {
    //         diesel::update(comments::table)
    //             .filter(
    //                 comments::author.eq(t_author)
    //                 .and(comments::id.eq(t_id))
    //             )
    //             .set(comments::body.eq("--"))
    //             .get_result(t_conn)?
    //     }
    //     else {
    //         diesel::delete(
    //             comments_dsl.filter(
    //                 comments::author.eq(t_author)
    //                 .and(comments::id.eq(t_id))
    //             ))
    //             .get_result(t_conn)?
    //     };

    //     // return the deleted model
    //     Ok(model.into())
    // }

    // pub fn create(t_conn: &PgConnection, t_comment: Comment) -> Result<Comment,Error> {
    //     // create model
    //     let model: CommentModel = 
    //         diesel::insert_into(comments::table)
    //         .values(&t_comment)
    //         .get_result(t_conn)?;

    //     Ok(model.into())
    // }

    // pub fn delete(t_conn: &PgConnection, t_author: View, t_id: i32) -> Result<Comment,Error> {
    //     match t_author.id {
    //         Some(id) => Comment::delete_from(t_conn,id,t_id),
    //         _ => Err(BowtieError::NoId)?
    //     }
    // }

    // pub fn for_post(t_conn: &PgConnection, t_post: i32) -> Vec<(View,Comment)> {
    //     match views::table
    //         .inner_join(
    //             comments::table
    //             .on(comments::author.eq(views::id))
    //         )
    //         .filter(comments::post.eq(t_post))
    //         .load::<(ViewModel,CommentModel)>(t_conn) {
    //             Ok(p)  => p.into_iter()
    //                         .map(|p| (p.0.into(),p.1.into()))
    //                         .collect(),
    //             Err(_) => vec![]
    //         }
    // }
}

// #[derive(Serialize,Debug)]
// pub struct Comments {
//     pub root:     Option<(View,Comment)>,
//     pub children: Vec<Box<Comments>>
// }

// impl Comments {

//     pub fn new() -> Self {
//         Self {
//             root:     None,
//             children: vec![]
//         }
//     }

//     pub fn from( t_root: Option<(View,Comment)> ) -> Self {
//         Self {
//             root:     t_root,
//             children: vec![]
//         }
//     }

//     pub fn for_post(t_conn: &PgConnection, t_post: i32) -> Comments {
//         let mut children = Comment::for_post(t_conn,t_post);
//         Comments::for_data(None,&mut children)
//     }

//     pub fn for_data(t_root: Option<(View,Comment)>, t_comments: &mut Vec<(View,Comment)>) -> Comments {
//         // get the root comment id and init a new Comments struct
//         let id = t_root.as_ref().and_then(|c| c.1.id.clone());
//         let mut comments = Comments::from(t_root);
        
//         // drain all the children of the current root
//         let mut children = vec![];
//         let mut i = 0;
//         while i < t_comments.len() {
//             if t_comments[i].1.parent == id {
//                 children.push(t_comments.remove(i));
//             } else {
//                 i += 1;
//             }
//         }

//         // for each child, create a 'Comments' struct
//         for child in children.into_iter() {
//             comments.children.push(Box::new(
//                 Comments::for_data(
//                     Some(child),
//                     t_comments
//                 )
//             ));
//         }

//         comments
//     }

// }

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

        let c1 = Comment::create(&db,
            63,
            34,
            Some(52),
            "Level 3".to_string()
        );
        
        assert!(c1.is_ok());
    }

    // #[test]
    // fn test_comments() {
    //     dotenv().ok();

    //     let url = std::env::var("DATABASE_URL")
    //         .expect("Environment variable: 'DATABASE_URL' not found");

    //     let db = PgConnection::establish(&url)
    //         .expect("Could not connect to database");

    //     let comments = Comment::thread(&db,34);
    //     dbg!(comments);
    //     assert!(true);
    // }

}