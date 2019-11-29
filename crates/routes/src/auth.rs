use rocket_contrib::{
    templates::Template
};

use rocket::{
    http::{Cookies,Cookie},
    request::{FlashMessage,LenientForm},
    response::{Flash,Redirect}
};

use diesel::prelude::*;
use std::env;

use bowtie_models::user::*;
use bowtie_models::context::*;

const COOKIE_NAME: &str = "bowtie_session_token";

#[get("/login")]
pub fn login_get( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("auth/login",Context {
        user:  user,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[post("/login", data = "<form>")]
pub fn login_post( mut cookies:Cookies, form: LenientForm<LoginForm> ) -> Result<Redirect,Flash<Redirect>> {
    let c = db_or!(flash!("/login", "Server is unavailable"));

    let u = match User::from_username(&c,&form.username) {
        Some(u) if u.validate(&form.password) => u,
        _ => return flash!("/login", "Invalid username or password")
    };

    match u.to_token() {
        Ok(t) => {
            cookies.add(Cookie::new(COOKIE_NAME,t));
            Ok(Redirect::to("/profile"))
        },
        _ => flash!("/login", "There was a problem")
    }
}

#[get("/logout")]
pub fn logout(mut cookies:Cookies) -> Redirect {
    cookies.remove(Cookie::named(COOKIE_NAME));
    Redirect::to("/")
}

#[get("/register")]
pub fn register_get( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("auth/register",Context {
        user:  user,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[post("/register", data = "<form>")]
pub fn register_post( form: LenientForm<RegisterForm> ) -> Result<Redirect,Flash<Redirect>> {
    let c = db_or!(flash!("/register", "Server is unavailable"));

    if form.password1 != form.password2 {
        return flash!("/register", "Passwords don't match");
    }

    match User::create_from(&c,&form.username,&form.password1) {
        Ok(_) => Ok(Redirect::to("/login")), 
        _ => flash!("/register", "Username is taken")
    }
}

#[post("/unregister")]
pub fn unregister( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("auth/unregister",Context {
        user:  user,
        flash: unflash!(msg),
        ..Default::default()
    })
}

#[get("/recover")]
pub fn recover( user: Option<User>, msg: Option<FlashMessage> ) -> Template {
    Template::render("auth/recover",Context {
        user:  user,
        flash: unflash!(msg),
        ..Default::default()
    })
}