use rocket_contrib::templates::Template;
use rocket::request::{FromRequest,Outcome};
use rocket::Request;

use bowtie_models::context::*;
use bowtie_models::session::*;

macro_rules! session_from {
    ( $r:ident ) => {
        match Session::from_request($r) {
            Outcome::Success(s) => Some(s),
            _ => None
        }
    }
}

#[catch(404)]
pub fn handler_404(request: &Request) -> Template {
    Template::render("errors/404",Context {
        session: session_from!(request),
        ..Default::default()
    })
}

// @todo Add more error handlers
// @body Needs at least a 500 page.