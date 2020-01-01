pub use bowtie_data::schema::*;
use diesel::dsl;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

use crate::view::*;
use crate::error::*;

model!(
    table:  follows,
    traits: [Identifiable,Associations],
    Follow {
        subscriber: i32,
        publisher:  i32
});

impl Follow {

    pub fn create(
        t_conn: &PgConnection, 
        t_subscriber: i32, 
        t_publisher:  i32) -> Result<Follow,Error> 
    {
        let model: FollowModel = 
        diesel::insert_into(follows::table)
            .values(&Follow {
                id:      None,
                subscriber: t_subscriber,
                publisher:  t_publisher,
            })
            .get_result(t_conn)?;

        Ok(model.into())
    }

    pub fn delete(
        t_conn: &PgConnection, 
        t_subscriber: i32, 
        t_publisher:  i32) -> Result<Follow,Error> 
    {
        let model: FollowModel =
        diesel::delete(follows::table)
        .filter(
            follows::subscriber.eq(t_subscriber)
            .and(follows::publisher.eq(t_publisher))
        )
        .get_result(t_conn)?;

        Ok(model.into())
    }    

    pub fn exists(
        t_conn: &PgConnection, 
        t_subscriber: i32, 
        t_publisher:  i32) -> bool 
    {
        diesel::select(dsl::exists(
            follows::table.filter(follows::subscriber.eq(t_subscriber)
            .and(follows::publisher.eq(t_publisher)))
        ))
        .get_result(t_conn).unwrap_or(false)
    }

}