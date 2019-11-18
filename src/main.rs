#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate log;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rusqlite;
extern crate simplelog;
extern crate medallion;
extern crate base64;

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

use simplelog::*;

mod user;
mod session;
use user::*;
use session::*;

const STATIC_CSS:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/css");
const STATIC_JS:   &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/js");
const STATIC_IMG:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/img");
const STATIC_FONT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/font");

#[derive(Serialize, Deserialize, Debug)]
struct Site {
    name: String,
}

#[get("/")]
fn index(site: State<Site>) -> Template {
    Template::render("index", &(*site))
}

#[get("/login")]
fn login_get(_session: Session, site: State<Site>) -> Template {
    Template::render("login", &(*site))
}

#[get("/profile")]
fn profile(session: Session) -> Result<Template,Redirect> {
    if session.user.is_some(){
        Ok(Template::render("profile", &session))
    }
    else {
        Err(Redirect::to("/login"))
    }
}

#[post("/login", data = "<login_form>")]
fn login_post(session: Session, mut cookies: Cookies, login_form: Form<LoginForm>) -> Redirect {
    if session.login(&mut cookies,&login_form).is_some() {
        Redirect::to("/profile")
    }
    else {
        Redirect::to("/login")
    }
}

#[post("/register", data = "<register_form>")]
fn register_post(session: Session, register_form: Form<RegisterForm>) -> Redirect {
    let result = session.register(&register_form);
    dbg!(result);

    Redirect::to("/login")
}

#[get("/logout")]
fn logout(session: Session, mut cookies: Cookies) -> Redirect {
    session.logout(&mut cookies);
    Redirect::to("/")
}

fn main() {

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap()
            // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    let site = Site {
        name: String::from("Michael House")
    };

    rocket::ignite()
        .attach(Template::fairing())
        .manage(site)
        .mount("/", routes![index,
            login_get, login_post, register_post,
            logout, profile
        ])
        .mount("/css",  StaticFiles::from(STATIC_CSS ))
        .mount("/js",   StaticFiles::from(STATIC_JS  ))
        .mount("/img",  StaticFiles::from(STATIC_IMG ))
        .mount("/font", StaticFiles::from(STATIC_FONT))
        .launch();
}
