use rocket_contrib::{
    templates::Template
};

use rocket::{
    http::{Cookies,Cookie},
    request::{FlashMessage,Form},
    response::{Flash,Redirect}
};

use diesel::prelude::*;
use std::env;

use bowtie_models::user::*;
use bowtie_models::view::*;
use bowtie_models::post::*;
use bowtie_models::context::*;

#[get("/profile")]
pub fn main( _user: User ) -> Redirect {
    Redirect::to("/profile/feed")
}

#[get("/profile/feed")]
pub fn feed( user: User, msg: Option<FlashMessage> ) -> Template {
    // let posts = match db!() {
    //     Some(c) => user.posts(&c),
    //     _ => vec![]
    // };
    Template::render("profile/feed",Context {
        user:  Some(user),
        posts: vec![],
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[get("/profile/delete?<id>")]
pub fn delete( user: User, id: i32 ) -> Result<Redirect,Flash<Redirect>> {
    let conn = db_or!(flash!("/profile/feed","Database not availabe"));
    match (Post::for_id(&conn,id), user.id) {
        (Some(post),Some(uid)) if uid == post.view_id => {
            match post.delete(&conn) {
                Ok(_) => Ok(Redirect::to("/profile/feed")),
                _ => flash!("/profile/feed","Could not delete post")
            }
        },
        _ => {
            flash!("/profile/feed","No post with that id")
        }
    }
}

#[get("/profile/write")]
pub fn write( user: User, msg: Option<FlashMessage>  ) -> Template {
    Template::render("profile/write",Context {
        user: Some(user),
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[post("/profile/write", data = "<form>")]
pub fn write_post( user: User, form: Form<PostForm>  ) -> Result<Redirect,Flash<Redirect>> {
    let c = db_or!(flash!("/profile/write", "Server is unavailable"));

    // match Post::create(&c,&user,&form.title,&form.body) {
    //     Ok(_) => Ok(Redirect::to("/profile/feed")), 
    //     Err(_) => flash!("/profile/write", "Couldn't create post")
    // }
    flash!("/profile/write", "Couldn't create post")
}

// #[get("/profile/view?<view>")]
// pub fn view( mut user: User, view: i32 ) -> Result<Redirect,Flash<Redirect>> {
//     let c = db_or!(flash!("/profile", "Server is unavailable"));
//     match View::for_id(&c,view) {
//         Some(v) if Some(v.user_id) == user.id => {
//             // add to settings here
//             Ok(Redirect::to("/profile"))
//         },
//         _ => {
//             Ok(Redirect::to("/profile"))
//         }
//     }
// }

