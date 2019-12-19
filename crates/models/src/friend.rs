pub use bowtie_data::{schema::*,traits::*};

use bowtie_data::schema::friends::dsl::friends as friends_dsl;

use diesel::prelude::*;
use serde::{Serialize};
use failure::*;
use std::env;

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

    pub fn create_from(t_view1: i32, t_view2: i32) -> Result<Friend,Error> {
        Friend::create(Friend {
            id: None,
            view1: t_view1,
            view2: t_view2
        })
    }

    pub fn delete_from(t_view1: i32, t_view2: i32) -> Result<Friend,Error> {
        let conn = db!(Err(BowtieError::NoConnection)?);
        conn.transaction::<_, Error, _>(|| {
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
                .get_result(&conn)?;

            // return the deleted model
            Ok(model.into())
        })
    }

    pub fn create(t_friend: Friend) -> Result<Friend,Error> {
        let conn = db!(Err(BowtieError::NoConnection)?);
        conn.transaction::<_, Error, _>(|| {
            // create model
            let model: FriendModel = 
                diesel::insert_into(friends::table)
                .values(&t_friend)
                .get_result(&conn)?;

            Ok(model.into())
        })
    }

    pub fn delete(t_view1: View, t_view2: View) -> Result<Friend,Error> {
        match (t_view1.id,t_view2.id) {
            (Some(id1),Some(id2)) => Friend::delete_from(id1,id2),
            _ => Err(BowtieError::NoId)?
        }
    }

    pub fn friends(t_view: i32) -> Vec<View> {
        let conn = db!(vec![]);

        match views::table
            .inner_join(
                friends::table
                .on(friends::view1.eq(t_view)
                .or(friends::view2.eq(t_view)))
            )
            .select((views::id,views::user_id,views::name))
            .filter(views::id.ne(t_view))
            .load::<ViewModel>(&conn) {
                Ok(p)  => p.into_iter()
                           .map(|m| m.into())
                           .collect(),
                Err(_) => vec![]
            }
    }

    pub fn request(t_sender: i32, t_receiver: i32) -> Result<FriendRequest,Error> {
        let conn = db!(Err(BowtieError::NoConnection)?);
        conn.transaction::<_, Error, _>(|| {
            // create model
            let model: FriendRequestModel = 
                diesel::insert_into(friend_requests::table)
                .values(FriendRequest {
                    id: None,
                    sender:   t_sender,
                    receiver: t_receiver,
                    accepted: false
                })
                .get_result(&conn)?;

            Ok(model.into())
        })
    }

}