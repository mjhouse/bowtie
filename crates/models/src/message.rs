pub use bowtie_data::schema::*;

use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

use crate::View;

#[derive(Serialize, Queryable, Debug)]
pub struct Message {
    pub id:       i32,
    pub sender:   i32,
    pub receiver: i32,
    pub body:     String,
    pub created:  NaiveDateTime
}

#[derive(Identifiable,Insertable,Debug,Serialize)]
#[table_name="messages"]
pub struct MessageModel {
    pub id:       Option<i32>,
    pub sender:   i32,
    pub receiver: i32,
    pub body:     String,
    pub created:  NaiveDateTime
}

impl Message {

    pub fn create_from(
        t_conn:     &PgConnection, 
        t_sender:   i32, 
        t_receiver: i32, 
        t_body:     String) -> Result<Message,Error> {

        let result =
        diesel::insert_into(messages::table)
            .values(&MessageModel {
                id:       None,
                sender:   t_sender,
                receiver: t_receiver,
                body:     t_body,
                created:  Utc::now().naive_utc()
            })
            .get_result(t_conn)?;

        Ok(result)
    }

    pub fn delete_from(
        t_conn: &PgConnection, 
        t_receiver: i32, 
        t_id: i32) -> Result<Message,Error> {

        let result = 
        diesel::delete(messages::table)
            .filter(
                messages::receiver.eq(t_receiver)
                .and(messages::id.eq(t_id)))
            .get_result(t_conn)?;

        // return the deleted model
        Ok(result)
    }

    pub fn messages(t_conn: &PgConnection, t_view: i32) -> Vec<Message> {
        match messages::table
            .filter(
                messages::sender.eq(t_view)
                .or(messages::receiver.eq(t_view))
            )
            .load::<Message>(t_conn) {
                Ok(p)  => p,
                Err(e) => {
                    warn!("Error during query: {}",e);
                    vec![]
                }
            }
    }

    pub fn received(t_conn: &PgConnection, t_view: i32) -> Vec<(View,Message)> {
        match views::table
            .inner_join(
                messages::table
                .on(messages::sender.eq(views::id))
            )
            .filter(
                messages::receiver.eq(t_view)
            )
            .load::<(View,Message)>(t_conn) {
                Ok(p)  => p,
                Err(e) => {
                    warn!("Error during query: {}",e);
                    vec![]
                }
            }
    }

}