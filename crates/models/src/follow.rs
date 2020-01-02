pub use bowtie_data::schema::*;

use diesel::dsl;
use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

#[derive(Serialize, Queryable, Debug)]
pub struct Follow {
    pub id:         i32,
    pub subscriber: i32,
    pub publisher:  i32
}

#[derive(Identifiable,Insertable,Debug,Serialize)]
#[table_name="follows"]
pub struct FollowModel {
    pub id:         Option<i32>,
    pub subscriber: i32,
    pub publisher:  i32
}

impl Follow {

    pub fn create(
        t_conn: &PgConnection, 
        t_subscriber: i32, 
        t_publisher:  i32) -> Result<Follow,Error> 
    {
        let result = 
        diesel::insert_into(follows::table)
            .values(&FollowModel {
                id:      None,
                subscriber: t_subscriber,
                publisher:  t_publisher,
            })
            .get_result(t_conn)?;

        Ok(result)
    }

    pub fn delete(
        t_conn: &PgConnection, 
        t_subscriber: i32, 
        t_publisher:  i32) -> Result<Follow,Error> 
    {
        let result = 
        diesel::delete(follows::table)
        .filter(
            follows::subscriber.eq(t_subscriber)
            .and(follows::publisher.eq(t_publisher))
        )
        .get_result(t_conn)?;

        Ok(result)
    }    

    pub fn exists(
        t_conn: &PgConnection, 
        t_subscriber: i32, 
        t_publisher:  i32) -> bool 
    {
        diesel::select(dsl::exists(
            follows::table.filter(
                follows::subscriber.eq(t_subscriber)
                .and(follows::publisher.eq(t_publisher)))))
        .get_result(t_conn)
        .unwrap_or(false)
    }

}