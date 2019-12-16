use rocket_contrib::{
    templates::Template
};

use rocket::{
    request::{FlashMessage,Form},
    response::{Flash,Redirect},
    http::{Cookies}
};

use diesel::prelude::*;
use std::env;

use bowtie_models::user::*;
use bowtie_models::view::*;
use bowtie_models::post::*;
use bowtie_models::context::*;
use bowtie_models::session::*;

use crate::forms::*;

#[get("/profile")]
pub fn main( session: Session ) -> Redirect {
    Redirect::to("/profile/feed")
}

#[get("/profile/feed")]
pub fn feed( session: Session, msg: Option<FlashMessage> ) -> Template {
    let posts = match (db!(),session.view) {
        (Some(c),Some(id)) => Post::for_view(&c,id),
        _ => vec![]
    };

    Template::render("profile/feed",Context {
        session: Some(session),
        posts:   posts,
        flash:   unflash!(msg),
        ..Default::default()
    })
}

#[get("/profile/delete?<id>")]
pub fn delete( session: Session, id: i32 ) -> Result<Redirect,Flash<Redirect>> {
    match (Post::for_id(id), session.view) {
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
pub fn write( session: Session, msg: Option<FlashMessage>  ) -> Template {
    Template::render("profile/write",Context {
        session: Some(session),
        flash:   unflash!(msg),
        ..Default::default()
    })
}

#[post("/profile/write", data = "<form>")]
pub fn write_post( session: Session, form: Form<PostForm>  ) -> Result<Redirect,Flash<Redirect>> {
    match session.view {
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
pub fn settings_get( session: Session, msg: Option<FlashMessage>  ) -> Template {
    let views = match session.id {
        Some(id) => View::for_user(id),
        None => vec![]
    };

    Template::render("profile/settings",Context {
        session: Some(session),
        views:   views,
        flash:   unflash!(msg),
        ..Default::default()
    })
}

#[post("/profile/views", data = "<form>")]
pub fn views_post( mut session: Session, mut cookies: Cookies, form: Form<ViewForm> ) -> Result<Redirect,Flash<Redirect>> {
    match (Action::from(form),session.id,session.view) {
        (Action::Create(name),Some(uid),_) => {
            match View::create_from(uid,&name) {
                Ok(_)  => Ok(Redirect::to("/profile/settings")),
                _ => flash!("/profile/settings","Could not create view")
            }
        },
        (Action::Delete(vid),Some(uid),cv)  => {
            match View::for_id(vid) {
                // if view to delete: 
                //      is owned by the current user
                //      is not the active view
                Some(v) if v.user_id == uid && v.id != cv => {
                    match View::delete(v) {
                        Ok(_)  => Ok(Redirect::to("/profile/settings")),
                        _ => flash!("/profile/settings","Could not delete view")
                    }
                },
                _ => flash!("/profile/settings","Could not delete view")
            }
        },
        (Action::Active(vid),Some(uid),_)  => {
            match View::for_id(vid) {
                Some(v) if v.user_id == uid => {
                    session.view = v.id;
                    match session.set(&mut cookies) {
                        Ok(_) => Ok(Redirect::to("/profile/settings")),
                        _ => flash!("/profile/settings","Could not activate view")
                    }
                }
                _ => flash!("/profile/settings","Could not activate view")
            }
        },
        _ => flash!("/profile/settings","Couldn't understand request")
    }
}
