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
    http::{Cookies},
    request::{LenientForm},
    response::{Redirect}
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

#[get("/")]
fn index() -> Template {
    Template::render("index",Context::empty())
}

// -----------------------------------------
// Authentication
#[get("/login")]
fn login_get() -> Template {
    Template::render("login",Context::empty())
}

#[post("/login", data = "<form>")]
fn login_post( config: State<Config>, cookies:Cookies, form: LenientForm<LoginForm> ) -> Redirect {
    let conn = config.establish_connection().unwrap();
    match User::from_username(&conn,&form.username) {
        Some(u) if u.validate(&form.password) => {
            dbg!(&u);
            Redirect::to("/login")
        } 
        _ => Redirect::to("/login")
    }
}

#[get("/logout")]
fn logout() -> Redirect {
    Redirect::to("/login")
}

#[get("/register")]
fn register_get() -> Template {
    Template::render("register",Context::empty()) // CHANGE THIS
}

#[post("/register")]
fn register_post() -> Redirect {
    Redirect::to("/register")
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
