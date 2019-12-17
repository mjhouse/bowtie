use serde::{Serialize, Deserialize};
use medallion::{Header,Payload,Token,};
use failure::{Error};

use rocket::{
    request::{FromRequest,Outcome,Request},
    http::{Cookies,Cookie}
};

use crate::{
    view::{View},
    user::{User},
    error::{BowtieError}
};

const COOKIE_NAME: &'static str = "bowtie_session_token";
const SERVER_KEY: &[u8;10] = b"secret_key";
const ISSUER:  &str = "bowtie.com";
const SUBJECT: &str = "user";

macro_rules! logs {
    ( $s:expr ) => { |e| { error!("{}",e); Err($s)? } }
}

#[derive(Default,Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionClaims {
    pub id:       Option<i32>,
    pub view:     Option<i32>,
    pub username: String
}

#[derive(Default,Debug, Serialize, Deserialize)]
pub struct Session {
    pub id:       Option<i32>,
    pub view:     Option<i32>,
    pub username: String
}

impl Session {

    pub fn user( &self ) -> Result<User,Error> {
        match self.id {
            Some(id) => {
                match User::for_id(id) {
                    Some(u) => Ok(u),
                    None => Err(BowtieError::RecordNotFound)?
                }
            },
            None => Err(BowtieError::NoId)? 
        }
    }

    pub fn update( t_user:i32, t_view: i32, cookies: &mut Cookies ) -> Result<Session,Error> {
        match View::find_from(t_user,t_view) {
            Ok(v) => {
                let session = Session {
                    id: Some(t_user),
                    view: Some(t_view),
                    username: v.name.clone(),
                };
                match session.set(cookies) {
                    Ok(_)  => Ok(session),
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn get( t_cookies: &Cookies ) -> Result<Session,Error> {
        match t_cookies.get(COOKIE_NAME) {
            Some(t) => Session::from_token(t.value()),
            _ => Err(BowtieError::NoCookieFound)?
        }
    }

    pub fn set( &self, t_cookies: &mut Cookies ) -> Result<(),Error> {
        match self.to_token() {
            Ok(t) => {
                t_cookies.add(
                    Cookie::build(COOKIE_NAME,t)
                    .path("/")
                    .finish());
                Ok(()) },
            Err(e) => Err(e)
        }
    }

    pub fn to_token( &self ) -> Result<String,Error> {
        let header: Header<()> = Default::default();

        let payload = Payload {
            iss: Some(ISSUER.into()),
            sub: Some(SUBJECT.into()),
            claims: Some(SessionClaims::from(self)),
            ..Payload::default()
        };

        Token::new(header, payload)
            .sign(SERVER_KEY)
            .or_else(logs!(BowtieError::FailedToSign))
    }

    pub fn from_token( t_token:&str ) -> Result<Session,Error> {
        Token::<(), SessionClaims>::parse(t_token)
        .or_else(logs!(BowtieError::FailedToParse))
        .and_then(|t|{
            t.verify(SERVER_KEY)
            .or_else(logs!(BowtieError::TokenNotVerified))
            .and_then(|r|{
                match t.payload.claims {
                    Some(c) if r => Ok(Session::from(&c)),
                    _ => Err(BowtieError::TokenNotVerified)?
                }
            })
        })
    }
}

impl From<&User> for Session {
    fn from(t_user: &User) -> Self {
        Session {
            id:       t_user.id,
            view:     None,
            username: t_user.username.clone()
        }
    }
}

impl From<&SessionClaims> for Session {
    fn from(t_claims: &SessionClaims) -> Self {
        Session {
            id:       t_claims.id,
            view:     t_claims.view,
            username: t_claims.username.clone()
        }
    }
}

impl From<&Session> for SessionClaims {
    fn from(t_session: &Session) -> Self {
        SessionClaims {
            id:       t_session.id,
            view:     t_session.view,
            username: t_session.username.clone()
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self,Self::Error> {
        match Session::get(&request.cookies()){
            Ok(u)  => Outcome::Success(u),
            Err(_) => Outcome::Forward(())
        }
    }

}