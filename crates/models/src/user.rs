pub use bowtie_data::{schema::*,traits::*};
use crate::view::*;
use crate::error::*;
use crate::session::*;

use diesel::prelude::*;

use bowtie_data::schema::users::dsl::users as users_dsl;
use bowtie_data::schema::views::dsl::views as views_dsl;
use bowtie_data::schema::posts::dsl::posts as posts_dsl;

use serde::{Serialize};
use whirlpool::{Whirlpool, Digest};
use base64::encode;
use std::env;

use rocket::{
    request::{FromRequest,Outcome,Request}
};

use failure::*;

macro_rules! hash {
    ( $s:expr ) => { Whirlpool::new().chain(&$s).result(); }
}

// Creates insertion and query structs (<Object>/<Object>Model),
model!(
    table:  users,
    traits: [Identifiable,Default,AsChangeset],
    User {
        email:    Option<String>,
        username: String,
        passhash: String
});

// Creates 'for_<field>' query functions.
queries!( 
    table: users,
    model: User,
    one: {
        id:i32        => users::id,
        username:&str => users::username
    }
);


impl User {
    pub const COOKIE_NAME: &'static str = "bowtie_session_token";

    // @todo Make macro to generate common model create/delete functions
    // @body `create_from`, `create` and `delete` are common to all models

    pub fn create_from(t_name: &str, t_password: &str) -> Result<User,Error> {
        User::create(User {
            id:       None,
            email:    None,
            username: t_name.into(),
            passhash: encode(&hash!(t_password))
        })
    }

    pub fn create(t_user: User) -> Result<User,Error> {
        let conn = db!(Err(BowtieError::NoConnection)?);

        conn.transaction::<_, Error, _>(|| {
            // create model
            let model: UserModel = 
                diesel::insert_into(users::table)
                .values(&t_user)
                .get_result(&conn)?;

            // create default view
            let view = View {
                id: None,
                user_id: model.id,
                name: model.username.clone()
            };
        
            diesel::insert_into(views::table)
                .values(&view)
                .execute(&conn)?;

            Ok(model.into())
        })
    }

    pub fn delete(t_user: User) -> Result<User,Error> {
        let conn = db!(Err(BowtieError::NoConnection)?);

        conn.transaction::<_, Error, _>(|| {
            let id = match t_user.id {
                Some(id) => id,
                _ => Err(BowtieError::NoId)?
            };

            // find all view models
            let ids = views::table
                .filter(views::user_id.eq(id))
                .select(views::id)
                .load::<i32>(&conn)?;

            // delete all posts associated with the user's views
            diesel::delete(
                posts_dsl.filter(
                    posts::view_id.eq_any(ids)))
                .execute(&conn)?;

            // delete all views associated with the user
            diesel::delete(
                views_dsl.filter(
                    views::user_id.eq(id)))
                .execute(&conn)?;

            // delete the user
            let model: UserModel = 
            diesel::delete(
                users_dsl.filter(
                    users::id.eq(id)))
                .get_result(&conn)?;

            // return the deleted user
            Ok(model.into())
        })
    }

    pub fn views( &self ) -> Vec<View> {
        let conn = db!(vec![]);

        let id = match self.id {
            Some(id) => id,
            _ => return vec![]
        };

        match views::table
            .filter(views::user_id.eq(id))
            .load::<ViewModel>(&conn) {
                Ok(v)  => v.into_iter()
                           .map(|m| m.into())
                           .collect(),
                Err(_) => vec![]
            }
    }

    pub fn validate( &self, t_password:&str ) -> bool {
        let hash = encode(&hash!(t_password));
        self.passhash == hash
    }

}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<User,()> {
        match Session::get(&request.cookies()){
            Ok(s)  => {
                match s.user() {
                    Ok(u) =>  Outcome::Success(u),
                    Err(_) => Outcome::Forward(()) 
                }
            },
            Err(_) => Outcome::Forward(())
        }
    }

}
