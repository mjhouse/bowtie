use bowtie_data::schema::messages::dsl::messages as messages_dsl;
pub use bowtie_data::schema::*;

use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

use crate::error::*;
use crate::view::*;

model!(
    table:  messages,
    traits: [Identifiable,Associations],
    Message {
        sender:   i32,
        receiver: i32,
        body:     String,
        created:  NaiveDateTime
});



impl Message {

    pub fn create_from(t_conn: &PgConnection, t_sender: i32, t_receiver: i32, t_body: String) -> Result<Message,Error> {
        Message::create(
            t_conn,
            Message {
                id:       None,
                sender:   t_sender,
                receiver: t_receiver,
                body:     t_body,
                created:  Utc::now().naive_utc()
            }
        )
    }

    pub fn delete_from(t_conn: &PgConnection, t_receiver: i32, t_id: i32) -> Result<Message,Error> {
        let model: MessageModel = 
        diesel::delete(
            messages_dsl.filter(
                messages::receiver.eq(t_receiver)
                .and(messages::id.eq(t_id))
            ))
            .get_result(t_conn)?;

        // return the deleted model
        Ok(model.into())
    }

    pub fn create(t_conn: &PgConnection, t_message: Message) -> Result<Message,Error> {
        // create model
        let model: MessageModel = 
            diesel::insert_into(messages::table)
            .values(&t_message)
            .get_result(t_conn)?;

        Ok(model.into())
    }

    pub fn delete(t_conn: &PgConnection, t_sender: View, t_receiver: View) -> Result<Message,Error> {
        match (t_sender.id,t_receiver.id) {
            (Some(sid),Some(rid)) => Message::delete_from(t_conn,sid,rid),
            _ => Err(BowtieError::NoId)?
        }
    }

    pub fn messages(t_conn: &PgConnection, t_view: i32) -> Vec<Message> {
        match messages::table
            .filter(
                messages::sender.eq(t_view)
                .or(messages::receiver.eq(t_view))
            )
            .load::<MessageModel>(t_conn) {
                Ok(p)  => p.into_iter()
                           .map(|m| m.into())
                           .collect(),
                Err(_) => vec![]
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
            .load::<(ViewModel,MessageModel)>(t_conn) {
                Ok(p)  => p.into_iter()
                           .map(|p| (p.0.into(),p.1.into()))
                           .collect(),
                Err(_) => vec![]
            }
    }

}