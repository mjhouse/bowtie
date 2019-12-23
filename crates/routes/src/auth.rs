use rocket::State;
use crate::resources::*;

#[get("/login")]
pub fn login( resources: State<Resources> ) -> Page {
    resources.page("/auth/login",false)
}

#[get("/register")]
pub fn register( resources: State<Resources> ) -> Page {
    resources.page("/auth/register",false)
}

#[get("/unregister")]
pub fn unregister( resources: State<Resources> ) -> Page {
    resources.page("/auth/unregister",false)
}

#[get("/recover")]
pub fn recover( resources: State<Resources> ) -> Page {
    resources.page("/auth/recover",false)
}

pub mod api {

    pub mod account {
        
        use rocket::{
            response::{Flash,Redirect},
            request::{Form},
            http::{Cookies,Cookie}
        };
    
        use bowtie_models::{
            session::{Session},
            user::{User}
        };
    
        type ApiResponse = Result<Redirect,Flash<Redirect>>;
    
        #[derive(FromForm)]
        pub struct LoginForm {
            pub username: String,
            pub password: String
        }

        #[derive(FromForm)]
        pub struct RegisterForm {
            pub username:  String,
            pub password1: String,
            pub password2: String
        }
    
        #[post("/api/v1/login?<redirect>", data = "<form>")]
        pub fn login( redirect:    Option<String>,
                      mut cookies: Cookies, 
                      form:        Form<LoginForm>) -> ApiResponse {
            let path = redirect.unwrap_or("/profile".to_string());
            match User::for_username(&form.username) {
                Some(user) if user.validate(&form.password) => {
                    match Session::create(&user, &mut cookies) {
                        Ok(_)  => Ok(Redirect::to(path)),
                        Err(_) => flash!("/login", "Could not create session")
                    }
                },
                _ => flash!("/login", "Invalid username or password")
            }
        }

        // @todo Make it possible to remove cookie from '/api/v1/logout'
        #[get("/logout")]
        pub fn logout( mut cookies: Cookies ) -> ApiResponse {
            cookies.remove(Cookie::named(User::COOKIE_NAME));
            Ok(Redirect::to("/"))
        }

        #[post("/api/v1/register?<redirect>", data = "<form>")]
        pub fn register( redirect:    Option<String>, 
                         form:        Form<RegisterForm>) -> ApiResponse {
            let path = redirect.unwrap_or("/login".to_string());
            if form.password1 == form.password2 {
                match User::create_from(&form.username,&form.password1) {
                    Ok(_) => Ok(Redirect::to(path)), 
                    _ => flash!("/register", "Username is taken")
                }
            }
            else {
                flash!("/register", "Passwords don't match")
            }
        }

    }
}