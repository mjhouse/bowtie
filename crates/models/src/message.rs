pub use bowtie_data::{schema::*,traits::*};
use bowtie_data::schema::messages::dsl::messages as messages_dsl;

use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Serialize};
use failure::*;
use std::env;

use crate::error::*;
use crate::view::*;

model!(
    table:  messages,
    traits: [Identifiable,Associations],
    Message {
        sender: i32,
        receiver: i32,
        body:    String,
        created: NaiveDateTime
});

impl_for!( Message,
    id:i32 => messages::id
);

impl Message {

    pub fn create_from(t_sender: i32, t_receiver: i32, t_body: String) -> Result<Message,Error> {
        Message::create(Message {
            id:       None,
            sender:   t_sender,
            receiver: t_receiver,
            body:     t_body,
            created:  Utc::now().naive_utc()
        })
    }

    pub fn delete_from(t_receiver: i32, t_id: i32) -> Result<Message,Error> {
        let conn = db!(Err(BowtieError::NoConnection)?);
        conn.transaction::<_, Error, _>(|| {

            let model: MessageModel = 
            diesel::delete(
                messages_dsl.filter(
                    messages::receiver.eq(t_receiver)
                    .and(messages::id.eq(t_id))
                ))
                .get_result(&conn)?;

            // return the deleted model
            Ok(model.into())
        })
    }

    pub fn create(t_message: Message) -> Result<Message,Error> {
        let conn = db!(Err(BowtieError::NoConnection)?);
        conn.transaction::<_, Error, _>(|| {
            // create model
            let model: MessageModel = 
                diesel::insert_into(messages::table)
                .values(&t_message)
                .get_result(&conn)?;

            Ok(model.into())
        })
    }

    pub fn delete(t_sender: View, t_receiver: View) -> Result<Message,Error> {
        match (t_sender.id,t_receiver.id) {
            (Some(sid),Some(rid)) => Message::delete_from(sid,rid),
            _ => Err(BowtieError::NoId)?
        }
    }

    pub fn messages(t_view: i32) -> Vec<Message> {
        let conn = db!(vec![]);

        match messages::table
            .filter(
                messages::sender.eq(t_view)
                .or(messages::receiver.eq(t_view))
            )
            .load::<MessageModel>(&conn) {
                Ok(p)  => p.into_iter()
                           .map(|m| m.into())
                           .collect(),
                Err(_) => vec![]
            }

    }

}