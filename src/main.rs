#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rusqlite;

extern crate medallion;
extern crate base64;
extern crate sha3;

const STATIC_CSS:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/css");
const STATIC_JS:   &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/js");
const STATIC_IMG:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/img");
const STATIC_FONT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/font");

use rocket_contrib::{
    serve::StaticFiles,
    templates::Template
};

use serde::{
    Serialize,
    Deserialize
};

use rocket::{
    State,
    http::{Cookies},
    request::{Form},
    response::{Redirect}
};

mod user;
mod session;
use session::*;

#[derive(Serialize, Deserialize, Debug)]
struct Site {
    name: String,
}

#[get("/")]
fn index(site: State<Site>) -> Template {
    Template::render("index", &(*site))
}

#[get("/login")]
fn login_get(session: Session, site: State<Site>) -> Template {
    dbg!(session.user);

    Template::render("login", &(*site))
}

#[get("/profile")]
fn profile(session: Session, site: State<Site>) -> Template {
    Template::render("login", &(*site))
}

#[post("/login", data = "<login_form>")]
fn login_post(session: Session, mut cookies: Cookies, login_form: Form<LoginForm>) -> Redirect {
    if session.login(&mut cookies,&login_form).is_some() {
        Redirect::to("/")
    }
    else {
        Redirect::to("/login")
    }
}

#[get("/logout")]
fn logout(session: Session, mut cookies: Cookies) -> Redirect {
    session.logout(&mut cookies);
    Redirect::to("/")
}

fn main() {
    let site = Site {
        name: String::from("Michael House")
    };

    rocket::ignite()
        .attach(Template::fairing())
        .manage(site)
        .mount("/", routes![index,
            login_get,login_post,logout
        ])
        .mount("/css",  StaticFiles::from(STATIC_CSS ))
        .mount("/js",   StaticFiles::from(STATIC_JS  ))
        .mount("/img",  StaticFiles::from(STATIC_IMG ))
        .mount("/font", StaticFiles::from(STATIC_FONT))
        .launch();
}
