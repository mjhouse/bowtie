pub use bowtie_data::{schema::*,traits::*};

use diesel::prelude::*;
use rocket::{
    http::{Cookies,Cookie}
};

use bowtie_data::schema::users::dsl;
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
    ( $s:expr ) => { |e| { error!("{}",e); Err($s) } }
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

access!( User,
    id:i32        => users::id,
    email:&str    => users::email,
    username:&str => users::username,
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

    pub fn new(t_name: &str, t_password: &str) -> Self {
        User {
            id:       None,
            view:     None,
            email:    None,
            username: t_name.into(),
            passhash: encode(&hash!(t_password))
        }
    }

    pub fn create(t_user: User) -> Result<User,Error> {
        let uri  = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&uri)?;

        let model: UserModel = 
        diesel::insert_into(users::table)
            .values(&t_user)
            .get_result(&conn)?;

        Ok(model.into())
    }

    pub fn destroy(t_id:i32) -> Result<User,Error> {
        let uri  = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&uri)?;

        let model: UserModel = 
        diesel::delete(dsl::users.filter(users::id.eq(t_id)))
            .get_result(&conn)?;

        Ok(model.into())
    }

    pub fn update(t_user: &User) -> Result<User,Error> {
        let uri  = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&uri)?;

        let model: UserModel = 
        diesel::update(users::table)
            .set(t_user)
            .get_result(&conn)?;

        Ok(model.into())
    }

    pub fn validate( &self, t_password:&str ) -> bool {
        let given_hash = encode(&hash!(t_password));
        self.passhash == given_hash
    }











    pub fn to_cookie( &self, t_cookies: &mut Cookies ) {
        if let Ok(t) = self.to_token() {
            t_cookies.add(Cookie::new(User::COOKIE_NAME,t));
        }
    }

    pub fn from_cookie( t_cookies: &Cookies ) -> Option<Self> {
        t_cookies.get(User::COOKIE_NAME)
        .or(None)
        .and_then(|t|{ 
            User::from_token(t.value())
            .ok()
            .or(None)
            .and_then(|u| Some(u)) 
        })
    }

    pub fn to_token( &self ) -> Result<String,TokenError> {
        let header: Header<()> = Default::default();

        let payload = Payload {
            iss: Some(ISSUER.into()),
            sub: Some(SUBJECT.into()),
            claims: Some(self.to_claims()),
            ..Payload::default()
        };

        Token::new(header, payload)
            .sign(SERVER_KEY)
            .or_else(logs!(TokenError::FailedToSign))
    }

    pub fn from_token( t_token:&str ) -> Result<User,TokenError> {
        Token::<(), UserClaims>::parse(t_token)
        .or_else(logs!(TokenError::FailedToParse))
        .and_then(|t|{
            t.verify(SERVER_KEY)
            .or_else(logs!(TokenError::TokenNotVerified))
            .and_then(|r|{
                match t.payload.claims {
                    Some(c) if r => Ok(User::from_claims(&c)),
                    _ => Err(TokenError::TokenNotVerified)
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
            Some(u) => Outcome::Success(u),
            None => Outcome::Forward(())
        }
    }

}
