pub use bowtie_data::{schema::*,traits::*};
use crate::user::User;
use crate::post::{Post,PostModel};
use crate::error::*;

use bowtie_data::schema::views::dsl::views as views_dsl;
use bowtie_data::schema::posts::dsl::posts as posts_dsl;

use diesel::prelude::*;
use serde::{Serialize};
use failure::*;
use std::env;

model!(
    table:  views,
    owner:  (User),
    traits: [Identifiable,Associations],
    View {
        user_id: i32,
        name: String
});

impl_for!( View,
    id:i32 => views::id,
    name:&str => views::name
);

impl View {

    // --------------------------------------------------------
    // Creation / Destruction
    pub fn create_from(t_user: i32, t_name: &str) -> Result<View,Error> {
        View::create(View {
            id: None,
            user_id: t_user,
            name: t_name.into()
        })
    }

    pub fn delete_from(t_user: i32, t_view: i32) -> Result<View,Error> {
        let conn = db!(Err(BowtieError::NoConnection)?);

        conn.transaction::<_, Error, _>(|| {
            // delete all posts associated with the view
            diesel::delete(
                posts_dsl.filter(
                    posts::view_id.eq(t_view)))
                .execute(&conn)?;

            // delete the view
            let model: ViewModel = 
            diesel::delete(
                views_dsl.filter(
                    views::user_id.eq(t_user)
                    .and(views::id.eq(t_view))
                ))
                .get_result(&conn)?;

            // return the deleted view
            Ok(model.into())
        })
    }

    pub fn create(t_view: View) -> Result<View,Error> {
        let conn = db!(Err(BowtieError::NoConnection)?);

        conn.transaction::<_, Error, _>(|| {
            // create model
            let model: ViewModel = 
                diesel::insert_into(views::table)
                .values(&t_view)
                .get_result(&conn)?;

            Ok(model.into())
        })
    }

    pub fn delete(t_view: View) -> Result<View,Error> {
        match (t_view.user_id,t_view.id) {
            (uid,Some(vid)) => View::delete_from(uid,vid),
            _ => Err(BowtieError::NoId)?
        }
    }
    // --------------------------------------------------------

    pub fn find_from(t_user: i32, t_view: i32) -> Result<View,Error> {
        let conn = db!(Err(BowtieError::NoConnection)?);

        conn.transaction::<_, Error, _>(|| {
            // find the view
            let model: ViewModel = 
            views::table
                .filter(
                    views::user_id.eq(t_user)
                    .and(views::id.eq(t_view))
                )
                .first::<ViewModel>(&conn)?;

            // return the deleted view
            Ok(model.into())
        })
    }

    pub fn posts( &self ) -> Vec<Post> {
        let conn = db!(vec![]);

        let id = match self.id {
            Some(i) => i,
            _ => return vec![]
        };

        match posts::table
            .filter(posts::view_id.eq(id))
            .load::<PostModel>(&conn) {
                Ok(p)  => p.into_iter()
                           .map(|m| m.into())
                           .collect(),
                Err(_) => vec![]
            }
    }

    pub fn for_user(t_id: i32) -> Vec<View> {
        let conn = db!(vec![]);
        query!(many: &conn, views::user_id.eq(t_id))
    }

    pub fn first(t_id: i32) -> Option<i32> {
        let conn = db!(None);
        let view: Option<View> = query!(one: &conn, views::user_id.eq(t_id));
        match view {
            Some(v) => v.id,
            None => None
        }
    }

}