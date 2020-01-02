pub use bowtie_data::schema::*;

use diesel::dsl;
use diesel::prelude::*;
use serde::{Serialize};
use failure::*;

use crate::View;

#[derive(Serialize, Queryable, Debug)]
pub struct Friend {
    pub id:       i32,
    pub sender:   i32,
    pub receiver: i32,
    pub accepted: bool
}

#[derive(Identifiable,Insertable,Debug,Serialize)]
#[table_name="friends"]
pub struct FriendModel {
    pub id:       Option<i32>,
    pub sender:   i32,
    pub receiver: i32,
    pub accepted: bool
}

impl Friend {

    pub fn create_from(
        t_conn:     &PgConnection, 
        t_sender:   i32, 
        t_receiver: i32, 
        t_accepted: bool) -> Result<Friend,Error> {

        let result = 
        diesel::insert_into(friends::table)
            .values(&FriendModel {
                id:       None,
                sender:   t_sender,
                receiver: t_receiver,
                accepted: t_accepted
            })
            .get_result(t_conn)?;

        Ok(result)
    }

    pub fn delete_from(
        t_conn:     &PgConnection, 
        t_sender:   i32, 
        t_receiver: i32) -> Result<Friend,Error> {
        // delete the friend record
        let result = 
        diesel::delete(friends::table)
            .filter(
                friends::sender.eq(t_sender)
                .and(friends::receiver.eq(t_receiver))
                .or(
                    friends::sender.eq(t_receiver)
                    .and(friends::receiver.eq(t_sender))
                ))
            .get_result(t_conn)?;

        // return the deleted model
        Ok(result)
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
            .load::<View>(t_conn) {
                Ok(p)  => p,
                Err(e) => {
                    warn!("Error during query: {}",e);
                    vec![]
                }
            }
    }

    pub fn requests(t_conn: &PgConnection, t_view: i32) -> Vec<(View,Friend)> {
        match views::table
            .inner_join(
                friends::table
                .on(friends::sender.eq(views::id)
                .or(friends::receiver.eq(views::id)))
            )
            .filter(
                friends::sender.eq(t_view)
                .or(friends::receiver.eq(t_view))
            )
            .filter(views::id.ne(t_view))
            .load::<(View,Friend)>(t_conn) {
                Ok(p)  => p,
                Err(e) => {
                    warn!("Error during query: {}",e);
                    vec![]
                }
            }
    }

    pub fn accept(
        t_conn:     &PgConnection, 
        t_sender:   i32, 
        t_receiver: i32) -> Result<Friend,Error> {

        let result =
        diesel::update(friends::table)
            .filter(
                friends::sender.eq(t_sender)
                .and(friends::receiver.eq(t_receiver))
            )
            .set(friends::accepted.eq(true))
            .get_result(t_conn)?;
        
        Ok(result)
    }

    pub fn exists(
        t_conn:   &PgConnection, 
        t_view:   i32, 
        t_friend: i32) -> bool {
        diesel::select(dsl::exists(
            friends::table.filter(
                friends::sender.eq(t_view)
                .and(friends::receiver.eq(t_friend))
                .or(
                    friends::sender.eq(t_friend)
                    .and(friends::receiver.eq(t_view)))
                .and(friends::accepted.eq(true))
        )))
        .get_result(t_conn)
        .unwrap_or(false)
    }

}