#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate bowtie_routes;
extern crate bowtie_data;

use dotenv::dotenv;
use rocket_contrib::{
    serve::StaticFiles,
};

use bowtie_routes::resources::Resources;
use bowtie_routes::errors;
use bowtie_routes::public;
use bowtie_routes::profile;
use bowtie_routes::auth;
use bowtie_data::Conn;

const RESOURCES:   &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources");
const STATIC_IMG:  &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/static/img");
const STATIC_FONT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/static/font");

fn main() {
    dotenv().ok();

    rocket::ignite()
        .manage(Resources::from(RESOURCES,false))
        .manage(Conn::initialize("DATABASE_URL"))
        .mount("/", routes![
            // public routes
            public::index, 
            public::about,
            public::search,
            public::users,
            public::posts,
            
            // authentication routes            
            auth::pages::login,
            auth::pages::register,
            auth::pages::unregister,
            auth::pages::recover,

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

            profile::api::comments::create,
            profile::api::comments::delete,

            profile::api::messages::create,

            profile::api::requests::create,
            profile::api::requests::delete,
            profile::api::requests::update,

            profile::api::posts::create,
            profile::api::posts::delete,

            profile::api::views::create,
            profile::api::views::update,
            profile::api::views::delete,
        ])
        .mount("/img",  StaticFiles::from(STATIC_IMG ))
        .mount("/font", StaticFiles::from(STATIC_FONT))
        .register(catchers![
            errors::handler_404,
            errors::handler_500
        ])
        .launch();
}
