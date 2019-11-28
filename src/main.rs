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
use bowtie_routes::validation;

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
            // about,
            
            // authentication routes
            validation::login_get, 
            validation::login_post, 
            validation::logout,
            validation::register_get, 
            validation::register_post, 
            validation::unregister,
            
            // profile routes
            profile::main,
            profile::wall,
            profile::write,
            profile::write_post
        ])
        .mount("/css",  StaticFiles::from(STATIC_CSS ))
        .mount("/js",   StaticFiles::from(STATIC_JS  ))
        .mount("/img",  StaticFiles::from(STATIC_IMG ))
        .mount("/font", StaticFiles::from(STATIC_FONT))
        .launch();
}
