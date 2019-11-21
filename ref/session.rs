use std::default::Default;
use log::{info, trace, warn, error};
use serde::{Serialize, Deserialize};

use rocket::{
    request::{FromRequest,Outcome,Request},
    http::{Method,Cookies,Cookie}
};

use crate::user::*;

const COOKIE_NAME: &str    = "bowtie_session_token";

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
        // let user = t_cookies.get(COOKIE_NAME)
        //     .and_then(|t|{ User::from_token(t.value()).ok() });

        Self {
            user: None
        }
    }

    pub fn register( &self, t_form:&RegisterForm ) -> Option<User> {
        // let pass1 = t_form.password1.trim();
        // let pass2 = t_form.password2.trim();

        // if pass1 == pass2 {
        //     User::create(&t_form.username,&pass1).ok()
        // }
        // else {
        //     None
        // }
        None
    }

    pub fn login( &self, t_cookies:&mut Cookies, t_form:&LoginForm ) -> Option<User> {
        // match User::from_username(&t_form.username) {
        //     Ok(user) if user.validate(&t_form.password) => {
        //         match user.to_token() {
        //             Ok(token) => {
        //                 t_cookies.add(Cookie::new(COOKIE_NAME,token));
        //                 Some(user)
        //             }
        //             _ => None
        //         }
        //     }
        //     _ => None
        // }
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
