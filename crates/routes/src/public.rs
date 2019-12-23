use rocket::{
    State,
    request::{LenientForm}
};

use tera::{Context};

use crate::resources::*;
use bowtie_models::view::*;
use bowtie_models::post::*;
use bowtie_models::search::*;

#[get("/")]
pub fn index( resources: State<Resources> ) -> Page {
    resources.page("/public/index",false)
}

#[get("/about")]
pub fn about( resources: State<Resources> ) -> Page {
    resources.page("/public/about",false)
}

#[get("/search?<query..>")]
pub fn search( resources: State<Resources>, query: LenientForm<SearchQuery> ) -> Page {
    resources.page("/public/search",false)
        .with_context(context!(
            "search" => Search::from(&query)))
}

#[get("/users/<name>")]
pub fn users( resources: State<Resources>, name: String ) -> Page {
    let (posts,view) = match View::for_name(&name) {
        Some(v) => (v.posts(),Some(v)),
        None    => (vec![],None)
    };

    resources.page("/public/user",false)
        .with_context(context!(
            "posts" => posts,
            "view"  => view))
}

#[get("/posts/<id>")]
pub fn posts( resources: State<Resources>, id: i32 ) -> Page {
    resources.page("/public/post",false)
        .with_context(context!(
            "post" => Post::for_id(id)))
}