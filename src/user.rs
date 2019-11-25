use crate::schema::users;
use diesel::prelude::*;
use crate::models::*;

use serde::{Serialize, Deserialize};
use whirlpool::{Whirlpool, Digest};
use base64::encode;

use rocket::{
    request::{FromRequest,Outcome,Request},
    http::{Method}
};

use medallion::{
    Header,
    Payload,
    Token,
};

use diesel::result::Error as DieselError;

const SERVER_KEY: &[u8;10] = b"secret_key";
const ISSUER:  &str = "bowtie.com";
const SUBJECT: &str = "user";
const COOKIE:  &str = "bowtie_session_token";

macro_rules! logs {
    ( $s:expr ) => { |e| { error!("{}",e); Err($s) } }
}

macro_rules! hash {
    ( $s:expr ) => { Whirlpool::new().chain(&$s).result(); }
}

macro_rules! query_by {
    ( $c:expr, $q:expr ) => {
        match users::table
            .filter($q)
            .first::<UserModel>($c)
        {
            Ok(u) => Some(u.into()),
            Err(e) => {
                warn!("Error during query: {}",e);
                None
            }
        }
    }
}

#[derive(Insertable,Debug,Serialize)]
#[table_name="users"]
pub struct User {
    pub email:    String,
    pub username: String,
    pub passhash: String
}

#[derive(Default,Debug, Serialize, Deserialize, PartialEq)]
pub struct UserClaims {
    pub email:    String,
    pub username: String
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
    
    pub fn create(t_conn: &PgConnection, t_email: &str, t_username: &str, t_passhash: &str) -> Result<User,DieselError> {
        let new_user = User {
            email:    t_email.into(),
            username: t_username.into(),
            passhash: t_passhash.into()
        };
    
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(t_conn)
            .or_else(|e|  Err(e))
            .and_then(|m: UserModel| Ok(m.into()))
    }

    pub fn from_email(t_conn: &PgConnection, t_email: &str) -> Option<User> {
        query_by!(t_conn,users::email.eq(t_email))
    }

    pub fn from_username(t_conn: &PgConnection, t_username: &str) -> Option<User> {
        query_by!(t_conn,users::username.eq(t_username))
    }

    pub fn from_passhash(t_conn: &PgConnection, t_passhash: &str) -> Option<User> {
        query_by!(t_conn,users::passhash.eq(t_passhash))
    }

    pub fn from_id(t_conn: &PgConnection, t_id: i32) -> Option<User> {
        query_by!(t_conn,users::id.eq(t_id))
    }

    pub fn validate( &self, t_password:&str ) -> bool {
        let given_hash = encode(&hash!(t_password));
        self.passhash == given_hash
    }

    pub fn all(t_conn: &PgConnection) -> Vec<User> {
        match users::table.load::<UserModel>(t_conn) {
            Ok(v)  => v.into_iter().map(|m| m.into()).collect(),
            Err(e) => {
                warn!("Error during query: {}",e);
                vec![]
            }
        }
    }

    pub fn all_slice(t_conn: &PgConnection, t_offset: i64, t_limit: i64) -> Vec<User> {            
        match users::table
            .offset(t_offset)
            .limit(t_limit)
            .load::<UserModel>(t_conn)
        {
            Ok(v)  => v.into_iter().map(|m| m.into()).collect(),
            Err(e) => {
                warn!("Error during query: {}",e);
                vec![]
            }
        }
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
            email: self.email.clone(),
            username: self.username.clone()
        }
    }

    pub fn from_claims( t_claims: &UserClaims ) -> User {
        User{
            email: t_claims.email.clone(),
            username: t_claims.username.clone(),
            passhash: String::new(),
        }
    }

}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<User,()> {
        match request.method(){
            Method::Get => {
                match request
                    .cookies()
                    .get(COOKIE)
                    .or(None)
                    .and_then(|t|{ 
                        User::from_token(t.value())
                        .ok()
                        .or(None)
                        .and_then(|u| Some(u)) 
                    })
                    {
                        Some(u) => Outcome::Success(u),
                        None => Outcome::Forward(())
                    }
            },
            _ => Outcome::Forward(())
        }
    }

}

impl From<UserModel> for User {
    fn from(model: UserModel) -> Self {
        User {
            email: model.email,
            username: model.username,
            passhash: model.passhash
        }
    }
}

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        UserModel {
            id: -1,
            email: user.email,
            username: user.username,
            passhash: user.passhash
        }
    }
}
