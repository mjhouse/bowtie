pub use bowtie_data::{schema::*};

use bowtie_data::schema::friends::dsl::friends as friends_dsl;
use bowtie_data::schema::friend_requests::dsl::friend_requests as requests_dsl;

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
        view1: i32,
        view2: i32
});

queries!( 
    table: friends,
    model: Friend,
    one: {
        id:i32 => friends::id
    }
);

// Creates insertion and query structs for 'friend_requests' table:
//      FriendRequest/FriendRequestModel
model!(
    table:  friend_requests,
    traits: [Identifiable,Associations],
    FriendRequest {
        sender:   i32,
        receiver: i32,
        accepted: bool
});

queries!( 
    table: friend_requests,
    model: FriendRequest,
    one: {
        id:i32 => friend_requests::id
    }
);

impl Friend {

    pub fn create_from(t_conn: &PgConnection, t_view1: i32, t_view2: i32) -> Result<Friend,Error> {
        Friend::create(
            t_conn,
            Friend {
                id: None,
                view1: t_view1,
                view2: t_view2
            }
        )
    }

    pub fn delete_from(t_conn: &PgConnection, t_view1: i32, t_view2: i32) -> Result<Friend,Error> {
        // delete the friend record by searching for either
        // combination of a friend pair- view1/view2 or 
        // view2/view1
        let model: FriendModel = 
        diesel::delete(
            friends_dsl.filter(
                friends::view1.eq(t_view1)
                .and(friends::view2.eq(t_view2))
                .or(
                    friends::view1.eq(t_view2)
                    .and(friends::view2.eq(t_view1))
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

    pub fn delete(t_conn: &PgConnection, t_view1: View, t_view2: View) -> Result<Friend,Error> {
        match (t_view1.id,t_view2.id) {
            (Some(id1),Some(id2)) => Friend::delete_from(t_conn,id1,id2),
            _ => Err(BowtieError::NoId)?
        }
    }

    pub fn friends(t_conn: &PgConnection, t_view: i32) -> Vec<View> {
        match views::table
            .inner_join(
                friends::table
                .on(friends::view2.eq(views::id)
                .and(friends::view1.eq(t_view)))
            )
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
                friend_requests::table
                .on(friend_requests::receiver.eq(t_view)
                .and(friend_requests::sender.eq(views::id)
                .and(friend_requests::accepted.eq(false))))
            )
            .select((views::id,views::user_id,views::name))
            .load::<ViewModel>(t_conn) {
                Ok(p)  => p.into_iter()
                           .map(|m| m.into())
                           .collect(),
                Err(_) => vec![]
            }
    }

    pub fn request(t_conn: &PgConnection, t_sender: i32, t_receiver: i32) -> Result<FriendRequest,Error> {
        // create model
        let model: FriendRequestModel = 
            diesel::insert_into(friend_requests::table)
            .values(FriendRequest {
                id: None,
                sender:   t_sender,
                receiver: t_receiver,
                accepted: false
            })
            .get_result(t_conn)?;

        Ok(model.into())
    }

    pub fn accept(t_conn: &PgConnection, t_sender: i32, t_receiver: i32) -> Result<Friend,Error> {
        t_conn.transaction::<_, Error, _>(|| {
            // update the request to set accepted
            diesel::update(friend_requests::table)
                .set(friend_requests::accepted.eq(true))
                .execute(t_conn)?;

            // create friend record for both accounts
            let model: FriendModel = 
                diesel::insert_into(friends::table)
                .values(&vec![
                    Friend {
                        id: None,
                        view1: t_sender,
                        view2: t_receiver
                    },
                    Friend {
                        id: None,
                        view1: t_receiver,
                        view2: t_sender
                    }
                ])
                .get_result(t_conn)?;
    
            Ok(model.into())
        })
    }

    pub fn deny(t_conn: &PgConnection, t_sender: i32, t_receiver: i32) -> Result<FriendRequest,Error> {
        let model: FriendRequestModel = 
            diesel::delete(
                requests_dsl.filter(
                    friend_requests::sender.eq(t_sender)
                    .and(friend_requests::receiver.eq(t_receiver))
                ))
                .get_result(t_conn)?;

        Ok(model.into())
    }

}