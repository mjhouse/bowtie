pub use bowtie_data::{schema::*,traits::*};
use crate::user::User;

use diesel::prelude::*;
use serde::{Serialize};
use chrono::prelude::*;
use std::env;

use diesel::result::Error as DieselError;

model!(
    table:  views,
    owner:  (User),
    traits: [Identifiable,Associations],
    View {
        user_id: i32
});

access!( View,
    id:i32 => views::id,
    user_id:i32 => views::user_id
);

impl View {

    pub fn new() -> Self {
        Self {
            id: None,
            user_id: 0
        }
    }

    pub fn create(t_conn: &PgConnection, t_id: i32) -> Result<Self,DieselError> {
        let new_view = View {
            id:      None,
            user_id: t_id
        };
    
        diesel::insert_into(views::table)
            .values(&new_view)
            .get_result(t_conn)
            .or_else(|e|  Err(e))
            .and_then(|m: ViewModel| Ok(m.into()))
    }

    pub fn for_user(t_conn: &PgConnection, t_id: i32) -> Vec<View> {
        query!(many: t_conn, views::user_id.eq(t_id))
    }

}