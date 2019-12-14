pub use bowtie_data::{schema::*,traits::*};
use crate::user::User;
use crate::error::*;

use bowtie_data::schema::views::dsl::views as views_dsl;
use bowtie_data::schema::posts::dsl::posts as posts_dsl;

use diesel::prelude::*;
use serde::{Serialize};
use chrono::prelude::*;
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

    pub fn create_from(t_user: i32, t_name:&str) -> Result<View,Error> {
        View::create(View {
            id: None,
            user_id: t_user,
            name: t_name.into()
        })
    }

    pub fn create(t_view: View) -> Result<View,Error> {
        let uri  = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&uri)?;

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
        let uri  = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&uri)?;

        conn.transaction::<_, Error, _>(|| {
            let id = match t_view.id {
                Some(id) => id,
                _ => Err(BowtieError::NoId)?
            };

            // delete all posts associated with the view
            diesel::delete(
                posts_dsl.filter(
                    posts::view_id.eq(id)))
                .execute(&conn)?;

            // delete the view
            let model: ViewModel = 
            diesel::delete(
                views_dsl.filter(
                    views::id.eq(id)))
                .get_result(&conn)?;

            // return the deleted view
            Ok(model.into())
        })
    }

    pub fn for_user(t_id: i32) -> Vec<View> {
        let conn = db!(vec![]);
        query!(many: &conn, views::user_id.eq(t_id))
    }

}