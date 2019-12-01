use rocket_contrib::{
    templates::Template
};

use rocket::{
    request::{FlashMessage,Form},
    response::{Flash,Redirect}
};

use diesel::prelude::*;
use std::env;

use bowtie_models::user::*;
use bowtie_models::post::*;
use bowtie_models::context::*;

#[get("/profile")]
pub fn main( _user: User ) -> Redirect {
    Redirect::to("/profile/feed")
}

#[get("/profile/feed")]
pub fn feed( user: User, msg: Option<FlashMessage>  ) -> Template {
    let posts = match db!() {
        Some(c) => user.posts(&c),
        _ => vec![]
    };

    Template::render("profile/feed",Context {
        user:  Some(user),
        posts: posts,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[get("/profile/delete?<id>")]
pub fn delete( user: User, id: i32 ) -> Result<Redirect,Flash<Redirect>> {
    let conn = db_or!(flash!("/profile/feed","Database not availabe"));
    match (Post::from_id(&conn,id), user.id) {
        (Some(post),Some(uid)) if uid == post.user_id => {
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

    match Post::create(&c,&user,&form.title,&form.body) {
        Ok(_) => Ok(Redirect::to("/profile/feed")), 
        Err(_) => flash!("/profile/write", "Couldn't create post")
    }
}