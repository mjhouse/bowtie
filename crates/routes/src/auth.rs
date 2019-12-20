use rocket_contrib::{
    templates::Template
};

use rocket::{
    http::{Cookies,Cookie},
    request::{FlashMessage,LenientForm},
    response::{Flash,Redirect}
};

use bowtie_models::user::*;
use bowtie_models::context::*;
use bowtie_models::session::*;

use crate::forms::*;

type GetResponse  = Result<Template,Flash<Redirect>>;
type PostResponse = Result<Redirect,Flash<Redirect>>;

#[get("/login")]
pub fn login_get( session: Option<Session>, msg: Option<FlashMessage> ) -> GetResponse {
    Ok(Template::render("auth/login",Context {
        session:  session,
        flash:    unflash!(msg),
        ..Default::default()
    }))
}

#[post("/login", data = "<form>")]
pub fn login_post( mut cookies:Cookies, form: LenientForm<LoginForm> ) -> PostResponse {
    match User::for_username(&form.username) {
        Some(user) if user.validate(&form.password) => {
            match Session::create(&user, &mut cookies) {
                Ok(_)  => Ok(Redirect::to("/profile")),
                Err(_) => flash!("/login", "Could not create session")
            }
        },
        _ => flash!("/login", "Invalid username or password")
    }
}

#[get("/logout")]
pub fn logout(mut cookies:Cookies) -> PostResponse {
    cookies.remove(Cookie::named(User::COOKIE_NAME));
    Ok(Redirect::to("/"))
}

#[get("/register")]
pub fn register_get( session: Option<Session>, msg: Option<FlashMessage> ) -> GetResponse {
    Ok(Template::render("auth/register",Context {
        session: session,
        flash:   unflash!(msg),
        ..Default::default()
    }))
}

#[post("/register", data = "<form>")]
pub fn register_post( form: LenientForm<RegisterForm> ) -> PostResponse {
    match form.password1 == form.password2 {
        true => {
            match User::create_from(&form.username,&form.password1) {
                Ok(_) => Ok(Redirect::to("/login")), 
                _ => flash!("/register", "Username is taken")
            }
        }
        _ => flash!("/register", "Passwords don't match")
    }
}

#[post("/unregister")]
pub fn unregister( session: Option<Session>, msg: Option<FlashMessage> ) -> GetResponse {
    Ok(Template::render("auth/unregister",Context {
        session: session,
        flash:   unflash!(msg),
        ..Default::default()
    }))
}

#[get("/recover")]
pub fn recover( session: Option<Session>, msg: Option<FlashMessage> ) -> GetResponse {
    Ok(Template::render("auth/recover",Context {
        session: session,
        flash:   unflash!(msg),
        ..Default::default()
    }))
}