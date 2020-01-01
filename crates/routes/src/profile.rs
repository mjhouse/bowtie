
// @todo Set up database connection pooling
// @body https://api.rocket.rs/v0.5/rocket_contrib/databases/index.html

pub mod pages {
    
    use rocket::{
        State,
        response::{Redirect},
    };

    use tera::{Context};

    use crate::resources::*;
    use bowtie_data::Conn;
    use bowtie_models::{
        friend::Friend,
        message::Message,
        session::Session,
        post::Post
    };

    /// The base page for a user's profile. Redirects
    /// to `/profile/feed`
    #[get("/profile")]
    pub fn main() -> Redirect {
        Redirect::to("/profile/feed")
    }
    
    /// Display posts for the current view and recent posts from
    /// subscribed views.
    #[get("/profile/feed")]
    pub fn feed( conn: Conn, resources: State<Resources>, session: Session ) -> Page {
        let posts    = Post::for_view(&conn,session.view);
        let followed = Post::for_followed(&conn,session.view);
        let friends  = Post::for_friends(&conn,session.view);
        Page::render(&resources,"/profile/feed",true)
            .with_context(context!(
                "posts"    => posts,
                "followed" => followed,
                "friends"  => friends ))
    }
    
    /// Display friends of the current view.
    #[get("/profile/friends")]
    pub fn friends( conn: Conn, resources: State<Resources>, session: Session ) -> Page {
        let friend_requests = Friend::requests(&conn,session.view); 
        Page::render(&resources,"/profile/friends",true)
            .with_context(context!(
                "friend_requests" => friend_requests))
    }
    
    /// Sent and received messages for the current view.
    #[get("/profile/messages")]
    pub fn messages( conn: Conn, resources: State<Resources>, session: Session ) -> Page {
        let received = Message::received(&conn,session.view);
        let friends  = Friend::friends(&conn,session.view);
        Page::render(&resources,"/profile/messages",true)
            .with_context(context!(
                "received" => received,
                "friends"  => friends))
    }
    
    /// Write a new post for the current view.
    #[get("/profile/write")]
    pub fn write( resources: State<Resources> ) -> Page {
        Page::render(&resources,"/profile/write",true)
    }
    
    /// Change settings for the current view or switch
    /// to a different view.
    #[get("/profile/settings")]
    pub fn settings( resources: State<Resources> ) -> Page {
        Page::render(&resources,"/profile/settings",true)
    }

}

pub mod api {

    use rocket::{
        response::{Flash,Redirect},
        request::{Form},
        http::{Cookies}
    };

    use bowtie_data::Conn;

    type ApiResponse = Result<Redirect,Flash<Redirect>>;

