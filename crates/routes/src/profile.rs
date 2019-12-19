
// @todo Set up database connection pooling
// @body https://api.rocket.rs/v0.5/rocket_contrib/databases/index.html

pub mod pages {

    use rocket_contrib::{
        templates::Template
    };
    
    use rocket::{
        request::{FlashMessage},
        response::{Redirect},
    };
    
    use diesel::prelude::*;
    use std::env;
    
    use bowtie_models::*;

    #[get("/profile")]
    pub fn main( _session: Session ) -> Redirect {
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
    
    #[get("/profile/friends")]
    pub fn friends( session: Session, msg: Option<FlashMessage> ) -> Template {
        let friends = match session.view {
            Some(id) => Friend::friends(id),
            _ => vec![]
        };
    
        Template::render("profile/friends",Context {
            session: Some(session),
            views:   friends,
            flash:   unflash!(msg),
            ..Default::default()
        })
    }
    
    #[get("/profile/messages")]
    pub fn messages( session: Session, msg: Option<FlashMessage> ) -> Template {
        let messages = match session.view {
            Some(id) => Message::messages(id),
            _ => vec![]
        };
    
        Template::render("profile/messages",Context {
            session: Some(session),
            messages: messages,
            flash:    unflash!(msg),
            ..Default::default()
        })
    }
    
    #[get("/profile/write")]
    pub fn write( session: Session, msg: Option<FlashMessage>  ) -> Template {
        Template::render("profile/write",Context {
            session: Some(session),
            flash:   unflash!(msg),
            ..Default::default()
        })
    }
    
    #[get("/profile/settings")]
    pub fn settings( session: Session, msg: Option<FlashMessage>  ) -> Template {
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

}

pub mod api {

    use rocket::{
        response::{Flash,Redirect},
        request::{Form},
        http::{Cookies}
    };

    type ApiResponse = Result<Redirect,Flash<Redirect>>;

    macro_rules! unpack {
        ( $p:ident, $c:ident ) => {
            match Session::get(&$c) {
                Ok(s) => {
                    match (s.id,s.view) {
                        (Some(u),Some(v)) => (u,v),
                        _ => {
                            warn!("User or View id was None during unpack");
                            return flash!($p,"User not found")
                        }
                    }
                },
                Err(e) => {
                    warn!("Error getting Session from Cookies: {}",e);
                    return flash!($p,"Not logged in")
                }
            };
        }
    }

    /*  Posts API
        This module contains endpoints that handle the
        creation, deletion and modification of posts.
    */
    pub mod posts {

        use super::*;
        
        use bowtie_models::{
            session::{Session},
            post::{Post}
        }; 

        #[derive(FromForm)]
        pub struct CreatePost {
            pub title: String,
            pub body:  String,
        }

        #[derive(FromForm)]
        pub struct DeletePost {
            pub value: i32,
        }

        #[post("/api/v1/posts/create?<redirect>", data = "<form>")]
        pub fn create( redirect: Option<String>,
                       cookies:  Cookies, 
                       form:     Form<CreatePost>) -> ApiResponse {
            let path = redirect.unwrap_or("/write".to_string());
            let (_,vid) = unpack!(path,cookies);

            match Post::create_from(vid,&form.title,&form.body) {
                Ok(_) => Ok(Redirect::to(path)),
                _ => flash!(path,"Could not create post")
            }
        }

        #[post("/api/v1/posts/delete?<redirect>", data = "<form>")]
        pub fn delete( redirect: Option<String>,
                       cookies:  Cookies, 
                       form:     Form<DeletePost>) -> ApiResponse {
            let path = redirect.unwrap_or("/feed".to_string());
            let (_,vid) = unpack!(path,cookies);

            match Post::delete_from(vid,form.value) {
                Ok(_) => Ok(Redirect::to(path)),
                _ => flash!(path,"Could not delete post")
            }
        }

    }

    /*  Views API
        This module contains endpoints that handle the
        creation, deletion and modification of views.
    */
    pub mod views {
        
        use super::*;
        
        use bowtie_models::{
            session::{Session},
            view::{View}
        }; 

        #[derive(FromForm,Debug)]
        pub struct CreateView {
            pub value:  String
        }

        #[derive(FromForm,Debug)]
        pub struct UpdateView {
            pub value:  i32
        }

        #[derive(FromForm,Debug)]
        pub struct DeleteView {
            pub value:  i32
        }

        #[post("/api/v1/views/create?<redirect>", data = "<form>")]
        pub fn create( redirect: Option<String>,
                       cookies:  Cookies, 
                       form:     Form<CreateView>) -> ApiResponse {
            let path = redirect.unwrap_or("/profile".to_string());
            let (uid,_) = unpack!(path,cookies);

            match View::create_from(uid,&form.value) {
                Ok(_) => Ok(Redirect::to(path)),
                _ => flash!(path,"Could not create view")
            }
        }

        #[post("/api/v1/views/update?<redirect>", data = "<form>")]
        pub fn update( redirect:    Option<String>, 
                       mut cookies: Cookies, 
                       form:        Form<UpdateView>) -> ApiResponse {
            let path = redirect.unwrap_or("/profile".to_string());
            let (uid,_) = unpack!(path,cookies);
            
            match Session::update(uid,form.value,&mut cookies) {
                Ok(_) => Ok(Redirect::to(path)),
                _ => flash!(path,"Could not update view")
            }
        }

        #[post("/api/v1/views/delete?<redirect>", data = "<form>")]
        pub fn delete( redirect: Option<String>, 
                       cookies:  Cookies, 
                       form:     Form<DeleteView>) -> ApiResponse {
            let path = redirect.unwrap_or("/profile".to_string());
            let (uid,cid) = unpack!(path,cookies);

            if form.value == cid {
                return flash!(path,"Cannot delete current view")
            }

            match View::delete_from(uid,form.value) {
                Ok(_) => Ok(Redirect::to(path)),
                _ => flash!(path,"Could not delete view")
            }
        }

    }

}


