pub use bowtie_data::{schema::*,traits::*};
use crate::view::*;
use crate::post::*;
use crate::error::*;

use diesel::prelude::*;
use rocket::{
    http::{Cookies,Cookie}
};

use bowtie_data::schema::users::dsl::users as users_dsl;
use bowtie_data::schema::views::dsl::views as views_dsl;
use bowtie_data::schema::posts::dsl::posts as posts_dsl;

use serde::{Serialize, Deserialize};
use whirlpool::{Whirlpool, Digest};
use base64::encode;
use std::env;

use rocket::{
    request::{FromRequest,Outcome,Request},
    http::{Method}
};

use medallion::{
    Header,
    Payload,
    Token,
};

use diesel::ConnectionError as ConnectionError;
use diesel::result::Error as DieselError;
use failure::*;

const SERVER_KEY: &[u8;10] = b"secret_key";
const ISSUER:  &str = "bowtie.com";
const SUBJECT: &str = "user";

macro_rules! logs {
    ( $s:expr ) => { |e| { error!("{}",e); Err($s)? } }
}

macro_rules! hash {
    ( $s:expr ) => { Whirlpool::new().chain(&$s).result(); }
}

// generate an insertion and query struct (User/UserModel),
// From implementations and basic helper macros/methods.
model!(
    table:  users,
    traits: [Identifiable,Default,AsChangeset],
    User {
        email:    Option<String>,
        username: String,
        passhash: String,
        view:     Option<i32>
});

impl_for!( User,
    id:i32        => users::id,
    email:&str    => users::email,
    username:&str => users::username,
    view:i32      => users::view
);

impl_set!( User,
    email:&str    => users::email,
    view:i32      => users::view
);

#[derive(Default,Debug, Serialize, Deserialize, PartialEq)]
pub struct UserClaims {
    pub id:       i32,
    pub email:    String,
    pub username: String,
    pub view:     Option<i32>
}

#[derive(FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String
}

#[derive(FromForm)]
pub struct RegisterForm {
    pub username:  String,
    pub password1: String,
    pub password2: String
}

#[derive(Debug)]
pub enum TokenError {
    FailedToSign,
    FailedToParse,
    TokenNotVerified
}

impl User {
    pub const COOKIE_NAME: &'static str = "bowtie_session_token";

    // @todo Make macro to generate common model create/delete functions
    // @body `create_from`, `create` and `delete` are common to all models

    pub fn create_from(t_name: &str, t_password: &str) -> Result<User,Error> {
        User::create(User {
            id:       None,
            view:     None,
            email:    None,
            username: t_name.into(),
            passhash: encode(&hash!(t_password))
        })
    }

    pub fn create(t_user: User) -> Result<User,Error> {
        let conn = db!(Err(BowtieError::NoConnection)?);

        conn.transaction::<_, Error, _>(|| {
            // create model
            let mut model: UserModel = 
                diesel::insert_into(users::table)
                .values(&t_user)
                .get_result(&conn)?;

            // create default view
            let view = View {
                id: None,
                user_id: model.id,
                name: model.username.clone()
            };
        
            let vmodel: ViewModel = 
            diesel::insert_into(views::table)
                .values(&view)
                .get_result(&conn)?;

            // update view id in user record
            model = diesel::update(users::table)
                .filter(users::id.eq(model.id))
                .set(users::view.eq(vmodel.id))
                .get_result(&conn)?;

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

    pub fn posts( &self ) -> Vec<Post> {
        let conn = db!(vec![]);

        let id = match self.view {
            Some(id) => id,
            _ => return vec![]
        };

        match posts::table
            .filter(posts::view_id.eq(id))
            .load::<PostModel>(&conn) {
                Ok(p)  => p.into_iter()
                           .map(|m| m.into())
                           .collect(),
                Err(_) => vec![]
            }
    }

    pub fn validate( &self, t_password:&str ) -> bool {
        let hash = encode(&hash!(t_password));
        self.passhash == hash
    }

    pub fn to_cookie( &self, t_cookies: &mut Cookies ) -> Result<Cookie,Error> {
        match self.to_token() {
            Ok(t) => {
                let cookie = Cookie::new(User::COOKIE_NAME,t);
                t_cookies.add(cookie.clone());
                Ok(cookie)
            },
            _ => Err(BowtieError::TokenCreationFailed)?
        }
    }

    pub fn from_cookie( t_cookies: &Cookies ) -> Result<User,Error> {
        match t_cookies.get(User::COOKIE_NAME) {
            Some(t) => User::from_token(t.value()),
            _ => Err(BowtieError::NoCookieFound)?
        }
    }

    pub fn to_token( &self ) -> Result<String,Error> {
        let header: Header<()> = Default::default();

        let payload = Payload {
            iss: Some(ISSUER.into()),
            sub: Some(SUBJECT.into()),
            claims: Some(self.to_claims()),
            ..Payload::default()
        };

        Token::new(header, payload)
            .sign(SERVER_KEY)
            .or_else(logs!(BowtieError::FailedToSign))
    }

    pub fn from_token( t_token:&str ) -> Result<User,Error> {
        Token::<(), UserClaims>::parse(t_token)
        .or_else(logs!(BowtieError::FailedToParse))
        .and_then(|t|{
            t.verify(SERVER_KEY)
            .or_else(logs!(BowtieError::TokenNotVerified))
            .and_then(|r|{
                match t.payload.claims {
                    Some(c) if r => Ok(User::from_claims(&c)),
                    _ => Err(BowtieError::TokenNotVerified)?
                }
            })
        })
    }

    pub fn to_claims( &self ) -> UserClaims {
        UserClaims {
            id:       self.id.unwrap_or(0).clone(),
            email:    self.email.as_ref().unwrap_or(&String::new()).clone(),
            username: self.username.clone(),
            view:     self.view
        }
    }

    pub fn from_claims( t_claims: &UserClaims ) -> User {
        User{
            id:       Some(t_claims.id.clone()),
            email:    Some(t_claims.email.clone()),
            username: t_claims.username.clone(),
            passhash: String::new(),
            view:     t_claims.view
        }
    }

}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<User,()> {
        match User::from_cookie(&request.cookies()){
            Ok(u)  => Outcome::Success(u),
            Err(_) => Outcome::Forward(())
        }
    }

}
