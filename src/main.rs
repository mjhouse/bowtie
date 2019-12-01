#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate bowtie_routes;

use rocket_contrib::{
    serve::StaticFiles,
    templates::Template
};

use dotenv::dotenv;
use bowtie_routes::public;
use bowtie_routes::profile;
use bowtie_routes::auth;

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
            
            // authentication routes
            auth::login_get, 
            auth::login_post, 
            auth::logout,
            auth::register_get, 
            auth::register_post, 
            auth::unregister,
            
            // profile routes
            profile::main,
            profile::feed,
            profile::write,
            profile::write_post,
            profile::delete
        ])
        .mount("/css",  StaticFiles::from(STATIC_CSS ))
        .mount("/js",   StaticFiles::from(STATIC_JS  ))
        .mount("/img",  StaticFiles::from(STATIC_IMG ))
        .mount("/font", StaticFiles::from(STATIC_FONT))
        .launch();
}
