pub mod get {

    use rocket::{
        State,
        request::{LenientForm}
    };
    
    use tera::{Context};
    
    // @todo Make bowtie_models export internal modules
    // @body These imports are getting out of hand
    
    use crate::resources::*;
    use bowtie_models::view::*;
    use bowtie_models::post::*;
    use bowtie_models::search::*;
    use bowtie_models::friend::*;
    use bowtie_models::comment::*;
    use bowtie_models::follow::*;
    use bowtie_models::session::*;
    
    use bowtie_data::Conn;

    #[get("/")]
    pub fn index( resources: State<Resources> ) -> Page {
        Page::render(&resources,"/public/index",false)
    }
    
    #[get("/about")]
    pub fn about( resources: State<Resources> ) -> Page {
        Page::render(&resources,"/public/about",false)
    }
    
    #[get("/search?<query..>")]
    pub fn search( conn: Conn, resources: State<Resources>, query: LenientForm<SearchQuery> ) -> Page {
        Page::render(&resources,"/public/search",false)
            .with_context(context!(
                "search" => Search::from(&conn,&query)))
    }
    
    #[get("/user/<name>")]
    pub fn user( conn: Conn, session: Option<Session>, resources: State<Resources>, name: String ) -> Page {
        let (posts,view) = match View::for_name(&conn,&name) {
            Some(v) => (v.posts(&conn),Some(v)),
            None    => (vec![],None)
        };
    
        // @todo Refactor to reduce the number of queries
        // @body There are 4 queries in the worst case at this endpoint
    
        let followed = match (session.as_ref(),view.as_ref()) {
            (Some(ref s),Some(ref v)) => {
                if let Some(id) = v.id {
                    Follow::exists(&conn,s.view,id)
                }
                else {
                    false
                }
            },
            (_,_) => false
        };
    
        let friended = match (session,view.as_ref()) {
            (Some(ref s),Some(ref v)) => {
                if let Some(id) = v.id {
                    Friend::exists(&conn,s.view,id)
                }
                else {
                    false
                }
            },
            (_,_) => false
        };
    
        Page::render(&resources,"/public/user",false)
            .with_context(context!(
                "posts"    => posts,
                "view"     => view,
                "followed" => followed,
                "friended" => friended))
    }
    
    #[get("/post/<id>")]
    pub fn post( conn: Conn, resources: State<Resources>, id: i32 ) -> Page {
        let comments   = Comment::for_post(&conn,id);
        let submission = Post::for_id(&conn,id).ok();
    
        Page::render(&resources,"/public/post",false)
            .with_context(context!(
                "submission" => submission,
                "comments"   => comments
            ))
    }
    
    #[get("/comment/<id>")]
    pub fn comment( conn: Conn, resources: State<Resources>, id: i32 ) -> Page {
        let comments   = Comment::for_comment(&conn,id);
        let submission = Comment::for_id(&conn,id).ok();
    
        let crumbs = match submission {
            Some((_,ref c)) => c.get_path(),
            _ => vec![]
        };
    
        Page::render(&resources,"/public/comment",false)
            .with_context(context!(
                "submission" => submission,
                "comments"   => comments,
                "crumbs"     => crumbs
            ))
    }
}
