#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate log;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate medallion;
extern crate base64;

use dotenv::dotenv;
use std::env;

use rocket_contrib::{
    serve::StaticFiles,
    templates::Template
};

use rocket::{
    State,
    http::{Cookies},
    request::{Form},
    response::{Redirect}
};

mod schema;
mod models;
mod user;

use user::*;
use models::*;
use schema::users::dsl::*;
use diesel::prelude::*;

const STATIC_CSS:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/css");
const STATIC_JS:   &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/js");
const STATIC_IMG:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/img");
const STATIC_FONT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/font");

#[get("/")]
fn index() -> Template {
    Template::render("index",{})
}

#[get("/login")]
fn login() -> Template {
    Template::render("login",{})
}

#[get("/logout")]
fn logout() -> Redirect {
    Redirect::to("/login")
}

#[get("/profile")]
fn profile() -> Template {
    Template::render("profile",{})
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    dotenv().ok();

    let connection = establish_connection();

    //let new_user = User::create(&connection,"TESTY3","TEST","MCTEST");
    let all_user = User::all_slice(&connection,1,2);

    println!("Displaying {} users", all_user.len());
    for user in all_user {
        println!("{}: {}", user.username, user.email);
    }


    // rocket::ignite()
    //     .attach(Template::fairing())
    //     .mount("/", routes![
    //         index, login, logout, profile
    //     ])
    //     .mount("/css",  StaticFiles::from(STATIC_CSS ))
    //     .mount("/js",   StaticFiles::from(STATIC_JS  ))
    //     .mount("/img",  StaticFiles::from(STATIC_IMG ))
    //     .mount("/font", StaticFiles::from(STATIC_FONT))
    //     .launch();
}
