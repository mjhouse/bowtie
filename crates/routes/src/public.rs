use rocket_contrib::templates::Template;
use rocket::request::{FlashMessage,LenientForm};

use bowtie_models::user::*;
use bowtie_models::context::*;
use bowtie_models::search::*;

#[get("/")]
pub fn index( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("public/index",Context {
        user: user,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[get("/about")]
pub fn about( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("public/about",Context {
        user: user,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[get("/search?<query..>")]
pub fn search( user: Option<User>, msg: Option<FlashMessage>, query: LenientForm<SearchQuery> ) -> Template {
    Template::render("public/search",Context {
        user: user,
        flash: unflash!(msg),
        search: Search::from(&query),
        ..Default::default()
    })
}