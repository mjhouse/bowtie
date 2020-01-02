pub use bowtie_data::schema::*;

use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

use crate::Post;

#[derive(Serialize, Queryable, Debug)]
pub struct View {
    pub id:      i32,
    pub user_id: i32,
    pub name:    String
}

#[derive(Identifiable,Insertable,Debug,Serialize)]
#[table_name="views"]
pub struct ViewModel {
    pub id:      Option<i32>,
    pub user_id: i32,
    pub name:    String
}

impl View {

    pub fn create_from(t_conn: &PgConnection, t_user: i32, t_name: &str) -> Result<View,Error> {
        View::create(
            t_conn,
            ViewModel {
                id: None,
                user_id: t_user,
                name: t_name.into()
            }
        )
    }

    pub fn delete_from(t_conn: &PgConnection, t_user: i32, t_view: i32) -> Result<View,Error> {
        t_conn.transaction::<_, Error, _>(|| {

            // delete all friend records for the view
            diesel::delete(friends::table)
                .filter(friends::sender.eq(t_view)
                        .or(friends::receiver.eq(t_view)))
                .execute(t_conn)?;


            // delete all posts associated with the view
            diesel::delete(posts::table)
                .filter(posts::view_id.eq(t_view))
                .execute(t_conn)?;

            // delete the view
            Ok(diesel::delete(views::table)
                .filter(views::user_id.eq(t_user)
                        .and(views::id.eq(t_view)))
                .get_result(t_conn)?)
        })
    }

    pub fn create(t_conn: &PgConnection, t_view: ViewModel) -> Result<View,Error> {
        Ok(diesel::insert_into(views::table)
            .values(&t_view)
            .get_result(t_conn)?)
    }

    pub fn delete(t_conn: &PgConnection, t_view: View) -> Result<View,Error> {
        View::delete_from(t_conn,t_view.user_id,t_view.id)
    }

    pub fn find_from(t_conn: &PgConnection, t_user: i32, t_view: i32) -> Result<View,Error> {
        t_conn.transaction::<_, Error, _>(|| {
            // find the view
            Ok(views::table
                .filter(views::user_id.eq(t_user)
                        .and(views::id.eq(t_view)))
                .first::<View>(t_conn)?)
        })
    }

    pub fn posts( &self, t_conn: &PgConnection ) -> Vec<Post> {
        match posts::table
            .filter(posts::view_id.eq(self.id))
            .load::<Post>(t_conn) {
                Ok(p)  => p,
                Err(_) => vec![]
            }
    }

    pub fn for_user(t_conn: &PgConnection, t_id: i32) -> Vec<View> {
        match views::table
            .filter(views::user_id.eq(t_id))
            .load::<View>(t_conn) {
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

    pub fn for_name(t_conn: &PgConnection, t_name: &str) -> Option<View> {
        views::table
            .filter(views::name.eq(t_name))
            .first::<View>(t_conn).ok()
    }

}