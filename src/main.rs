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

macro_rules! unflash {
    ( $f:expr ) => { 
        $f.map(|msg| Some(msg.msg().to_string()))
          .unwrap_or_else(|| None)
    }
}

#[get("/")]
fn index( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("index",Context {
        user: user,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[get("/login")]
fn login_get( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("login",Context {
        user:  user,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[post("/login", data = "<form>")]
fn login_post( config: State<Config>, mut cookies:Cookies, form: LenientForm<LoginForm> ) -> Result<Redirect,Flash<Redirect>> {
    let c = match config.establish_connection() {
        Some(c) => c,
        _ => return flash!("/login", "Server is unavailable")
    };

    let u = match User::from_username(&c,&form.username) {
        Some(u) if u.validate(&form.password) => u,
        _ => return flash!("/login", "Invalid username or password")
    };

    match u.to_token() {
        Ok(t) => {
            cookies.add(Cookie::new(COOKIE_NAME,t));
            Ok(Redirect::to("/profile"))
        },
        _ => flash!("/login", "There was a problem")
    }
}

#[get("/logout")]
fn logout(mut cookies:Cookies) -> Redirect {
    cookies.remove(Cookie::named(COOKIE_NAME));
    Redirect::to("/")
}

#[get("/register")]
fn register_get( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("register",Context {
        user:  user,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[post("/register", data = "<form>")]
fn register_post( config: State<Config>, form: LenientForm<RegisterForm> ) -> Result<Redirect,Flash<Redirect>> {
    let c = match config.establish_connection() {
        Some(c) => c,
        _ => return flash!("/register", "Server is unavailable")
    };

    if form.password1 != form.password2 {
        return flash!("/register", "Passwords don't match");
    }

    match User::create_from(&c,&form.username,&form.password1) {
        Ok(_) => Ok(Redirect::to("/login")), 
        _ => flash!("/register", "Username is taken")
    }
}

#[post("/unregister")]
fn unregister( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("unregister",Context {
        user:  user,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[get("/recover")]
fn recover( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("recover",Context {
        user:  user,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[get("/profile")]
fn profile( user: User, msg: Option<FlashMessage>  ) -> Template {
    Template::render("profile",Context {
        user: Some(user),
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[get("/profile/post")]
fn profile_post( user: User, msg: Option<FlashMessage>  ) -> Template {
    Template::render("profile_post",Context {
        user: Some(user),
        flash: unflash!(msg),
        ..Default::default()
    })
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(Template::fairing())
        .manage(Config::new())
        .mount("/", routes![
            // public routes
            index, 
            // about,
            
            // authentication routes
            login_get, 
            login_post, 
            logout,
            register_get, 
            register_post, 
            unregister,
            
            // profile routes
            profile,
            profile_post
        ])
        .mount("/css",  StaticFiles::from(STATIC_CSS ))
        .mount("/js",   StaticFiles::from(STATIC_JS  ))
        .mount("/img",  StaticFiles::from(STATIC_IMG ))
        .mount("/font", StaticFiles::from(STATIC_FONT))
        .launch();
}
