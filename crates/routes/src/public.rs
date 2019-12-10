use rocket_contrib::templates::Template;
use rocket::request::{FlashMessage,LenientForm};
use rocket::response::{Redirect};
use diesel::prelude::*;
use std::env;

use bowtie_models::user::*;
use bowtie_models::post::*;
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

#[get("/users/<name>")]
pub fn users( user: Option<User>, msg: Option<FlashMessage>, name: String ) -> Template {
    let conn  = db!();
    let mut view  = None;
    let mut posts = vec![];

    if let Some(c) = conn {
        view  = User::for_username(&c,&name);
        
        if let Some(User { id:Some(id), ..}) = view {
            posts = Post::for_user(&c,id)
        }
    }

    Template::render("public/user",Context {
        user: user,
        view_user: view,
        posts: posts,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[get("/posts/<id>")]
pub fn posts( user: Option<User>, msg: Option<FlashMessage>, id: i32 ) -> Template {
    let viewing = db!().and_then(|c|{
            Post::for_id(&c,id)
        });

    Template::render("public/post",Context {
        user: user,
        view_post: viewing,
        flash: unflash!(msg),
        ..Default::default()
    })
}