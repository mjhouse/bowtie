use std::default::Default;
use log::{info, trace, warn, error};
use serde::{Serialize, Deserialize};

use rocket::{
    request::{FromRequest,Outcome,Request},
    http::{Method,Cookies,Cookie}
};

use medallion::{
    Header,
    Payload,
    Token,
};

use crate::user::*;

const COOKIE_NAME: &str    = "bowtie_session_token";
const SERVER_KEY: &[u8;10] = b"secret_key";

#[derive(FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub submit:   String
}

#[derive(FromForm)]
pub struct RegisterForm {
    pub username:  String,
    pub password1: String,
    pub password2: String,
    pub submit:    String
}

#[derive(Default,Debug, Serialize, Deserialize, PartialEq)]
struct SessionClaims {
    id: i64,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub user: Option<User>,
}

impl Session {

    pub fn from( t_cookies:&Cookies ) -> Self {
        Self {
            user: Session::user_from_cookie(t_cookies)
        }
    }

    fn user_from_cookie( t_cookies:&Cookies ) -> Option<User> {
        let mut user = None;
        if let Some(cookie) = t_cookies.get(COOKIE_NAME){
            if let Ok(token) = Token::<(), SessionClaims>::parse(cookie.value()) {
                if token.verify(SERVER_KEY).unwrap_or(false){
                    let claims = token.payload.claims.unwrap();
                    let id = claims.id;
                    let username = claims.username;
                    user = Some ( User{
                        id: id,
                        username: username,
                        passhash: String::new(),
                    });
                }
            }
        }
        user
    }

    pub fn register( &self, t_form:&RegisterForm ) -> Option<User> {
        let pass1 = t_form.password1.trim();
        let pass2 = t_form.password2.trim();
        if pass1 != pass2 { return None; }
        User::create(&t_form.username,&pass1).ok()
    }

    pub fn login( &self, t_cookies:&mut Cookies, t_form:&LoginForm ) -> Option<User> {

        // find the user in the database
        if let Some(user) = User::from_username(&t_form.username).ok() {
            // compare the users password to the given password
            if user.validate(&t_form.password) {
                // if the passwords match, create a JWT token
                let header: Header<()> = Default::default();

                let id = user.id.clone();
                let un = user.username.clone();

                let payload = Payload {
                    iss: Some("bowtie.com".into()),
                    sub: Some("user".into()),
                    claims: Some(SessionClaims {
                        id: id.into(),
                        username: un.into(),
                        ..SessionClaims::default()
                    }),
                    ..Payload::default()
                };

                // insert the token into the users cookies
                let token = Token::new(header, payload);
                if let Ok(jwt) = token.sign(SERVER_KEY) {
                    t_cookies.add(Cookie::new(COOKIE_NAME,jwt.clone()));
                    return Some(user);
                }
            }
        }
        None
    }

    pub fn logout( &self, t_cookies:&mut Cookies ) {
        if self.user.is_some() {
            t_cookies.remove(Cookie::named(COOKIE_NAME));
        }
    }

}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Session,()> {
        match request.method(){
            Method::Get | Method::Post => {
                Outcome::Success(Session::from(&request.cookies()))
            }
            _ => Outcome::Forward(())
        }
    }

}
