#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate log;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate medallion;
extern crate base64;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use rocket_contrib::{
    serve::StaticFiles,
    templates::Template
};

use rocket::{
    State,
    http::{Cookies,Cookie},
    request::{FlashMessage,LenientForm},
    response::{Flash,Redirect}
};

mod schema;
mod models;
mod user;
mod config;
mod context;

use user::*;
use models::*;
use config::*;
use context::*;

const STATIC_CSS:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/css");
const STATIC_JS:   &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/js");
const STATIC_IMG:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/img");
const STATIC_FONT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/font");

const COOKIE_NAME: &str = "bowtie_session_token";

macro_rules! flash {
    ( $p:expr, $m:expr ) => { Err(Flash::error(Redirect::to($p), $m)) }
}

#[get("/")]
fn index() -> Template {
    Template::render("index",Context::empty())
}

// -----------------------------------------
// Authentication
#[get("/login")]
fn login_get( flash: Option<FlashMessage> ) -> Template {
    let msg = flash.map(|msg| Some(msg.msg().to_string()))
                   .unwrap_or_else(|| None);
        
    Template::render("login",Context::flash(msg))
}

#[post("/login", data = "<form>")]
fn login_post( config: State<Config>, mut cookies:Cookies, form: LenientForm<LoginForm> ) -> Result<Redirect,Flash<Redirect>> {
    if let Some(c) = config.establish_connection() {
        match User::from_username(&c,&form.username) {
            Some(u) if u.validate(&form.password) => {
                match u.to_token() {
                    Ok(token) => {
                        cookies.add(Cookie::new(COOKIE_NAME,token));
                        Ok(Redirect::to("/profile"))
                    }
                    _ => flash!("/login", "There was a problem")
                }
            }
            _ => flash!("/login", "Invalid username or password")
        }
    }
    else {
        flash!("/login", "Server is unavailable")
    }
}

#[get("/logout")]
fn logout(mut cookies:Cookies) -> Redirect {
    cookies.remove(Cookie::named(COOKIE_NAME));
    Redirect::to("/")
}

#[get("/register")]
fn register_get( flash: Option<FlashMessage> ) -> Template {
    let msg = flash.map(|msg| Some(msg.msg().to_string()))
                   .unwrap_or_else(|| None);
        
    Template::render("register",Context::flash(msg))
}

#[post("/register", data = "<form>")]
fn register_post( config: State<Config>, mut cookies:Cookies, form: LenientForm<RegisterForm> ) -> Result<Redirect,Flash<Redirect>> {
    if let Some(c) = config.establish_connection() {
        // match User::from_username(&c,&form.username) {
        //     Some(u) if u.login(&form.password) => {
        //         match u.to_token() {
        //             Ok(token) => {
        //                 cookies.add(Cookie::new(COOKIE_NAME,token));
        //                 Ok(Redirect::to("/profile"))
        //             }
        //             _ => Err(Flash::error(Redirect::to("/login"), "There was a problem"))
        //         }
        //     }
        //     _ => Err(Flash::error(Redirect::to("/login"), "Invalid username or password"))
        // }
        Err(Flash::error(Redirect::to("/register"), "Server is unavailable"))
    }
    else {
        flash!("/register", "Server is unavailable")
    }
}

#[post("/unregister")]
fn unregister() -> Template {
    Template::render("unregister",Context::empty())
}

#[get("/recover")]
fn recover() -> Template {
    Template::render("recover",Context::empty())
}

// -----------------------------------------



#[get("/profile")]
fn profile( user: User ) -> Template {
    Template::render("profile",{})
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(Template::fairing())
        .manage(Config::new())
        .mount("/", routes![
            index, 
            login_get, login_post, logout,
            register_get, register_post, unregister, 
            profile
        ])
        .mount("/css",  StaticFiles::from(STATIC_CSS ))
        .mount("/js",   StaticFiles::from(STATIC_JS  ))
        .mount("/img",  StaticFiles::from(STATIC_IMG ))
        .mount("/font", StaticFiles::from(STATIC_FONT))
        .launch();
}
