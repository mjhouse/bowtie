use rocket_contrib::templates::Template;
use rocket::{
    State,
    request::{FlashMessage,LenientForm}
};

use crate::styles::*;

use bowtie_models::view::*;
use bowtie_models::post::*;
use bowtie_models::context::*;
use bowtie_models::session::*;
use bowtie_models::search::*;

#[get("/")]
pub fn index( styles: State<Styles>, session: Option<Session>, msg: Option<FlashMessage> ) -> Template {
    Template::render("public/index",Context {
        session: session,
        sheet:   styles.sheet("light","index"),
        flash:   unflash!(msg),
        ..Default::default()
    })
}

#[get("/about")]
pub fn about( session: Option<Session>, msg: Option<FlashMessage> ) -> Template {
    Template::render("public/about",Context {
        session: session,
        flash:   unflash!(msg),
        ..Default::default()
    })
}

#[get("/search?<query..>")]
pub fn search( session: Option<Session>, msg: Option<FlashMessage>, query: LenientForm<SearchQuery> ) -> Template {
    Template::render("public/search",Context {
        session: session,
        flash:   unflash!(msg),
        search:  Search::from(&query),
        ..Default::default()
    })
}

#[get("/users/<name>")]
pub fn users( session: Option<Session>, msg: Option<FlashMessage>, name: String ) -> Template {
    let (posts,view) = match View::for_name(&name) {
        Some(v) => (v.posts(),Some(v)),
        None    => (vec![],None)
    };

    Template::render("public/user",Context {
        session: session,
        view:    view,
        posts:   posts,
        flash:   unflash!(msg),
        ..Default::default()
    })
}

#[get("/posts/<id>")]
pub fn posts( session: Option<Session>, msg: Option<FlashMessage>, id: i32 ) -> Template {
    let post = Post::for_id(id);
    Template::render("public/post",Context {
        session: session,
        post:    post,
        flash:   unflash!(msg),
        ..Default::default()
    })
}