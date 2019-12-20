#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] 
extern crate rocket;
extern crate bowtie_routes;

use dotenv::dotenv;
use rocket_contrib::{
    serve::StaticFiles,
    templates::Template
};

use bowtie_routes::errors;
use bowtie_routes::public;
use bowtie_routes::profile;
use bowtie_routes::auth;
use bowtie_routes::styles;

const STATIC_SCSS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/scss");
const STATIC_CSS:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/css");
const STATIC_JS:   &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/js");
const STATIC_IMG:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/img");
const STATIC_FONT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/font");

fn main() {
    dotenv().ok();
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![
            // public routes
            public::index, 
            public::about,
            public::search,
            public::users,
            public::posts,
            
            // authentication routes
            auth::login_get, 
            auth::login_post, 
            auth::logout,
            auth::register_get, 
            auth::register_post, 
            auth::unregister,
            
            // profile routes
            profile::pages::main,
            profile::pages::feed,
            profile::pages::friends,
            profile::pages::messages,
            profile::pages::write,
            profile::pages::settings,

            profile::api::posts::create,
            profile::api::posts::delete,

            profile::api::views::create,
            profile::api::views::update,
            profile::api::views::delete,
        ])
        .mount("/css",  StaticFiles::from(STATIC_CSS ))
        .mount("/js",   StaticFiles::from(STATIC_JS  ))
        .mount("/img",  StaticFiles::from(STATIC_IMG ))
        .mount("/font", StaticFiles::from(STATIC_FONT))
        .register(catchers![
            errors::handler_404
        ])
        .manage(styles::Styles::from(STATIC_SCSS))
        .launch();
}
