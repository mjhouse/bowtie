pub mod get {
    
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

pub mod post {

    use rocket::{
        response::{Flash,Redirect},
        request::{Form},
        http::{Cookies}
    };

    use bowtie_models::*; 
    use bowtie_data::Conn;

    type PostResponse = Result<Redirect,Flash<Redirect>>;

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

    /*  Follows
        This module contains endpoints that handle the
        creation and deletion of follower relationships.
    */
    pub mod follow {
        use super::*;

        #[derive(FromForm,Debug)]
        pub struct FollowForm {
            pub publisher:  i32
        }

        #[post("/follow/create?<redirect>", data = "<form>")]
        pub fn create( conn:     Conn,
                       redirect: String,
                       cookies:  Cookies, 
                       form:     Form<FollowForm>) -> PostResponse {
            let (_,vid) = unpack!(redirect,cookies);

            match Follow::create(&conn,vid,form.publisher) {
                Ok(_) => Ok(Redirect::to(redirect)),
                Err(e) => {
                    warn!("Could not create follow: {}",e);
                    flash!(redirect,"Could not create following relationship")
                }
            }
        }

        #[post("/follow/delete?<redirect>", data = "<form>")]
        pub fn delete( conn:     Conn,
                       redirect: String,
                       cookies:  Cookies, 
                       form:     Form<FollowForm>) -> PostResponse {
            let (_,vid) = unpack!(redirect,cookies);

            match Follow::delete(&conn,vid,form.publisher) {
                Ok(_) => Ok(Redirect::to(redirect)),
                Err(e) => {
                    warn!("Could not delete follow: {}",e);
                    flash!(redirect,"Could not delete following relationship")
                }
            }
        }

    }

    /*  Comments
        This module contains endpoints that handle the
        creation, deletion and modification of comments.
    */
    pub mod comment {
        use super::*;

        #[derive(FromForm,Debug)]
        pub struct CreateComment {
            pub post:   i32,
            pub parent: Option<i32>,
            pub body:   String
        }

        #[post("/comment/create?<redirect>", data = "<form>")]
        pub fn create( conn:     Conn,
                       redirect: String,
                       cookies:  Cookies, 
                       form:     Form<CreateComment>) -> PostResponse {
            let (_,vid) = unpack!(redirect,cookies);

            match Comment::create(&conn,vid,form.post,form.parent,form.body.clone()) {
                Ok(_) => Ok(Redirect::to(redirect)),
                Err(e) => {
                    warn!("Could not create comment: {}",e);
                    flash!(redirect,"Could not create comment")
                }
            }
        }

        #[get("/comment/delete?<redirect>&<id>")]
        pub fn delete( conn:     Conn,
                       redirect: String,
                       cookies:  Cookies, 
                       id:       i32) -> PostResponse {
            let (_,vid) = unpack!(redirect,cookies);

            match Comment::delete(&conn,vid,id) {
                Ok(_) => Ok(Redirect::to(redirect)),
                Err(e) => {
                    warn!("Could not delete comment: {}",e);
                    flash!(redirect,"Could not delete comment")
                }
            }
        }

    }

    /*  Messages
        This module contains endpoints that handle the
        creation, deletion and modification of messages.
    */
    pub mod message {
        use super::*;

        #[derive(FromForm,Debug)]
        pub struct CreateMessage {
            pub receiver: i32,
            pub body:     String
        }

        #[post("/message/create?<redirect>", data = "<form>")]
        pub fn create( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<CreateMessage>) -> PostResponse {
            let path = redirect.unwrap_or("/profile/messages".to_string());
            let (_,vid) = unpack!(path,cookies);

            match Message::create_from(&conn,vid,form.receiver,form.body.clone()) {
                Ok(_) => Ok(Redirect::to(path)),
                Err(e) => {
                    warn!("Could not send message: {}",e);
                    flash!(path,"Could not send message")
                }
            }
        }

    }

    /*  Friends
        This module contains endpoints that handle the
        creation, deletion and modification of friend
        requests.
    */
    pub mod friend {
        use super::*;

        #[derive(FromForm,Debug)]
        pub struct FriendForm {
            pub value: i32,
        }

        #[derive(FromForm,Debug)]
        pub struct UpdateRequest {
            pub value:    i32,
            pub accepted: bool
        }

        #[post("/friend/create?<redirect>", data = "<form>")]
        pub fn create( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<FriendForm>) -> PostResponse {
            let path = redirect.unwrap_or("/profile/friends".to_string());
            let (_,vid) = unpack!(path,cookies);

            if vid == form.value { 
                return flash!(path,"That's just weird. Weirdo.") }

            match Friend::create_from(&conn,vid,form.value,false) {
                Ok(_) => Ok(Redirect::to(path)),
                Err(e) => {
                    warn!("Could not create friend request: {}",e);
                    flash!(path,"Could not create friend request")
                }
            }
        }

        #[post("/friend/update?<redirect>", data = "<form>")]
        pub fn update( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<UpdateRequest>) -> PostResponse {
            let path = redirect.unwrap_or("/profile/friends".to_string());
            let (_,vid) = unpack!(path,cookies);
            if form.accepted {
                match Friend::accept(&conn,form.value,vid) {
                    Ok(_)  => Ok(Redirect::to(path)),
                    Err(e) => {
                        warn!("Could not accept friend request: {}",e);
                        flash!(path,"Could not accept friend request")}
                }
            }
            else {
                match Friend::delete_from(&conn,vid,form.value) {
                    Ok(_) => Ok(Redirect::to(path)),
                    Err(e) => {
                        warn!("Could not deny friend request: {}",e);
                        flash!(path,"Could not deny friend request")
                    }
                }
            }
        }

        #[post("/friend/delete?<redirect>", data = "<form>")]
        pub fn delete( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<FriendForm>) -> PostResponse {
            let path = redirect.unwrap_or("/profile/friends".to_string());
            let (_,vid) = unpack!(path,cookies);

            match Friend::delete_from(&conn,vid,form.value) {
                Ok(_) => Ok(Redirect::to(path)),
                Err(e) => {
                    warn!("Could not delete friend: {}",e);
                    flash!(path,"Could not delete friend")
                }
            }
        }

    }

    /*  Posts
        This module contains endpoints that handle the
        creation, deletion and modification of posts.
    */
    pub mod post {
        use super::*;

        #[derive(FromForm)]
        pub struct CreatePost {
            pub title: String,
            pub body:  String,
        }

        #[derive(FromForm)]
        pub struct DeletePost {
            pub value: i32,
        }

        #[post("/posts/create?<redirect>", data = "<form>")]
        pub fn create( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<CreatePost>) -> PostResponse {
            let path = redirect.unwrap_or("/write".to_string());
            let (_,vid) = unpack!(path,cookies);

            match Post::create_from(&conn,vid,&form.title,&form.body) {
                Ok(_) => Ok(Redirect::to(path)),
                Err(e) => {
                    warn!("Could not create post: {}",e);
                    flash!(path,"Could not create post")
                }
            }
        }

        #[post("/posts/delete?<redirect>", data = "<form>")]
        pub fn delete( conn: Conn,
                       redirect: Option<String>,
                       cookies: Cookies, 
                       form: Form<DeletePost>) -> PostResponse {
            let path = redirect.unwrap_or("/feed".to_string());
            let (_,vid) = unpack!(path,cookies);

            match Post::delete_from(&conn,vid,form.value) {
                Ok(_)  => Ok(Redirect::to(path)),
                Err(e) => {
                    warn!("Could not delete post: {}",e);
                    flash!(path,"Could not delete post")
                }
            }
        }

    }

    /*  Views
        This module contains endpoints that handle the
        creation, deletion and modification of views.
    */
    pub mod view {
        
        use super::*;

        #[derive(FromForm,Debug)]
        pub struct CreateView {
            pub value:  String
        }

        #[derive(FromForm,Debug)]
        pub struct ViewForm {
            pub value:  i32
        }

        #[post("/views/create?<redirect>", data = "<form>")]
        pub fn create( conn: Conn,
                       redirect: Option<String>,
                       mut cookies: Cookies, 
                       form: Form<CreateView>) -> PostResponse {
            let path = redirect.unwrap_or("/profile".to_string());
            let (uid,_) = unpack!(path,cookies);

            match View::create_from(&conn,uid,&form.value) {
                Ok(v) => {
                    match Session::add_view(v.id,v.name,&mut cookies) {
                        Ok(_) => Ok(Redirect::to(path)),
                        _ => flash!(path,"Could not update session")  
                    }
                },
                Err(e) => {
                    warn!("Could not create view: {}",e);
                    flash!(path,"Could not create view")
                }
            }
        }

        #[post("/views/update?<redirect>", data = "<form>")]
        pub fn update( redirect:    Option<String>, 
                       mut cookies: Cookies, 
                       form:        Form<ViewForm>) -> PostResponse {
            let path = redirect.unwrap_or("/profile".to_string());
            match Session::set_view(form.value,&mut cookies) {
                Ok(_) => Ok(Redirect::to(path)),
                Err(e) => {
                    warn!("Could not update view: {}",e);
                    flash!(path,"Could not update view")
                }
            }
        }

        #[post("/views/delete?<redirect>", data = "<form>")]
        pub fn delete( conn: Conn,
                       redirect: Option<String>, 
                       mut cookies: Cookies, 
                       form: Form<ViewForm>) -> PostResponse {
            let path = redirect.unwrap_or("/profile".to_string());
            let (uid,cid) = unpack!(path,cookies);

            if form.value == cid {
                return flash!(path,"Cannot delete current view")
            }

            match View::delete_from(&conn,uid,form.value) {
                Ok(v) => {
                    match Session::remove_view(v.id,&mut cookies) {
                        Ok(_) => Ok(Redirect::to(path)),
                        Err(e) => {
                            warn!("Could not update session: {}",e);
                            flash!(path,"Could not update session")  
                        }
                    }
                },
                Err(e) => {
                    warn!("Could not delete view: {}",e);
                    flash!(path,"Could not delete view")
                }
            }
        }

    }

}