    macro_rules! unpack {
        ( $p:ident, $c:ident ) => {
            match Session::get(&$c) {
                Ok(s) => {
                    match (s.id,s.view) {
                        (Some(u),v) => (u,v),
                        _ => {
                            warn!("User id was None during unpack");
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

    pub mod follows {
        use super::*;
        
        use bowtie_models::{
            session::{Session},
            follow::{Follow}
        }; 

        #[derive(FromForm,Debug)]
        pub struct CreateFollow {
            pub publisher:  i32
        }

        #[derive(FromForm,Debug)]
        pub struct DeleteFollow {
            pub publisher:  i32
        }

        #[post("/api/v1/follow/create?<redirect>", data = "<form>")]
        pub fn create( conn:     Conn,
                       redirect: String,
                       cookies:  Cookies, 
                       form:     Form<CreateFollow>) -> ApiResponse {
            let (_,vid) = unpack!(redirect,cookies);

            match Follow::create(&conn,vid,form.publisher) {
                Ok(_) => Ok(Redirect::to(redirect)),
                _ => flash!(redirect,"Could not create following relationship")
            }
        }

        #[post("/api/v1/follow/delete?<redirect>", data = "<form>")]
        pub fn delete( conn:     Conn,
                       redirect: String,
                       cookies:  Cookies, 
                       form:     Form<DeleteFollow>) -> ApiResponse {
            let (_,vid) = unpack!(redirect,cookies);

            match Follow::delete(&conn,vid,form.publisher) {
                Ok(_) => Ok(Redirect::to(redirect)),
                _ => flash!(redirect,"Could not create following relationship")
            }
        }

    }

    pub mod comments {
        use super::*;
        
        use bowtie_models::{
            session::{Session},
            comment::{Comment}
        }; 

        #[derive(FromForm,Debug)]
        pub struct CreateComment {
            pub post:   i32,
            pub parent: Option<i32>,
            pub body:   String
        }

        #[post("/api/v1/comment/create?<redirect>", data = "<form>")]
        pub fn create( conn:     Conn,
                       redirect: String,
                       cookies:  Cookies, 
                       form:     Form<CreateComment>) -> ApiResponse {
            let (_,vid) = unpack!(redirect,cookies);

            match Comment::create(&conn,vid,form.post,form.parent,form.body.clone()) {
                Ok(_) => Ok(Redirect::to(redirect)),
                _ => flash!(redirect,"Could not create comment")
            }
        }

        #[get("/api/v1/comment/delete?<redirect>&<id>")]
        pub fn delete( conn:     Conn,
                       redirect: String,
                       cookies:  Cookies, 
                       id:       i32) -> ApiResponse {
            let (_,vid) = unpack!(redirect,cookies);

            match Comment::delete(&conn,vid,id) {
                Ok(_) => Ok(Redirect::to(redirect)),
                _ => flash!(redirect,"Could not delete comment")
            }
        }

    }

    /*  Messages API
        This module contains endpoints that handle the
        creation, deletion and modification of messages.
    */
    pub mod messages {
        use super::*;
        
        use bowtie_models::{
            session::{Session},
            message::{Message}
        }; 

        #[derive(FromForm,Debug)]
        pub struct CreateMessage {
            pub receiver: i32,
            pub body:     String
        }

        #[post("/api/v1/message/create?<redirect>", data = "<form>")]
        pub fn create( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<CreateMessage>) -> ApiResponse {
            let path = redirect.unwrap_or("/profile/messages".to_string());
            let (_,vid) = unpack!(path,cookies);

            match Message::create_from(&conn,vid,form.receiver,form.body.clone()) {
                Ok(_) => Ok(Redirect::to(path)),
                _ => flash!(path,"Could not send message")
            }
        }

    }

    /*  Requests API
        This module contains endpoints that handle the
        creation, deletion and modification of friend
        requests.
    */
    pub mod requests {
        use super::*;
        
        use bowtie_models::{
            session::{Session},
            friend::{Friend}
        }; 

        #[derive(FromForm,Debug)]
        pub struct CreateRequest {
            pub value: i32,
        }

        #[derive(FromForm,Debug)]
        pub struct DeleteRequest {
            pub value: i32,
        }

        #[derive(FromForm,Debug)]
        pub struct UpdateRequest {
            pub value:    i32,
            pub accepted: bool
        }

        #[post("/api/v1/friend/create?<redirect>", data = "<form>")]
        pub fn create( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<CreateRequest>) -> ApiResponse {
            let path = redirect.unwrap_or("/profile/friends".to_string());
            let (_,vid) = unpack!(path,cookies);

            if vid == form.value { 
                return flash!(path,"That's just weird. Weirdo.") }

            match Friend::create_from(&conn,vid,form.value,false) {
                Ok(_) => Ok(Redirect::to(path)),
                _ => flash!(path,"Could not create friend request")
            }
        }

        #[post("/api/v1/friend/update?<redirect>", data = "<form>")]
        pub fn update( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<UpdateRequest>) -> ApiResponse {
            let path = redirect.unwrap_or("/profile/friends".to_string());
            let (_,vid) = unpack!(path,cookies);
            if form.accepted {
                match Friend::accept(&conn,form.value,vid) {
                    Ok(_)  => Ok(Redirect::to(path)),
                    Err(e) => {
                        dbg!(e);
                        flash!(path,"Could not accept friend request")}
                }
            }
            else {
                match Friend::delete_from(&conn,vid,form.value) {
                    Ok(_) => Ok(Redirect::to(path)),
                    _ => flash!(path,"Could not deny friend request")
                }
            }
        }

        #[post("/api/v1/friend/delete?<redirect>", data = "<form>")]
        pub fn delete( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<DeleteRequest>) -> ApiResponse {
            let path = redirect.unwrap_or("/profile/friends".to_string());
            let (_,vid) = unpack!(path,cookies);

            match Friend::delete_from(&conn,vid,form.value) {
                Ok(_) => Ok(Redirect::to(path)),
                _ => flash!(path,"Could not delete friend")
            }
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
        pub fn create( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<CreatePost>) -> ApiResponse {
            let path = redirect.unwrap_or("/write".to_string());
            let (_,vid) = unpack!(path,cookies);

            match Post::create_from(&conn,vid,&form.title,&form.body) {
                Ok(_) => Ok(Redirect::to(path)),
                _ => flash!(path,"Could not create post")
            }
        }

        #[post("/api/v1/posts/delete?<redirect>", data = "<form>")]
        pub fn delete( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<DeletePost>) -> ApiResponse {
            let path = redirect.unwrap_or("/feed".to_string());
            let (_,vid) = unpack!(path,cookies);

            match Post::delete_from(&conn,vid,form.value) {
                Ok(_)  => Ok(Redirect::to(path)),
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
        pub fn create( conn: Conn,
                       redirect: Option<String>,
                       mut cookies: Cookies, 
                       form: Form<CreateView>) -> ApiResponse {
            let path = redirect.unwrap_or("/profile".to_string());
            let (uid,_) = unpack!(path,cookies);

            match View::create_from(&conn,uid,&form.value) {
                Ok(v) if v.id.is_some() => {
                    match Session::add_view(v.id.unwrap(),v.name,&mut cookies) {
                        Ok(_) => Ok(Redirect::to(path)),
                        _ => flash!(path,"Could not update session")  
                    }
                },
                Err(e) => {
                    dbg!(e);
                    flash!(path,"Could not create view")
                }
                _ => flash!(path,"Could not create view")
            }
        }

        #[post("/api/v1/views/update?<redirect>", data = "<form>")]
        pub fn update( redirect:    Option<String>, 
                       mut cookies: Cookies, 
                       form:        Form<UpdateView>) -> ApiResponse {
            let path = redirect.unwrap_or("/profile".to_string());
            match Session::set_view(form.value,&mut cookies) {
                Ok(_) => Ok(Redirect::to(path)),
                Err(e) => {
                    dbg!(e);
                    flash!(path,"Could not update view")
                }
            }
        }

        #[post("/api/v1/views/delete?<redirect>", data = "<form>")]
        pub fn delete( conn: Conn,
                       redirect: Option<String>, 
                       mut cookies: Cookies, 
                       form: Form<DeleteView>) -> ApiResponse {
            let path = redirect.unwrap_or("/profile".to_string());
            let (uid,cid) = unpack!(path,cookies);

            if form.value == cid {
                return flash!(path,"Cannot delete current view")
            }

            match View::delete_from(&conn,uid,form.value) {
                Ok(v) if v.id.is_some() => {
                    match Session::remove_view(v.id.unwrap(),&mut cookies) {
                        Ok(_) => Ok(Redirect::to(path)),
                        _ => flash!(path,"Could not update session")  
                    }
                },
                Err(e) => {
                    dbg!(e);
                    flash!(path,"Could not delete view")
                }
                _ => flash!(path,"Could not delete view")
            }
        }

    }

}


