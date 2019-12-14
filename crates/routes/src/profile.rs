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
    let posts = match (db!(),user.view) {
        (Some(c),Some(id)) => Post::for_view(&c,id),
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
    let conn = db!(flash!("/profile/feed","Database not availabe"));
    match (Post::for_id(id), user.view) {
        (Some(post),Some(vid)) if vid == post.view_id => {
            match Post::delete(post) {
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
    match user.view {
        Some(id) => {
            match Post::create_from(id,&form.title,&form.body) {
                Ok(_)  => Ok(Redirect::to("/profile/feed")), 
                Err(_) => flash!("/profile/write", "Couldn't create post")
            }
        }
        _ => {
            flash!("/profile/write", "User does not have an active view")
        }
    }
}

#[get("/profile/settings")]
pub fn settings_get( user: User, msg: Option<FlashMessage>  ) -> Template {
    let views = user.views();
    Template::render("profile/settings",Context {
        user: Some(user),
        views: views,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[get("/profile/views?<current>&<delete>")]
pub fn views_get( mut user: User, current: Option<i32>, delete: Option<i32> ) -> Result<Redirect,Flash<Redirect>> {
    // if let Some(id) = current {
    //     if let Some(view) = View::for_id(id){
    //         if let Some(uid) = user.id {
    //             if view.user_id == uid {
    //                 user.view = view.id;
    //                 User::update(&user);
    //             }
    //         }
    //     }
    // }

    // if let Some(id) = delete {
    //     let view = View::for_id(id);
    //     dbg!(view);
    // }

    Ok(Redirect::to("/profile/settings"))
}

#[post("/profile/views")]
pub fn views_post( user: User ) -> Result<Redirect,Flash<Redirect>> {
    Ok(Redirect::to("/profile/settings"))
}
