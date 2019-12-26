use rocket::{
    State,
    request::{LenientForm}
};

use tera::{Context};

use crate::resources::*;
use bowtie_models::view::*;
use bowtie_models::post::*;
use bowtie_models::search::*;
use bowtie_models::comment::*;

use bowtie_data::Conn;

#[get("/")]
pub fn index( resources: State<Resources> ) -> Page {
    Page::render(&resources,"/public/index",false)
}

#[get("/about")]
pub fn about( resources: State<Resources> ) -> Page {
    Page::render(&resources,"/public/about",false)
}

#[get("/search?<query..>")]
pub fn search( conn: Conn, resources: State<Resources>, query: LenientForm<SearchQuery> ) -> Page {
    Page::render(&resources,"/public/search",false)
        .with_context(context!(
            "search" => Search::from(&conn,&query)))
}

#[get("/users/<name>")]
pub fn users( conn: Conn, resources: State<Resources>, name: String ) -> Page {
    let (posts,view) = match View::for_name(&conn,&name) {
        Some(v) => (v.posts(&conn),Some(v)),
        None    => (vec![],None)
    };

    Page::render(&resources,"/public/user",false)
        .with_context(context!(
            "posts" => posts,
            "view"  => view))
}

#[get("/posts/<id>")]
pub fn posts( conn: Conn, resources: State<Resources>, id: i32 ) -> Page {
    let comments = Comments::for_post(&conn,id);
    let posts = Post::for_id(&conn,id);
    Page::render(&resources,"/public/post",false)
        .with_context(context!(
            "post"      => posts,
            "comments" => comments
        ))
}
