#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] 
extern crate rocket;
extern crate bowtie_routes;

use dotenv::dotenv;
use rocket_contrib::{
    serve::StaticFiles,
};

use bowtie_routes::resources::Resources;
use bowtie_routes::errors;
use bowtie_routes::public;
use bowtie_routes::profile;
use bowtie_routes::auth;

const RESOURCES:   &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources");
const STATIC_IMG:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/img");
const STATIC_FONT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/font");

fn main() {
    dotenv().ok();

    rocket::ignite()
        .manage(Resources::from(RESOURCES))
        .mount("/", routes![
            // public routes
            public::index, 
            public::about,
            public::search,
            public::users,
            public::posts,
            
            // authentication routes
            auth::login,
            auth::register, 
            auth::unregister,
            
            // auth::pages::login,
            // auth::pages::register,
            // auth::pages::unregister,

            auth::api::account::login,
            auth::api::account::logout,
            auth::api::account::register,
            // auth::api::account::unregister,

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
        .mount("/img",  StaticFiles::from(STATIC_IMG ))
        .mount("/font", StaticFiles::from(STATIC_FONT))
        .register(catchers![
            errors::handler_404
        ])
        .launch();
}
