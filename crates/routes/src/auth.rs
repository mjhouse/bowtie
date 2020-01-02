pub mod get {

    use rocket::State;
    use crate::resources::*;

    #[get("/login")]
    pub fn login( resources: State<Resources> ) -> Page {
        Page::render(&resources,"/auth/login",false)
    }
    
    #[get("/register")]
    pub fn register( resources: State<Resources> ) -> Page {
        Page::render(&resources,"/auth/register",false)
    }
    
    #[get("/unregister")]
    pub fn unregister( resources: State<Resources> ) -> Page {
        Page::render(&resources,"/auth/unregister",false)
    }
    
    #[get("/recover")]
    pub fn recover( resources: State<Resources> ) -> Page {
        Page::render(&resources,"/auth/recover",false)
    }
}

pub mod post {
        
    use rocket::{
        response::{Flash,Redirect},
        request::{Form},
        http::{Cookies}
    };

    use bowtie_models::*;
    use bowtie_data::Conn;

    type PostResponse = Result<Redirect,Flash<Redirect>>;

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

    #[post("/login?<redirect>", data = "<form>")]
    pub fn login( conn:        Conn, 
                    redirect:    Option<String>,
                    mut cookies: Cookies, 
                    form:        Form<LoginForm>) -> PostResponse {
        let path = redirect.unwrap_or("/profile".to_string());

        match User::for_name(&conn,&form.username) {
            Some(user) if user.validate(&form.password) => {
                match Session::create(&conn,&user, &mut cookies) {
                    Ok(_)  => Ok(Redirect::to(path)),
                    Err(e) => {
                        warn!("Could not create session: {}",e);
                        flash!("/login", "Could not create session")
                    }
                }
            },
            _ => flash!("/login", "Invalid username or password")
        }
    }

    // @todo Make it possible to remove cookie from '/api/v1/logout'
    #[get("/logout")]
    pub fn logout( mut cookies: Cookies ) -> PostResponse {
        Session::delete(&mut cookies);
        Ok(Redirect::to("/"))
    }

    #[post("/register?<redirect>", data = "<form>")]
    pub fn register( conn:        Conn,
                        redirect:    Option<String>, 
                        form:        Form<RegisterForm>) -> PostResponse {
        let path = redirect.unwrap_or("/login".to_string());
        if form.password1 == form.password2 {
            match User::create_from(&conn,&form.username,&form.password1) {
                Ok(_) => Ok(Redirect::to(path)), 
                _ => flash!("/register", "Username is taken")
            }
        }
        else {
            flash!("/register", "Passwords don't match")
        }
    }
}