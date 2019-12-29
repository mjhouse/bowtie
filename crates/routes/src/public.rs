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

#[get("/user/<name>")]
pub fn user( conn: Conn, resources: State<Resources>, name: String ) -> Page {
    let (posts,view) = match View::for_name(&conn,&name) {
        Some(v) => (v.posts(&conn),Some(v)),
        None    => (vec![],None)
    };

    Page::render(&resources,"/public/user",false)
        .with_context(context!(
            "posts" => posts,
            "view"  => view))
}

#[get("/post/<id>")]
pub fn post( conn: Conn, resources: State<Resources>, id: i32 ) -> Page {
    let comments = Comment::for_post(&conn,id);
    let post     = Post::for_id(&conn,id);
    Page::render(&resources,"/public/post",false)
        .with_context(context!(
            "post"     => post,
            "comments" => comments
        ))
}

use rocket::request::FromFormValue;
use rocket::http::RawStr;
use serde::Serialize;

#[derive(Serialize,Debug)]
pub struct BreadCrumb {
    pub path: Vec<i32>
}

impl<'v> FromFormValue<'v> for BreadCrumb {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<BreadCrumb, &'v RawStr> {
        Ok(BreadCrumb {
            path: form_value.as_str()
                            .split(",")
                            .map(|v| v.parse::<i32>()
                                      .unwrap_or(-1))
                            .filter(|i| i > &0)
                            .collect()
        })
    }
}

#[get("/comment/<id>?<breadcrumb>")]
pub fn comment( conn: Conn, resources: State<Resources>, id: i32, breadcrumb: Option<BreadCrumb> ) -> Page {
    let comments   = Comment::for_comment(&conn,id);
    let submission = Comment::for_id(&conn,id).ok();
    let crumbs     = breadcrumb.as_ref()    
                               .map(|b| b.path.clone())
                               .unwrap_or(vec![]);
    Page::render(&resources,"/public/comment",false)
        .with_context(context!(
            "submission" => submission,
            "comments"   => comments,
            "crumbs"     => crumbs
        ))
}
