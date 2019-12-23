use serde::{Serialize, Deserialize};
use medallion::{Header,Payload,Token,};
use failure::{Error};

use diesel::pg::PgConnection;

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
    pub view:     i32,
    pub views:    Vec<(i32,String)>,
    pub username: String
}

#[derive(Default,Debug, Serialize, Deserialize)]
pub struct Session {
    pub id:       Option<i32>,
    pub view:     i32,
    pub views:    Vec<(i32,String)>,
    pub username: String
}

impl Session {

    pub fn user( &self, t_conn: &PgConnection ) -> Result<User,Error> {
        match self.id {
            Some(id) => {
                match User::for_id(t_conn,id) {
                    Some(u) => Ok(u),
                    None => Err(BowtieError::RecordNotFound)?
                }
            },
            None => Err(BowtieError::NoId)? 
        }
    }

    pub fn create( t_conn: &PgConnection, t_user: &User, t_cookies: &mut Cookies ) -> Result<Session,Error> {
        let id = match t_user.id {
            Some(id) => id,
            _ => return Err(BowtieError::RecordNotFound)? 
        };
        
        // get all existing views as (id,name) pairs
        let views = View::for_user(t_conn,id.clone())
            .iter()
            .map(|v| (v.id.unwrap_or(-1),v.name.clone()))
            .collect::<Vec<(i32,String)>>();

        if views.len() > 0 {

            let (view,name) = match views.first() {
                Some((i,n)) => (i.clone(),n.clone()),
                _ => return Err(BowtieError::RecordNotFound)?
            };

            let session = Session {
                id:       Some(id),
                view:     view,
                views:    views,
                username: name
            };

            match session.set(t_cookies) {
                Ok(_)  => Ok(session),
                Err(e) => Err(e)
            }
        } else {
            Err(BowtieError::RecordNotFound)?
        }        
    }

    pub fn add_view( t_view:i32, t_name:String, cookies: &mut Cookies ) -> Result<Session,Error> {
        // parse the session out of the cookie
        match Session::get(cookies) {
            Ok(mut session) => {
                session.views.push((t_view,t_name));
                session.set(cookies)?;
                Ok(session)
            },
            Err(e) => Err(e)
        }
    }

    pub fn remove_view( t_view:i32, cookies: &mut Cookies ) -> Result<Session,Error> {
        // parse the session out of the cookie
        match Session::get(cookies) {
            Ok(mut session) => {
                session.views.retain(|v| v.0 != t_view);
                session.set(cookies)?;
                Ok(session)
            },
            Err(e) => Err(e)
        }
    }

    pub fn set_view( t_view:i32, cookies: &mut Cookies ) -> Result<Session,Error> {
        // parse the session out of the cookie
        match Session::get(cookies) {
            Ok(mut session) => {
                match session.views.iter().find(|x| x.0 == t_view) {
                    Some(v) => {
                        session.view = v.0;
                        session.username = v.1.clone();
                        session.set(cookies)?;
                        Ok(session)                        
                    },
                    None => Err(BowtieError::RecordNotFound)?
                }
            },
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

impl From<&SessionClaims> for Session {
    fn from(t_claims: &SessionClaims) -> Self {
        Session {
            id:       t_claims.id,
            view:     t_claims.view,
            views:    t_claims.views.clone(),
            username: t_claims.username.clone()
        }
    }
}

impl From<&Session> for SessionClaims {
    fn from(t_session: &Session) -> Self {
        SessionClaims {
            id:       t_session.id,
            view:     t_session.view,
            views:    t_session.views.clone(),
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