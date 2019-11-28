use rocket_contrib::templates::Template;
use rocket::request::FlashMessage;

use bowtie_models::user::*;
use bowtie_models::context::*;

#[get("/")]
pub fn index( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("index",Context {
        user: user,
        flash: unflash!(msg),
        ..Default::default()
    })
}