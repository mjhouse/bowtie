pub use bowtie_data::{schema::*};

use bowtie_data::schema::friends::dsl::friends as friends_dsl;

use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

use crate::error::*;
use crate::view::*;

// Creates insertion and query structs for 'friends' table:
//      Friend/FriendModel
model!(
    table:  friends,
    traits: [Identifiable,Associations],
    Friend {
        sender:   i32,
        receiver: i32,
        accepted: bool
});

queries!( 
    table: friends,
    model: Friend,
    one: {
        id:i32 => friends::id,
        sender:i32 => friends::sender,
        receiver:i32 => friends::receiver
    }
);

impl Friend {

    pub fn create_from(t_conn: &PgConnection, t_sender: i32, t_receiver: i32, t_accepted: bool) -> Result<Friend,Error> {
        Friend::create(
            t_conn,
            Friend {
                id:       None,
                sender:   t_sender,
                receiver: t_receiver,
                accepted: t_accepted
            }
        )
    }

    pub fn delete_from(t_conn: &PgConnection, t_sender: i32, t_receiver: i32) -> Result<Friend,Error> {
        // delete the friend record
        let model: FriendModel = 
        diesel::delete(
            friends_dsl.filter(
                friends::sender.eq(t_sender)
                .and(friends::receiver.eq(t_receiver))
                .or(
                    friends::sender.eq(t_receiver)
                    .and(friends::receiver.eq(t_sender))
                )
            ))
            .get_result(t_conn)?;

        // return the deleted model
        Ok(model.into())
    }

    pub fn create(t_conn: &PgConnection, t_friend: Friend) -> Result<Friend,Error> {
        // create model
        let model: FriendModel = 
            diesel::insert_into(friends::table)
            .values(&t_friend)
            .get_result(t_conn)?;

        Ok(model.into())
    }

    pub fn delete(t_conn: &PgConnection, t_sender: View, t_receiver: View) -> Result<Friend,Error> {
        match (t_sender.id,t_receiver.id) {
            (Some(id1),Some(id2)) => Friend::delete_from(t_conn,id1,id2),
            _ => Err(BowtieError::NoId)?
        }
    }

    pub fn friends(t_conn: &PgConnection, t_view: i32) -> Vec<View> {
        match views::table
            .inner_join(
                friends::table
                .on(friends::sender.eq(views::id)
                .or(friends::receiver.eq(views::id)))
            )
            .filter(
                friends::sender.eq(t_view)
                .or(friends::receiver.eq(t_view))
                .and(friends::accepted.eq(true))
            )
            .filter(views::id.ne(t_view))
            .select((views::id,views::user_id,views::name))
            .load::<ViewModel>(t_conn) {
                Ok(p)  => p.into_iter()
                           .map(|m| m.into())
                           .collect(),
                Err(_) => vec![]
            }
    }

    pub fn requests(t_conn: &PgConnection, t_view: i32) -> Vec<View> {
        match views::table
            .inner_join(
                friends::table
                .on(friends::sender.eq(views::id)
                .and(friends::receiver.eq(t_view)
                .and(friends::accepted.eq(false))))
            )
            .select((views::id,views::user_id,views::name))
            .load::<ViewModel>(t_conn) {
                Ok(p)  => p.into_iter()
                           .map(|m| m.into())
                           .collect(),
                Err(_) => vec![]
            }
    }

    pub fn accept(t_conn: &PgConnection, t_sender: i32, t_receiver: i32) -> Result<Friend,Error> {
        t_conn.transaction::<_, Error, _>(|| {
            // update the request to set accepted
            let model: FriendModel = 
            diesel::update(friends::table)
                .filter(
                    friends::sender.eq(t_sender)
                    .and(friends::receiver.eq(t_receiver))
                )
                .set(friends::accepted.eq(true))
                .get_result(t_conn)?;
    
            Ok(model.into())
        })
    }


}