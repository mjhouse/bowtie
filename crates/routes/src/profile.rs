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

macro_rules! database {
    ( $e:expr ) => {
        match env::var($e) {
            Ok(p) => {
                match PgConnection::establish(&p) {
                    Ok(c) => Some(c),
                    _ => None
                } 
            },
            _ => None
        }
    }
}

#[get("/profile")]
pub fn main( _user: User ) -> Redirect {
    Redirect::to("/profile/wall")
}

#[get("/profile/wall")]
pub fn wall( user: User, msg: Option<FlashMessage>  ) -> Template {
    Template::render("profile/wall",Context {
        user: Some(user),
        flash: unflash!(msg),
        ..Default::default()
    })
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
    let c = match database!("DATABASE_URL") {
        Some(c) => c,
        _ => return flash!("/profile/write", "Server is unavailable")
    };

    match Post::create(&c,&user,&form.title,&form.body) {
        Ok(_) => Ok(Redirect::to("/profile/wall")), 
        Err(_) => flash!("/profile/write", "Couldn't create post")
    }
}