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
            public::get::index, 
            public::get::about,
            public::get::search,
            public::get::user,
            public::get::post,
            public::get::comment,
            
            // authentication routes            
            auth::get::login,
            auth::get::register,
            auth::get::unregister,
            auth::get::recover,

            auth::post::login,
            auth::post::logout,
            auth::post::register,
            // auth::api::account::unregister,

            // profile routes
            profile::get::main,
            profile::get::feed,
            profile::get::friends,
            profile::get::messages,
            profile::get::write,
            profile::get::settings,

            profile::post::follow::create,
            profile::post::follow::delete,
            profile::post::comment::create,
            profile::post::comment::delete,
            profile::post::message::create,
            profile::post::friend::create,
            profile::post::friend::delete,
            profile::post::friend::update,
            profile::post::post::create,
            profile::post::post::delete,
            profile::post::view::create,
            profile::post::view::update,
            profile::post::view::delete,
        ])
        .mount("/img",  StaticFiles::from(STATIC_IMG ))
        .mount("/font", StaticFiles::from(STATIC_FONT))
        .register(catchers![
            errors::handler_404,
            errors::handler_500
        ])
        .launch();
}
